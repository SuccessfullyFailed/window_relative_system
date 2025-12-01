#[cfg(test)]
mod tests {
	use crate::{ WindowRelativeProfile, WindowRelativeProfileService };
	use task_syncer::TaskScheduler;
use window_controller::WindowController;
	use std::{ error::Error, ptr };



	#[test]
	#[allow(static_mut_refs)]
	fn test_profile_service_triggers() {

		// Create test service.
		static mut HISTORY:Vec<String> = Vec::new();
		struct TestService {}
		impl WindowRelativeProfileService for TestService {
			fn trigger_on_event(&self, event_name:&str) -> bool {
				["open", "activate", "deactivate", "close", "update", "test_event"].contains(&event_name)
			}
			fn run(&mut self, _scheduler:&TaskScheduler, _window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {
				unsafe { HISTORY.push(event_name.to_string()); }
				Ok(())
			}
		}

		// Create profile and get fake window.
		let mut profile:WindowRelativeProfile = WindowRelativeProfile::new("test_id", "test_title", "test_process_name");
		profile.add_service(TestService {});
		let fake_window:WindowController = WindowController::from_hwnd(ptr::null_mut());
		
		// Run services and test results.
		const EXPECTED_ON_EVENT:&[(&str, &[&str])] = &[
			("activate", &["open", "activate"]),
			("activate", &["activate"]),
			("update", &["update"]),
			("deactivate", &["deactivate", "close"]),
		];
		for (event_name, expected_history) in EXPECTED_ON_EVENT {
			unsafe {
				HISTORY = Vec::new();
				profile.trigger_event(&fake_window, *event_name).unwrap();
				assert_eq!(&HISTORY, expected_history);
			}
		}
	}

	#[test]
	fn test_profile_service_trigger_once() {

		// Create test service.
		static mut WAS_TRIGGERED:bool = false;
		struct TestService {}
		impl WindowRelativeProfileService for TestService {
			fn trigger_once(&self) -> bool {
				true
			}
			fn run(&mut self, _scheduler:&TaskScheduler, _window:&WindowController, _event_name:&str) -> Result<(), Box<dyn Error>> {
				unsafe { WAS_TRIGGERED = true; }
				Ok(())
			}
		}

		// Create profile and get fake window.
		let mut profile:WindowRelativeProfile = WindowRelativeProfile::new("test_id", "test_title", "test_process_name");
		profile.add_service(TestService {});
		let fake_window:WindowController = WindowController::from_hwnd(ptr::null_mut());
		
		// Run services and test results.
		assert_eq!(profile.services().len(), 1);
		profile.trigger_event(&fake_window, "").unwrap();
		assert_eq!(profile.services().len(), 0);
		assert!(unsafe { WAS_TRIGGERED });
	}

	#[test]
	fn test_profile_service_from_fn() {

		// Create profile and get fake window.
		let mut profile:WindowRelativeProfile = WindowRelativeProfile::new("test_id", "test_title", "test_process_name");
		static mut WAS_TRIGGERED:bool = false;
		profile.add_service(|_scheduler:&TaskScheduler, _window:&WindowController, _event:&str| {
			unsafe { WAS_TRIGGERED = true; }
			Ok(())
		});
		let fake_window:WindowController = WindowController::from_hwnd(ptr::null_mut());
		
		// Run services and test results.
		profile.trigger_event(&fake_window, "").unwrap();
		assert!(unsafe { WAS_TRIGGERED });
	}
}