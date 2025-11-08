#[cfg(test)]
mod tests {
	use crate::{ WindowRelativeProfile, WindowRelativeProfileService };
	use window_controller::WindowController;
	


	#[test]
	fn test_profile_service_full_test() {
		static mut VALIDATION_VARIABLE:u8 = 0;

		struct TestService {}
		impl WindowRelativeProfileService for TestService {
			fn apply_to_profile_ref(self, profile:&mut WindowRelativeProfile) {
				profile.add_activate_handler(|_, _, _| unsafe {
					VALIDATION_VARIABLE += 1;
					Ok(())
				});
			}
		}

		let mut profile:WindowRelativeProfile = WindowRelativeProfile::new("id", "title", "process_name");
		profile.add_service(TestService {});
		let window:WindowController = WindowController::active();

		assert_eq!(unsafe { VALIDATION_VARIABLE }, 0);
		profile.trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 1);
		profile.trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 2);
	}
}