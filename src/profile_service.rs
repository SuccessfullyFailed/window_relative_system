use window_controller::WindowController;
use std::{error::Error, sync::Arc};

use crate::WindowRelativeProfileSized;





pub struct WindowRelativeProfileServiceSet(Vec<Box<dyn WindowRelativeProfileService>>);
impl WindowRelativeProfileServiceSet {

	/* CONSTRUCTOR METHODS */

	/// Create a new set.
	pub fn new() -> WindowRelativeProfileServiceSet {
		WindowRelativeProfileServiceSet(Vec::new())
	}

	/// Return self with a new service.
	pub fn with_service<Service:WindowRelativeProfileService + 'static>(mut self, service:Service) -> Self {
		self.add_service(service);
		self
	}
	


	/* SERVICE MODIFICATION METHODS */
	
	/// Add a new service.
	pub fn add_service<Service:WindowRelativeProfileService + 'static>(&mut self, service:Service) {
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
	pub fn run(&mut self, window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {
		let mut index:usize = 0;
		while index < self.0.len() {
			self.0[index].run(window, event_name)?;
			if self.0[index].trigger_once() {
				self.0.remove(index);
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
	fn run(&mut self, _window:&WindowController, _event_name:&str) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}
impl<T:Fn(&WindowController, &str) -> Result<(), Box<dyn Error>> + Send + Sync + 'static> WindowRelativeProfileService for T {
	fn run(&mut self, window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {
		self(window, event_name)
	}
}




pub struct WindowRelativeProfileHandlerSet<Profile:WindowRelativeProfileSized>(Vec<Arc<dyn WindowRelativeProfileHandler<Profile>>>);
impl<Profile:WindowRelativeProfileSized> WindowRelativeProfileHandlerSet<Profile> {

	/* CONSTRUCTOR METHODS */

	/// Create a new set.
	pub fn new() -> WindowRelativeProfileHandlerSet<Profile> {
		WindowRelativeProfileHandlerSet(Vec::new())
	}

	/// Return self with a new service.
	pub fn with_service<Service:WindowRelativeProfileHandler<Profile> + 'static>(mut self, service:Service) -> Self {
		self.add_service(service);
		self
	}
	


	/* SERVICE MODIFICATION METHODS */
	
	/// Add a new service.
	pub fn add_service<Service:WindowRelativeProfileHandler<Profile> + 'static>(&mut self, service:Service) {
		self.0.push(Arc::new(service));
	}



	/* USAGE METHODS */

	/// Get all handlers cloned.
	pub fn handlers_cloned(&self) -> Vec<Arc<dyn WindowRelativeProfileHandler<Profile>>> {
		self.0.clone()
	}

	/// Run all services. Removes the services that are expired.
	pub fn run(&self, profile:&mut Profile, window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {
		for service in self.0.clone() {
			service.run(profile, window, event_name)?;
		}
		Ok(())
	}
}

pub trait WindowRelativeProfileHandler<Profile:WindowRelativeProfileSized>:Send + Sync {

	/// Run the service handler.
	fn run(&self, _profile:&mut Profile, _window:&WindowController, _event_name:&str) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}
impl<Profile:WindowRelativeProfileSized, T:Fn(&mut Profile, &WindowController, &str) -> Result<(), Box<dyn Error>> + Send + Sync + 'static> WindowRelativeProfileHandler<Profile> for T {
	fn run(&self, profile:&mut Profile, window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {
		self(profile, window, event_name)
	}
}