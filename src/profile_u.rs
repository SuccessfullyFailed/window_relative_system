#[cfg(test)]
mod tests {
	use crate::{ TestCore, WindowRelativeProfile, WindowRelativeProfileCore, WindowRelativeProfileProperties, WindowRelativeProfileServiceSet };
	use window_controller::WindowController;
	use task_syncer::{Task, TaskSystem};
	use std::sync::Mutex;
	


	#[test]
	fn test_profile_full_test() {
		static HISTORY:Mutex<Vec<String>> = Mutex::new(Vec::new());

		let fake_window:WindowController = WindowController::from_hwnd(std::ptr::null_mut());

		let mut profile:TestCore = TestCore::default();
		assert_eq!(profile.id(), TestCore::ID);
		assert_eq!(profile.title(), TestCore::TITLE);
		assert_eq!(profile.process_name(), TestCore::PROCESS_NAME);
		assert_eq!(profile.is_default_profile(), false);
		assert_eq!(profile.is_active(&fake_window, "active_process_name", "active_process_title"), false);
		assert_eq!(profile.is_active(&fake_window, TestCore::PROCESS_NAME, "active_process_title"), true);
		profile.trigger_event_with_window("activate", &fake_window).unwrap();
		assert!(HISTORY.lock().unwrap().is_empty());
		profile.trigger_event_with_window("deactivate", &fake_window).unwrap();
		assert!(HISTORY.lock().unwrap().is_empty());
		profile.trigger_event_with_window("open", &fake_window).unwrap();
		assert!(HISTORY.lock().unwrap().is_empty());

		// Create a new profile as the previous profile has already triggered the 'open' event on the first 'activation' event.
		use crate as window_relative_system;
		#[window_relative_profile_creator_macro::window_relative_profile]
		struct TestCoreB {}
		impl WindowRelativeProfile for TestCoreB {
			fn on_open(&mut self) -> Result<(), Box<dyn std::error::Error>> { HISTORY.lock().unwrap().push("open".to_string()); Ok(()) }
			fn on_activate(&mut self) -> Result<(), Box<dyn std::error::Error>> { HISTORY.lock().unwrap().push("activate".to_string()); Ok(()) }
			fn on_deactivate(&mut self) -> Result<(), Box<dyn std::error::Error>> { HISTORY.lock().unwrap().push("deactivate".to_string()); Ok(()) }
			fn on_close(&mut self) -> Result<(), Box<dyn std::error::Error>> { HISTORY.lock().unwrap().push("close".to_string()); Ok(()) }
		}
		let mut profile:TestCoreB = TestCoreB {
			properties: WindowRelativeProfileProperties::new("test_id", "test_title", "test_process_name.exe")
					.with_is_default()
					.with_active_checker(|_, _, process_name, _| process_name == "second_test_process_name.exe"),
			task_system: TaskSystem::new(),
			services: WindowRelativeProfileServiceSet::new(),
			handlers: Vec::new()
		};
		profile.task_system.add_task(Task::new("test_task", |_, _| { HISTORY.lock().unwrap().push("handled task".to_string()); Ok(()) }));
		profile.add_handler(|profile, _window, event_name| {
			if event_name == "trigger_handler" {
				HISTORY.lock().unwrap().push(format!("handler on profile: {}", profile.properties.id()));
			}
			Ok(())
		});
		
		assert_eq!(profile.id(), "test_id");
		assert_eq!(profile.title(), "test_title");
		assert_eq!(profile.process_name(), "test_process_name.exe");
		assert_eq!(profile.is_default_profile(), true);
		assert_eq!(profile.is_active(&fake_window, "active_process_name", "active_process_title"), false);
		assert_eq!(profile.is_active(&fake_window, "test_process_name.exe", "active_process_title"), false);
		assert_eq!(profile.is_active(&fake_window, "second_test_process_name.exe", "active_process_title"), true);
		profile.trigger_event_with_window("activate", &fake_window).unwrap();
		assert_eq!(HISTORY.lock().unwrap().remove(0), "open");
		assert_eq!(HISTORY.lock().unwrap().remove(0), "handled task");
		assert_eq!(HISTORY.lock().unwrap().remove(0), "activate");
		profile.trigger_event_with_window("deactivate", &fake_window).unwrap();
		assert_eq!(HISTORY.lock().unwrap().remove(0), "deactivate");
		assert_eq!(HISTORY.lock().unwrap().remove(0), "close");
		profile.trigger_event_with_window("open", &fake_window).unwrap();
		assert_eq!(HISTORY.lock().unwrap().remove(0), "open");
		profile.trigger_event_with_window("trigger_handler", &fake_window).unwrap();
		assert_eq!(HISTORY.lock().unwrap().remove(0), "handler on profile: test_id");
		assert!(HISTORY.lock().unwrap().is_empty());
	}
}