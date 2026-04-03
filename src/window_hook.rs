use winapi::um::winuser::{DispatchMessageW, GetMessageW, SetWinEventHook, TranslateMessage, EVENT_SYSTEM_FOREGROUND, MSG, WINEVENT_OUTOFCONTEXT};
use winapi::shared::{ minwindef::DWORD, ntdef::LONG, windef::{ HWINEVENTHOOK, HWINEVENTHOOK__, HWND } };
use std::{ mem, ptr::null_mut, sync::{ Mutex, MutexGuard }, thread };
use crate::WindowRelativeSystemRemoteControl;
use window_controller::WindowController;
use std::thread::JoinHandle;



static HOOK_HANDLE:Mutex<Option<JoinHandle<()>>> = Mutex::new(None);
static mut PREVIOUS_WINDOW:Option<WindowController> = None;
static REMOTE_CONTROLS:Mutex<Vec<WindowRelativeSystemRemoteControl>> = Mutex::new(Vec::new());



/// Create a signal trigger.
pub(crate) fn register_remote(remote:WindowRelativeSystemRemoteControl) {
	REMOTE_CONTROLS.lock().unwrap().push(remote);
	launch_hook_if_not_exist();
}

/// Create a window-hook event callback.
pub(crate) fn launch_hook_if_not_exist() {
	let mut hook_handle:MutexGuard<'_, Option<JoinHandle<()>>> = HOOK_HANDLE.lock().unwrap();
	if hook_handle.is_none() {
		*hook_handle = Some(thread::spawn(move || unsafe {

			// Create and validate hook.
			let hook:*mut HWINEVENTHOOK__ = SetWinEventHook(EVENT_SYSTEM_FOREGROUND, EVENT_SYSTEM_FOREGROUND, null_mut(), Some(win_event_proc), 0, 0, WINEVENT_OUTOFCONTEXT);
			if hook.is_null() {
				eprintln!("Failed to set event hook.");
				return;
			}

			// Figure out initial profile.
			let current_window:WindowController = WindowController::active();
			PREVIOUS_WINDOW = Some(current_window);

			// Keep listening for messages on hook.
			let mut msg:MSG = mem::zeroed();
			while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
				TranslateMessage(&msg);
				DispatchMessageW(&msg);
			}
		}));
	}
}



/// Handle a windows hook event to process changes in active window.
#[allow(static_mut_refs)]
unsafe extern "system" fn win_event_proc(_event_hook:HWINEVENTHOOK, event:DWORD, hwnd:HWND, _id_object:LONG, _id_child:LONG, _dw_event_thread:DWORD, _dwms_event_time:DWORD) {
	const ALTTAB_PROCESS_NAME:&str = "explorer.exe";
	const ALTTAB_CLASS_NAMES:&[&str] = &["ForegroundStaging", "XamlExplorerHostIslandWindow"];

	unsafe {
		if event == EVENT_SYSTEM_FOREGROUND {

			// Ignore event if the user is alt-tabbing.
			let current_window:WindowController = WindowController::from_hwnd(hwnd);
			let process_name:String = current_window.process_name().unwrap_or_default();
			if process_name == ALTTAB_PROCESS_NAME {
				let class:String = current_window.class();
				if ALTTAB_CLASS_NAMES.contains(&class.as_str()) {
					return;
				}
			}

			// Update profile in window-relative system.
			for remote_control in &*REMOTE_CONTROLS.lock().unwrap() {
				remote_control.handle_window_change(&PREVIOUS_WINDOW, &current_window);
			}
			PREVIOUS_WINDOW = Some(current_window);
		}
	}
}