#[cfg(test)]
mod tests {
	use task_syncer::TaskScheduler;
use window_controller::WindowController;
	use crate::WindowRelativeProfile;
	use std::sync::Mutex;
	


	#[test]
	fn test_profile_event_triggers() {
		static HISTORY:Mutex<Vec<String>> = Mutex::new(Vec::new());

		let mut profile:WindowRelativeProfile = {
			WindowRelativeProfile::new("test_id", "test_title", "test_process")
				.with_service(|_scheduler:&TaskScheduler, _window:&WindowController, event_name:&str| { HISTORY.lock().unwrap().push(format!("service {}", event_name)); Ok(()) })
				.with_handler(|_profile:&mut WindowRelativeProfile, _window:&WindowController, event_name:&str| { HISTORY.lock().unwrap().push(format!("handler {}", event_name)); Ok(()) })
		};
		let fake_window:WindowController = WindowController::from_hwnd(std::ptr::null_mut());

		profile.trigger_event(&fake_window, "activate").unwrap();
		assert_eq!(HISTORY.lock().unwrap().drain(..).collect::<Vec<String>>(), ["service open", "handler open", "service activate", "handler activate"].map(|s| s.to_string()));
		profile.trigger_event(&fake_window, "update").unwrap();
		assert_eq!(HISTORY.lock().unwrap().drain(..).collect::<Vec<String>>(), ["service update", "handler update"].map(|s| s.to_string()));
		profile.trigger_event(&WindowController::active(), "deactivate").unwrap();
		assert_eq!(HISTORY.lock().unwrap().drain(..).collect::<Vec<String>>(), ["service deactivate", "handler deactivate"].map(|s| s.to_string()));
		profile.trigger_event(&fake_window, "activate").unwrap();
		assert_eq!(HISTORY.lock().unwrap().drain(..).collect::<Vec<String>>(), ["service activate", "handler activate"].map(|s| s.to_string()));
		profile.trigger_event(&fake_window, "deactivate").unwrap();
		assert_eq!(HISTORY.lock().unwrap().drain(..).collect::<Vec<String>>(), ["service deactivate", "handler deactivate", "service close", "handler close"].map(|s| s.to_string()));
	}
}