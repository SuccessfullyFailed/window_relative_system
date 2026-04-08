#[cfg(test)]
mod tests {
	use crate::{ WindowRelativeProfile, window_relative_profile };
	use window_controller::WindowController;
	use std::{ error::Error, sync::Mutex };
	use crate as window_relative_system; // Makes the profile creation macro usable from within the crate.
	


	static EVENT_RUN_PROOF:Mutex<usize> = Mutex::new(0);



	window_relative_profile!(TestProfile, "test_profile", "test_process_name.exe");
	impl WindowRelativeProfile for TestProfile {
		fn on_event(&mut self, _window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {
			if event_name == "custom_event_tag" {
				*EVENT_RUN_PROOF.lock().unwrap() = 1;
			}
			Ok(())
		}
	}


	#[test]
	fn can_use_test_profile() {
		let mut profile:TestProfile = TestProfile::default();
		assert_eq!(*EVENT_RUN_PROOF.lock().unwrap(), 0);
		profile.execute_event(&WindowController::active(), "event_name").unwrap();
		assert_eq!(*EVENT_RUN_PROOF.lock().unwrap(), 0);
		profile.execute_event(&WindowController::active(), "custom_event_tag").unwrap();
		assert_eq!(*EVENT_RUN_PROOF.lock().unwrap(), 1);
	}
}