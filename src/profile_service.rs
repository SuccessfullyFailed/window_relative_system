use window_controller::WindowController;
use std::{ error::Error, sync::Arc };



pub struct WindowRelativeProfileServiceSet<ProfileStruct>(Vec<Arc<dyn WindowRelativeProfileService<ProfileStruct> + Send + Sync>>);
impl<ProfileStruct> WindowRelativeProfileServiceSet<ProfileStruct> {

	/* CONSTRUCTOR METHODS */

	/// Create a new set.
	pub fn new() -> WindowRelativeProfileServiceSet<ProfileStruct> {
		WindowRelativeProfileServiceSet(Vec::new())
	}

	/// Return self with a new service.
	pub fn with_service<Service:WindowRelativeProfileService<ProfileStruct> + 'static>(mut self, service:Service) -> Self {
		self.add_service(service);
		self
	}
	


	/* SERVICE MODIFICATION METHODS */
	
	/// Add a new service.
	pub fn add_service<Service:WindowRelativeProfileService<ProfileStruct> + 'static>(&mut self, service:Service) {
		self.0.push(Arc::new(service));
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

	/// Run all services. Returns the indexes of the services that are expired.
	pub fn run(&self, profile:&mut ProfileStruct, window:&WindowController, event_name:&str) -> Result<Vec<usize>, Box<dyn Error>> {
		let mut expired:Vec<usize> = Vec::new();
		for (index,service) in self.0.iter().enumerate() {
			let run:bool = service.trigger_event_names().contains(&event_name) || service.trigger_event_names().contains(&"*");
			let run_once:bool = service.trigger_event_names() == &["once"];
			if run || run_once {
				service.run(profile, window, event_name)?;
				if run_once {
					expired.push(index);
				}
			}
		}
		Ok(expired)
	}
}
impl<T> Clone for WindowRelativeProfileServiceSet<T> {
	fn clone(&self) -> Self {
		WindowRelativeProfileServiceSet(self.0.iter().map(|arc| arc.clone()).collect())
	}
}



pub trait WindowRelativeProfileService<ProfileStruct>:Send + Sync {

	/// When the service should trigger.
	fn trigger_event_names(&self) -> &[&str] {
		&["*"]
	}

	/// Run the service. Requires 'when_to_trigger' to be implemented to execute.
	fn run(&self, _profile:&mut ProfileStruct, _window:&WindowController, _event_name:&str) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}
impl<ProfileStruct, T:Fn(&mut ProfileStruct, &WindowController, &str) -> Result<(), Box<dyn Error>> + Send + Sync + 'static> WindowRelativeProfileService<ProfileStruct> for T {
	fn run(&self, profile:&mut ProfileStruct, window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {
		self(profile, window, event_name)
	}
}