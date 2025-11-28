#[cfg(test)]
mod tests {
	use crate::{ TestCore, WindowRelativeProfile, WindowRelativeProfileSized, WindowRelativeProfileService };
	use window_controller::WindowController;
	use std::{ error::Error, ptr };



	#[test]
	#[allow(static_mut_refs)]
	fn test_profile_service_triggers() {
		static mut HISTORY:Vec<String> = Vec::new();

		struct TestService {}
		impl<T> WindowRelativeProfileService<T> for TestService {
			fn trigger_on_event(&self, event_name:&str) -> bool {
				["open", "activate", "deactivate", "close", "update", "test_event"].contains(&event_name)
			}
			fn run(&self, _profile:&mut T, _window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {
				unsafe { HISTORY.push(event_name.to_string()); }
				Ok(())
			}
		}

		let mut profile:TestCore = TestCore::default();
		profile.add_service(TestService {});
		let window:WindowController = WindowController::from_hwnd(ptr::null_mut());
		
		const EXPECTED_ON_EVENT:&[(&str, &[&str])] = &[
			("activate", &["open", "activate"]),
			("activate", &["activate"]),
			("update", &["update"]),
			("deactivate", &["deactivate", "close"]),
		];
		for (event_name, expected_history) in EXPECTED_ON_EVENT {
			unsafe {
				HISTORY = Vec::new();
				profile.trigger_event_with_window(*event_name, &window).unwrap();
				assert_eq!(&HISTORY, expected_history);
			}
		}
	}

	#[test]
	fn test_profile_service_trigger_once() {
		static mut WAS_TRIGGERED:bool = false;

		struct TestService {}
		impl<T> WindowRelativeProfileService<T> for TestService {
			fn trigger_once(&self) -> bool {
				true
			}
			fn run(&self, _profile:&mut T, _window:&WindowController, _event_name:&str) -> Result<(), Box<dyn Error>> {
				unsafe { WAS_TRIGGERED = true; }
				Ok(())
			}
		}

		let mut profile:TestCore = TestCore::default();
		profile.add_service(TestService {});
		
		assert_eq!(profile.services.len(), 1);
		profile.trigger_event("").unwrap();
		assert_eq!(profile.services.len(), 0);
		assert!(unsafe { WAS_TRIGGERED });
	}

	#[test]
	fn test_profile_service_from_fn() {
		static mut WAS_TRIGGERED:bool = false;

		let mut profile:TestCore = TestCore::default();
		profile.add_service(|_self:&mut TestCore, _window:&WindowController, _event:&str| {
			unsafe { WAS_TRIGGERED = true; }
			Ok(())
		});
		
		profile.trigger_event("").unwrap();
		assert!(unsafe { WAS_TRIGGERED });
	}
}