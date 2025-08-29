#[cfg(test)]
mod tests {
	use crate::WindowRelativeProfile;
	use task_syncer::Task;
	use std::sync::Mutex;
	


	#[test]
	fn test_profile_full_test() {
		static HISTORY:Mutex<Vec<String>> = Mutex::new(Vec::new());

		let mut profile:WindowRelativeProfile = WindowRelativeProfile::new("test_id", "test_title", "test_process_name.exe");
		assert_eq!(profile.id(), "test_id");
		assert_eq!(profile.title(), "test_title");
		assert_eq!(profile.process_name(), "test_process_name.exe");
		assert_eq!(profile.is_default_profile(), false);
		assert_eq!(profile.is_active("active_process_name", "active_process_title"), false);
		assert_eq!(profile.is_active("test_process_name.exe", "active_process_title"), true);
		profile.trigger_activate_event().unwrap();
		assert!(HISTORY.lock().unwrap().is_empty());
		profile.trigger_deactivate_event().unwrap();
		assert!(HISTORY.lock().unwrap().is_empty());
		profile.trigger_open_event().unwrap();
		assert!(HISTORY.lock().unwrap().is_empty());

		// Create a new profile as the previous profile has already triggered the 'open' event on the first 'activation' event.
		let mut profile:WindowRelativeProfile = WindowRelativeProfile::new("test_id", "test_title", "test_process_name.exe")
								.with_is_default_profile()
								.with_active_checker(|_, active_process_name, _| active_process_name == "second_test_process_name.exe")
								.with_open_handler(|_| { HISTORY.lock().unwrap().push("open".to_string()); Ok(()) })
								.with_activate_handler(|_| { HISTORY.lock().unwrap().push("activate".to_string()); Ok(()) })
								.with_deactivate_handler(|_| { HISTORY.lock().unwrap().push("deactivate".to_string()); Ok(()) })
								.with_task(Task::new("test_task", |_| { HISTORY.lock().unwrap().push("handled task".to_string()); Ok(()) }))
								.with_named_operation("test_operation_name", || { HISTORY.lock().unwrap().push("handled named operation".to_string()); Ok(()) });
		assert_eq!(profile.id(), "test_id");
		assert_eq!(profile.title(), "test_title");
		assert_eq!(profile.process_name(), "test_process_name.exe");
		assert_eq!(profile.is_default_profile(), true);
		assert_eq!(profile.is_active("active_process_name", "active_process_title"), false);
		assert_eq!(profile.is_active("test_process_name.exe", "active_process_title"), false);
		assert_eq!(profile.is_active("second_test_process_name.exe", "active_process_title"), true);
		profile.trigger_activate_event().unwrap();
		assert_eq!(HISTORY.lock().unwrap().remove(0), "open");
		assert_eq!(HISTORY.lock().unwrap().remove(0), "activate");
		profile.trigger_deactivate_event().unwrap();
		assert_eq!(HISTORY.lock().unwrap().remove(0), "deactivate");
		profile.trigger_open_event().unwrap();
		assert_eq!(HISTORY.lock().unwrap().remove(0), "open");
	}
}