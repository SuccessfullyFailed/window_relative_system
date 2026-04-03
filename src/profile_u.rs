#[cfg(test)]
mod tests {
	use crate::{ ProfileStatus, WindowRelativeProfile, WindowRelativeProfileEssentials, TaskSystem };
	use window_controller::WindowController;
	use std::{ error::Error, sync::Mutex };



	static EVENT_RUN_PROOF:Mutex<usize> = Mutex::new(0);



	#[derive(WindowRelativeProfileEssentials)]
	struct TestProfile {
		name:&'static str,
		process_name:&'static str,
		task_system:TaskSystem,
		status:ProfileStatus
	}
	impl WindowRelativeProfile for TestProfile {
		fn on_event(&mut self, _window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {
			if event_name == "custom_event_tag" {
				*EVENT_RUN_PROOF.lock().unwrap() = 1;
			}
			Ok(())
		}
	}
	impl Default for TestProfile {
		fn default() -> Self {
			TestProfile {
				name: "test_profile",
				process_name: "test_process_name.exe",
				task_system: TaskSystem::default(),
				status: ProfileStatus::default()
			}
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