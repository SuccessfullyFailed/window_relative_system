#[cfg(test)]
mod tests {
	use crate::{ CoreWrapper, WindowRelativeProfile, WindowRelativeProfileCore, WindowRelativeProfileModifiers, WindowRelativeProfileProperties, WindowRelativeProfileService, WindowRelativeProfileServiceFunction, WindowRelativeServiceTrigger };
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
			fn install(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
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
			fn install(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
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
			fn install(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
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
			fn install(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
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
			fn install(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, _trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
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
	
	#[test]
	fn test_service_override() {
		static mut VALIDATION_VARIABLE:u8 = 0;

		struct TestService {}
		impl WindowRelativeProfileService for TestService {
			fn name(&self) -> &str {
				"TestService"
			}
			fn when_to_trigger(&self) -> WindowRelativeServiceTrigger {
				WindowRelativeServiceTrigger::None
			}
			fn install(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
				assert_eq!(trigger, WindowRelativeServiceTrigger::Open);
				unsafe { VALIDATION_VARIABLE += 1; }
				Ok(())
			}
		}

		let mut profile:CoreWrapper = CoreWrapper(WindowRelativeProfileCore::new("id", "title", "process_name"));
		profile.add_service(TestService {});
		profile.add_service_override(TestService {});
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
	fn test_service_convert_to_fn() {
		static mut VALIDATION_VARIABLE:u8 = 0;

		struct TestService {}
		impl WindowRelativeProfileService for TestService {
			fn name(&self) -> &str {
				"TestService"
			}
			fn when_to_trigger(&self) -> WindowRelativeServiceTrigger {
				WindowRelativeServiceTrigger::None
			}
			fn install(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
				unsafe { VALIDATION_VARIABLE = 10; }
			}
			fn run(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, _trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
				unsafe { VALIDATION_VARIABLE += 1; }
				Ok(())
			}
		}

		let profile:CoreWrapper = CoreWrapper(WindowRelativeProfileCore::new("id", "title", "process_name"));
		let window:WindowController = WindowController::active();
		let mut service_fn:Box<dyn FnMut(&WindowRelativeProfileProperties, &TaskScheduler, &WindowController) -> Result<(), Box<dyn Error + 'static>>> = TestService {}.as_function();


		assert_eq!(unsafe { VALIDATION_VARIABLE }, 0);
		service_fn(profile.properties(), profile.task_system().task_scheduler(), &window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 1);
		service_fn(profile.properties(), profile.task_system().task_scheduler(), &window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 2);
		service_fn(profile.properties(), profile.task_system().task_scheduler(), &window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 3);
	}
	
	#[test]
	fn test_service_convert_from_fn() {
		static mut VALIDATION_VARIABLE:u8 = 0;

		let service:Box<dyn Fn(&WindowRelativeProfileProperties, &TaskScheduler, &WindowController) -> Result<(), Box<dyn Error>> + Send + Sync + 'static> = Box::new(|_properties, _task_scheduler, _window| {
			unsafe { VALIDATION_VARIABLE += 1; }
			Ok(())
		});

		let mut profile:CoreWrapper = CoreWrapper(WindowRelativeProfileCore::new("id", "title", "process_name"));
		profile.add_service(service);
		let window:WindowController = WindowController::active();

		assert_eq!(unsafe { VALIDATION_VARIABLE }, 0);
		profile.core_mut().trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 0);
		profile.core_mut().trigger_activate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 0);
		profile.core_mut().trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 0);
		profile.core_mut().trigger_deactivate_event(&window).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 0);
		profile.core_mut().run_services_by_trigger(&window, WindowRelativeServiceTrigger::None).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 1);
		profile.core_mut().run_services_by_trigger(&window, WindowRelativeServiceTrigger::None).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 2);
		profile.core_mut().run_services_by_trigger(&window, WindowRelativeServiceTrigger::None).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 3);
		profile.core_mut().run_services_by_trigger(&window, WindowRelativeServiceTrigger::Activate).unwrap();
		assert_eq!(unsafe { VALIDATION_VARIABLE }, 3);
	}
}