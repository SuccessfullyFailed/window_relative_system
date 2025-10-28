#[cfg(test)]
mod tests {
	use window_controller::WindowController;
	use crate::WindowRelativeProfile;
	use task_syncer::Task;
	use std::sync::Mutex;
	


	#[test]
	fn test_profile_full_test() {
		static HISTORY:Mutex<Vec<String>> = Mutex::new(Vec::new());

		let fake_window:WindowController = WindowController::from_hwnd(std::ptr::null_mut());

		let mut profile:WindowRelativeProfile = WindowRelativeProfile::new("test_id", "test_title", "test_process_name.exe");
		assert_eq!(profile.id(), "test_id");
		assert_eq!(profile.title(), "test_title");
		assert_eq!(profile.process_name(), "test_process_name.exe");
		assert_eq!(profile.is_default_profile(), false);
		assert_eq!(profile.is_active(&fake_window, "active_process_name", "active_process_title"), false);
		assert_eq!(profile.is_active(&fake_window, "test_process_name.exe", "active_process_title"), true);
		profile.trigger_activate_event(&fake_window).unwrap();
		assert!(HISTORY.lock().unwrap().is_empty());
		profile.trigger_deactivate_event(&fake_window).unwrap();
		assert!(HISTORY.lock().unwrap().is_empty());
		profile.trigger_open_event(&fake_window).unwrap();
		assert!(HISTORY.lock().unwrap().is_empty());

		// Create a new profile as the previous profile has already triggered the 'open' event on the first 'activation' event.
		let mut profile:WindowRelativeProfile = WindowRelativeProfile::new("test_id", "test_title", "test_process_name.exe")
								.with_is_default_profile()
								.with_active_checker(|_, _, active_process_name, _| active_process_name == "second_test_process_name.exe")
								.with_open_handler(|_, _, _| { HISTORY.lock().unwrap().push("open".to_string()); Ok(()) })
								.with_activate_handler(|_, _, _| { HISTORY.lock().unwrap().push("activate".to_string()); Ok(()) })
								.with_deactivate_handler(|_, _, _| { HISTORY.lock().unwrap().push("deactivate".to_string()); Ok(()) })
								.with_close_handler(|_, _, _| { HISTORY.lock().unwrap().push("close".to_string()); Ok(()) })
								.with_task(Task::new("test_task", |_, _| { HISTORY.lock().unwrap().push("handled task".to_string()); Ok(()) }));
		assert_eq!(profile.id(), "test_id");
		assert_eq!(profile.title(), "test_title");
		assert_eq!(profile.process_name(), "test_process_name.exe");
		assert_eq!(profile.is_default_profile(), true);
		assert_eq!(profile.is_active(&fake_window, "active_process_name", "active_process_title"), false);
		assert_eq!(profile.is_active(&fake_window, "test_process_name.exe", "active_process_title"), false);
		assert_eq!(profile.is_active(&fake_window, "second_test_process_name.exe", "active_process_title"), true);
		profile.trigger_activate_event(&fake_window).unwrap();
		assert_eq!(HISTORY.lock().unwrap().remove(0), "open");
		assert_eq!(HISTORY.lock().unwrap().remove(0), "activate");
		profile.trigger_deactivate_event(&fake_window).unwrap();
		assert_eq!(HISTORY.lock().unwrap().remove(0), "handled task");
		assert_eq!(HISTORY.lock().unwrap().remove(0), "deactivate");
		assert_eq!(HISTORY.lock().unwrap().remove(0), "close");
		profile.trigger_open_event(&fake_window).unwrap();
		assert_eq!(HISTORY.lock().unwrap().remove(0), "open");
	}
}