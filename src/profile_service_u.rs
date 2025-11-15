#[cfg(test)]
mod tests {
	use crate::{ CoreWrapper, WindowRelativeProfile, WindowRelativeProfileCore, WindowRelativeProfileModifiers, WindowRelativeProfileService, WindowRelativeServiceTrigger };
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
				WindowRelativeServiceTrigger::None
			}
			fn install(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
				assert_eq!(trigger, WindowRelativeServiceTrigger::Open);
				unsafe { VALIDATION_VARIABLE += 1; }
				Ok(())
			}
		}

		let mut profile:CoreWrapper = CoreWrapper(WindowRelativeProfileCore::new("id", "title", "process_name"));
		profile.add_service(TestService {});
		let window:WindowController = WindowController::active();

		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.core_mut().trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.core_mut().trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.core_mut().trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.core_mut().trigger_deactivate_event(&window).unwrap();
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
				WindowRelativeServiceTrigger::Open
			}
			fn install(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
				assert_eq!(trigger, WindowRelativeServiceTrigger::Open);
				unsafe { VALIDATION_VARIABLE += 1; }
				Ok(())
			}
		}

		let mut profile:CoreWrapper = CoreWrapper(WindowRelativeProfileCore::new("id", "title", "process_name"));
		profile.add_service(TestService {});
		let window:WindowController = WindowController::active();

		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.core_mut().trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 11);
		profile.core_mut().trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 11);
		profile.core_mut().trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 11);
		profile.core_mut().trigger_deactivate_event(&window).unwrap();
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
				WindowRelativeServiceTrigger::Activate
			}
			fn install(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
				assert_eq!(trigger, WindowRelativeServiceTrigger::Activate);
				unsafe { VALIDATION_VARIABLE += 1; }
				Ok(())
			}
		}

		let mut profile:CoreWrapper = CoreWrapper(WindowRelativeProfileCore::new("id", "title", "process_name"));
		profile.add_service(TestService {});
		let window:WindowController = WindowController::active();

		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.core_mut().trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 11);
		profile.core_mut().trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 12);
		profile.core_mut().trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 12);
		profile.core_mut().trigger_deactivate_event(&window).unwrap();
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
				WindowRelativeServiceTrigger::Deactivate
			}
			fn install(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
				assert_eq!(trigger, WindowRelativeServiceTrigger::Deactivate);
				unsafe { VALIDATION_VARIABLE += 1; }
				Ok(())
			}
		}

		let mut profile:CoreWrapper = CoreWrapper(WindowRelativeProfileCore::new("id", "title", "process_name"));
		profile.add_service(TestService {});
		let window:WindowController = WindowController::active();

		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.core_mut().trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.core_mut().trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.core_mut().trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 11);
		profile.core_mut().trigger_deactivate_event(&window).unwrap();
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
				WindowRelativeServiceTrigger::All
			}
			fn install(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, _trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
				unsafe { VALIDATION_VARIABLE += 1; }
				Ok(())
			}
		}

		let mut profile:CoreWrapper = CoreWrapper(WindowRelativeProfileCore::new("id", "title", "process_name"));
		profile.add_service(TestService {});
		let window:WindowController = WindowController::active();

		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.core_mut().trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 12);
		profile.core_mut().trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 13);
		profile.core_mut().trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 14);
		profile.core_mut().trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 15);
	}

	#[test]
	fn test_profile_service_on_named_event() {
		static mut VALIDATION_VARIABLE:u8 = 0;

		struct TestService {}
		impl WindowRelativeProfileService for TestService {
			fn name(&self) -> &str {
				"TestService"
			}
			fn when_to_trigger(&self) -> WindowRelativeServiceTrigger {
				WindowRelativeServiceTrigger::NamedEvent("test_event".to_string())
			}
			fn install(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&crate::WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, _trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
				unsafe { VALIDATION_VARIABLE += 1; }
				Ok(())
			}
		}

		let mut profile:CoreWrapper = CoreWrapper(WindowRelativeProfileCore::new("id", "title", "process_name"));
		profile.add_service(TestService {});
		let window:WindowController = WindowController::active();

		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.core_mut().trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.core_mut().trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.core_mut().trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.core_mut().trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.trigger_event("fake_test_event").unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 10);
		profile.trigger_event("test_event").unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 11);
		profile.trigger_event("test_event").unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 12);
		profile.trigger_event("").unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 12);
	}
}