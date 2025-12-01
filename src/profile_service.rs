use task_syncer::TaskScheduler;
use window_controller::WindowController;
use std::error::Error;





pub struct WindowRelativeProfileServiceSet(Vec<Box<dyn WindowRelativeProfileService>>);
impl WindowRelativeProfileServiceSet {

	/* CONSTRUCTOR METHODS */

	/// Create a new set.
	pub fn new() -> WindowRelativeProfileServiceSet {
		WindowRelativeProfileServiceSet(Vec::new())
	}

	/// Return self with a new service.
	pub fn with_service<Service:WindowRelativeProfileService + 'static>(mut self, service:Service) -> Self {
		self.push(service);
		self
	}
	


	/* SERVICE MODIFICATION METHODS */
	
	/// Add a new service.
	pub fn push<Service:WindowRelativeProfileService + 'static>(&mut self, service:Service) {
		self.0.push(Box::new(service));
	}

	/// Remove a service.
	pub fn remove(&mut self, index:usize) {
		self.0.remove(index);
	}

	/// Get the amount of services present.
	pub fn len(&self) -> usize {
		self.0.len()
	}



	/* USAGE METHODS */

	/// Run all services. Removes the services that are expired.
	pub fn run(&mut self, task_scheduler:&TaskScheduler, window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {
		let mut index:usize = 0;
		while index < self.0.len() {
			if self.0[index].trigger_on_event(event_name) {
				self.0[index].run(task_scheduler, window, event_name)?;
				if self.0[index].trigger_once() {
					self.0.remove(index);
				} else {
					index += 1;
				}
			} else {
				index += 1;
			}
		}
		Ok(())
	}
}



pub trait WindowRelativeProfileService:Send + Sync {

	/// Whether or not the service should only trigger once.
	fn trigger_once(&self) -> bool {
		false
	}

	/// Whether or not the service should trigger on the given event.
	fn trigger_on_event(&self, _event_name:&str) -> bool {
		true
	}

	/// Run the service. Requires 'when_to_trigger' to be implemented to execute.
	fn run(&mut self, _task_scheduler:&TaskScheduler, _window:&WindowController, _event_name:&str) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}
impl<T:Fn(&TaskScheduler, &WindowController, &str) -> Result<(), Box<dyn Error>> + Send + Sync + 'static> WindowRelativeProfileService for T {
	fn run(&mut self, task_scheduler:&TaskScheduler, window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {
		self(task_scheduler, window, event_name)
	}
}