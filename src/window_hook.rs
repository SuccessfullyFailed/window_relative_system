use winapi::um::winuser::{DispatchMessageW, GetMessageW, SetWinEventHook, TranslateMessage, EVENT_SYSTEM_FOREGROUND, MSG, WINEVENT_OUTOFCONTEXT};
use winapi::shared::{ minwindef::DWORD, ntdef::LONG, windef::{ HWINEVENTHOOK, HWINEVENTHOOK__, HWND } };
use window_controller::WindowController;
use std::{ mem, ptr::null_mut, sync::{ Mutex, MutexGuard }, thread };

use crate::WindowRelativeSystem;



static HOOK_INSTALLED:Mutex<bool> = Mutex::new(false);



/// Install the window hook.
pub fn install(create_thread:bool) {
	if create_thread {
		thread::spawn(|| install(false));
	} else {
		unsafe {
		
			// Validate no existing hook.
			let mut hook_installed:MutexGuard<'_, bool> = HOOK_INSTALLED.lock().unwrap();
			if *hook_installed {
				eprintln!("Hook already in place.");
				return;
			}

			// Create and validate hook.
			let hook:*mut HWINEVENTHOOK__ = SetWinEventHook(EVENT_SYSTEM_FOREGROUND, EVENT_SYSTEM_FOREGROUND, null_mut(), Some(win_event_proc), 0, 0, WINEVENT_OUTOFCONTEXT);
			if hook.is_null() {
				eprintln!("Failed to set event hook.");
				return;
			}
			*hook_installed = true;

			// Keep listening for messages on hook.
			let mut msg:MSG = mem::zeroed();
			while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
				TranslateMessage(&msg);
				DispatchMessageW(&msg);
			}
		}
	}
}

/// Handle a windows hook event to process changes in active window.
unsafe extern "system" fn win_event_proc(_event_hook:HWINEVENTHOOK, event:DWORD, hwnd:HWND, _id_object:LONG, _id_child:LONG, _dw_event_thread:DWORD, _dwms_event_time:DWORD) {
	if event == EVENT_SYSTEM_FOREGROUND {
		let window_controller:WindowController = WindowController::from_hwnd(hwnd);
		WindowRelativeSystem::update_profile(window_controller);
	}
}