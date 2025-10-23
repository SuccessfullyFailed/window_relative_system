#[cfg(test)]
mod tests {
	use window_controller::WindowController;

use crate::{ WindowRelativeProfile, WindowRelativeSystem };
	use std::{ sync::Mutex, thread::sleep, time::Duration };


	static TEST_PROFILE_ADDED:Mutex<bool> = Mutex::new(false);
	const TEST_ID:&str = "test_id";


	/* ADDING A PROFILE */

	#[test]
	fn test_system_add_profile() {
		WindowRelativeSystem::add_profile(WindowRelativeProfile::new(TEST_ID, "test_title", "test_process_name.exe"));
		WindowRelativeSystem::execute_on_all_profiles(|profile| {
			if profile.id() == TEST_ID {
				*TEST_PROFILE_ADDED.lock().unwrap() = true;
			}
		});
		assert!(*TEST_PROFILE_ADDED.lock().unwrap());
	}

	/// Wait until the test profile is added.
	fn await_test_profile() {
		const INTERVAL:Duration = Duration::from_millis(1);
		const TRY_ADD_PROFILE_INDEX:usize = 1000;
		const MAX_ATTEMPTS:usize = 2000;

		let mut index:usize = 0;
		while !*TEST_PROFILE_ADDED.lock().unwrap() {
			sleep(INTERVAL);
			index += 1;
			if index == TRY_ADD_PROFILE_INDEX {
				test_system_add_profile();
			}
			if index == MAX_ATTEMPTS {
				panic!("Test profile was not added within reasonable timeframe.");
			}
		}
	}


	/* EXECUTE ON PROFILE TESTS */

	#[test]
	fn test_system_execute_on_current_profile() {
		static VALIDATOR:Mutex<bool> = Mutex::new(false);
		await_test_profile();

		WindowRelativeSystem::execute_on_current_profile(|profile| {
			if profile.is_default_profile() {
				*VALIDATOR.lock().unwrap() = true;
			}
		});

		assert!(*VALIDATOR.lock().unwrap());
	}

	#[test]
	fn test_system_execute_on_profile_by_id() {
		static VALIDATOR:Mutex<bool> = Mutex::new(false);
		await_test_profile();

		WindowRelativeSystem::execute_on_profile_by_id(TEST_ID, |profile| {
			if profile.id() == TEST_ID {
				*VALIDATOR.lock().unwrap() = true;
			}
		});

		assert!(*VALIDATOR.lock().unwrap());
	}

	#[test]
	fn test_system_execute_on_all_profiles() {
		static VALIDATOR_A:Mutex<bool> = Mutex::new(false);
		static VALIDATOR_B:Mutex<bool> = Mutex::new(false);
		await_test_profile();
		WindowRelativeSystem::add_profile(WindowRelativeProfile::new("test_2", "", ""));

		WindowRelativeSystem::execute_on_all_profiles(|profile| {
			if profile.id() == TEST_ID {
				*VALIDATOR_A.lock().unwrap() = true;
			}
			if profile.id() == "test_2" {
				*VALIDATOR_B.lock().unwrap() = true;
			}
		});

		assert!(*VALIDATOR_A.lock().unwrap());
		assert!(*VALIDATOR_B.lock().unwrap());
	}



	/* EXECUTE NAMED OPERATION TESTS */

	#[test]
	fn test_system_execute_named_operation_on_current_profile() {
		static VALIDATOR:Mutex<bool> = Mutex::new(false);

		// Do this test after all other tests have finished.
		await_test_profile();
		sleep(Duration::from_millis(100));
		
		let active_window:WindowController = WindowController::active();
		let mut profile:WindowRelativeProfile = WindowRelativeProfile::new("dummy_current_profile", &active_window.title(), &active_window.process_name().expect("Could not get current window process name"));
		profile.add_named_operation("test_operation_current", |_profile| { *VALIDATOR.lock().unwrap() = true; Ok(()) });
		WindowRelativeSystem::add_profile(profile);

		WindowRelativeSystem::update_profile(active_window);
		WindowRelativeSystem::execute_named_operation_on_current_profile("test_operation_current");

		assert!(*VALIDATOR.lock().unwrap());
	}

	#[test]
	fn test_system_execute_named_operation_on_profile_by_id() {
		static VALIDATOR:Mutex<bool> = Mutex::new(false);

		let mut profile:WindowRelativeProfile = WindowRelativeProfile::new("test_3", "", "");
		profile.add_named_operation("test_operation_by_id", |_profile| { *VALIDATOR.lock().unwrap() = true; Ok(()) });
		WindowRelativeSystem::add_profile(profile);

		WindowRelativeSystem::execute_named_operation_on_profile_by_id("test_3", "test_operation_by_id");

		assert!(*VALIDATOR.lock().unwrap());
	}

	#[test]
	fn test_system_execute_named_operation_on_all_profiles() {
		static VALIDATOR_A:Mutex<bool> = Mutex::new(false);
		static VALIDATOR_B:Mutex<bool> = Mutex::new(false);

		let mut profile_a:WindowRelativeProfile = WindowRelativeProfile::new("test_4", "", "");
		let mut profile_b:WindowRelativeProfile = WindowRelativeProfile::new("test_5", "", "");
		profile_a.add_named_operation("test_operation_all_profiles", |_profile| { *VALIDATOR_A.lock().unwrap() = true; Ok(()) });
		profile_b.add_named_operation("test_operation_all_profiles", |_profile| { *VALIDATOR_B.lock().unwrap() = true; Ok(()) });
		WindowRelativeSystem::add_profile(profile_a);
		WindowRelativeSystem::add_profile(profile_b);

		WindowRelativeSystem::execute_named_operation_on_all_profiles("test_operation_all_profiles");

		assert!(*VALIDATOR_A.lock().unwrap());
		assert!(*VALIDATOR_B.lock().unwrap());
	}
}