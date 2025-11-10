#[cfg(test)]
mod tests {
	use crate::{ WindowRelativeProfileCore, WindowRelativeProfileService, WindowRelativeServiceTrigger };
	use window_controller::WindowController;
	use task_syncer::TaskScheduler;
	use std::error::Error;



	#[test]
	fn test_profile_service_on_none() {
		static mut VALIDATION_VARIABLE:u8 = 0;

		struct TestService {}
		impl WindowRelativeProfileService for TestService {
			fn name(&self) -> &str {
				"TestService"
			}
			fn when_to_trigger(&self) -> WindowRelativeServiceTrigger {
				WindowRelativeServiceTrigger::NONE
			}
			fn install(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
				assert_eq!(trigger, WindowRelativeServiceTrigger::OPEN);
				unsafe { VALIDATION_VARIABLE += 1; }
				Ok(())
			}
		}

		let mut profile:WindowRelativeProfileCore = WindowRelativeProfileCore::new("id", "title", "process_name");
		profile.add_service(TestService {});
		let window:WindowController = WindowController::active();

		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
	}

	#[test]
	fn test_profile_service_on_open() {
		static mut VALIDATION_VARIABLE:u8 = 0;

		struct TestService {}
		impl WindowRelativeProfileService for TestService {
			fn name(&self) -> &str {
				"TestService"
			}
			fn when_to_trigger(&self) -> WindowRelativeServiceTrigger {
				WindowRelativeServiceTrigger::OPEN
			}
			fn install(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
				assert_eq!(trigger, WindowRelativeServiceTrigger::OPEN);
				unsafe { VALIDATION_VARIABLE += 1; }
				Ok(())
			}
		}

		let mut profile:WindowRelativeProfileCore = WindowRelativeProfileCore::new("id", "title", "process_name");
		profile.add_service(TestService {});
		let window:WindowController = WindowController::active();

		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 11);
		profile.trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 11);
		profile.trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 11);
		profile.trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 11);
	}

	#[test]
	fn test_profile_service_on_activate() {
		static mut VALIDATION_VARIABLE:u8 = 0;

		struct TestService {}
		impl WindowRelativeProfileService for TestService {
			fn name(&self) -> &str {
				"TestService"
			}
			fn when_to_trigger(&self) -> WindowRelativeServiceTrigger {
				WindowRelativeServiceTrigger::ACTIVATE
			}
			fn install(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
				assert_eq!(trigger, WindowRelativeServiceTrigger::ACTIVATE);
				unsafe { VALIDATION_VARIABLE += 1; }
				Ok(())
			}
		}

		let mut profile:WindowRelativeProfileCore = WindowRelativeProfileCore::new("id", "title", "process_name");
		profile.add_service(TestService {});
		let window:WindowController = WindowController::active();

		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 11);
		profile.trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 12);
		profile.trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 12);
		profile.trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 12);
	}

	#[test]
	fn test_profile_service_on_deactivate() {
		static mut VALIDATION_VARIABLE:u8 = 0;

		struct TestService {}
		impl WindowRelativeProfileService for TestService {
			fn name(&self) -> &str {
				"TestService"
			}
			fn when_to_trigger(&self) -> WindowRelativeServiceTrigger {
				WindowRelativeServiceTrigger::DEACTIVATE
			}
			fn install(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
				assert_eq!(trigger, WindowRelativeServiceTrigger::DEACTIVATE);
				unsafe { VALIDATION_VARIABLE += 1; }
				Ok(())
			}
		}

		let mut profile:WindowRelativeProfileCore = WindowRelativeProfileCore::new("id", "title", "process_name");
		profile.add_service(TestService {});
		let window:WindowController = WindowController::active();

		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 11);
		profile.trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 12);
	}

	#[test]
	fn test_profile_service_on_all() {
		static mut VALIDATION_VARIABLE:u8 = 0;

		struct TestService {}
		impl WindowRelativeProfileService for TestService {
			fn name(&self) -> &str {
				"TestService"
			}
			fn when_to_trigger(&self) -> WindowRelativeServiceTrigger {
				WindowRelativeServiceTrigger::ALL
			}
			fn install(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, _trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
				unsafe { VALIDATION_VARIABLE += 1; }
				Ok(())
			}
		}

		let mut profile:WindowRelativeProfileCore = WindowRelativeProfileCore::new("id", "title", "process_name");
		profile.add_service(TestService {});
		let window:WindowController = WindowController::active();

		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 12);
		profile.trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 13);
		profile.trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 14);
		profile.trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 15);
	}
}