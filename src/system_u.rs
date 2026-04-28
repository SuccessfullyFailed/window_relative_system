#[cfg(test)]
mod tests {
	use crate::{ TaskSystem, WindowRelativeProfile, WindowRelativeSystem, WindowRelativeSystemRemoteControl };
	use std::{ sync::Mutex, time::Duration, thread::{ self, sleep } };
	

	struct WindowRelativeProfileCore {
		name:&'static str,
		process_name:&'static str,
		task_system:TaskSystem
	}
	impl WindowRelativeProfile for WindowRelativeProfileCore {
		fn name(&self) -> &str { self.name }
		fn task_system(&self) -> &TaskSystem { &self.task_system }
		fn task_system_mut(&mut self) -> &mut TaskSystem { &mut self.task_system }
		fn matches_window(&self, _window:&window_controller::WindowController, process_name:&str, _process_title:&str) -> bool {
			process_name == self.process_name
		}
	}
	impl WindowRelativeProfileCore {
		fn new(name:&'static str, process_name:&'static str) -> WindowRelativeProfileCore {
			WindowRelativeProfileCore {
				name,
				process_name,
				task_system: TaskSystem::new()
			}
		}
	}



	const DEFAULT_PROFILE_NAME:&str = "default_test_profile_name";
	const DEFAULT_PROFILE_PROCESS_NAME:&str = "default_test_profile_process_name";
	const SECONDARY_PROFILE_NAME:&str = "secondary_profile_name";
	const SECONDARY_PROFILE_PROCESS_NAME:&str = "secondary_process_name";
	fn test_system() -> WindowRelativeSystem {
		WindowRelativeSystem::new(Box::new(WindowRelativeProfileCore::new(DEFAULT_PROFILE_NAME, DEFAULT_PROFILE_PROCESS_NAME)))
			.with_profile(Box::new(WindowRelativeProfileCore::new(SECONDARY_PROFILE_NAME, SECONDARY_PROFILE_PROCESS_NAME)))
	}



	/* EXECUTION METHODS TESTS */

	#[test]
	fn test_system_execute_on_all() {
		let profile_names:Vec<String> = {
			test_system().execute_on_all_profiles(|profile| {
				profile.name().to_string()
			})
		};
		assert_eq!(profile_names, vec![DEFAULT_PROFILE_NAME.to_string(), SECONDARY_PROFILE_NAME.to_string()]);
	}

	#[test]
	fn test_system_execute_on_current_profile() {
		let current_profile_name:String = {
			test_system().execute_on_current_profile(|profile| {
				profile.name().to_string()
			})
		};
		assert_eq!(current_profile_name, DEFAULT_PROFILE_NAME);
	}

	#[test]
	fn test_system_execute_on_profile_by_id() {
		let profile_name:Option<String> = {
			test_system().execute_on_profile_with_name(SECONDARY_PROFILE_NAME, |profile| {
				profile.name().to_string()
			})
		};
		assert_eq!(profile_name, Some(SECONDARY_PROFILE_NAME.to_string()));
	}

	#[test]
	fn test_system_on_default_profile() {
		assert!(
			test_system().execute_on_default_profile(|profile| profile.name() == DEFAULT_PROFILE_NAME)
		);
	}



	/* MISCELLANEOUS TESTS */

	#[test]
	fn test_system_can_be_sent_between_threads() {
		let mut system:WindowRelativeSystem = test_system();
		thread::spawn(move || {
			system.execute_on_default_profile(|_| {});
		});
	}

	#[test]
	fn test_system_can_be_used_while_running() {
		static PROFILE_NAME:Mutex<String> = Mutex::new(String::new());
		let mut system:WindowRelativeSystem = test_system();
		let remote:WindowRelativeSystemRemoteControl = system.create_remote();
		thread::spawn(move || {
			sleep(Duration::from_millis(10));
			remote.execute_on_default_profile(|profile| *PROFILE_NAME.lock().unwrap() = profile.name().to_string());
			sleep(Duration::from_millis(1));
			assert_eq!(*PROFILE_NAME.lock().unwrap(), DEFAULT_PROFILE_NAME);
		});
		thread::spawn(move || {
			system.run();
		});
		sleep(Duration::from_millis(500));
	}
}