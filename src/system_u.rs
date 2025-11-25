#[cfg(test)]
mod tests {
	use crate::{ TestCore, WindowRelativeProfile, WindowRelativeProfileProperties, WindowRelativeSystem };
	use std::{ sync::Mutex, thread::sleep, time::Duration };
	use task_syncer::TaskSystem;

	

	static TEST_PROFILE_ADDED:Mutex<bool> = Mutex::new(false);



	/* ADDING A PROFILE */

	#[test]
	fn test_system_add_profile() {
		WindowRelativeSystem::add_profile(TestCore::default());
		WindowRelativeSystem::execute_on_all_profiles(|profile| {
			if profile.id() == TestCore::ID {
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

		WindowRelativeSystem::execute_on_profile_by_id(TestCore::ID, |profile| {
			if profile.id() == TestCore::ID {
				*VALIDATOR.lock().unwrap() = true;
			}
		});

		assert!(*VALIDATOR.lock().unwrap());
	}

	#[test]
	fn test_system_execute_on_all_profiles() {
		use crate as window_relative_system;
		#[window_relative_profile_creator_macro::window_relative_profile]
		struct TestCoreB {}
		impl WindowRelativeProfile for TestCoreB {}

		static VALIDATOR_A:Mutex<bool> = Mutex::new(false);
		static VALIDATOR_B:Mutex<bool> = Mutex::new(false);
		await_test_profile();
		WindowRelativeSystem::add_profile(TestCoreB {
			properties: WindowRelativeProfileProperties::new("test_2", "", ""),
			task_system: TaskSystem::new(),
			handlers: Vec::new()
		});

		WindowRelativeSystem::execute_on_all_profiles(|profile| {
			if profile.id() == TestCore::ID {
				*VALIDATOR_A.lock().unwrap() = true;
			}
			if profile.id() == "test_2" {
				*VALIDATOR_B.lock().unwrap() = true;
			}
		});

		assert!(*VALIDATOR_A.lock().unwrap());
		assert!(*VALIDATOR_B.lock().unwrap());
	}
}