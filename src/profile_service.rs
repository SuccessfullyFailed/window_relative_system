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
	
	/// Add a new service.
	pub fn add_service<Service:WindowRelativeProfileService<ProfileStruct> + 'static>(&mut self, service:Service) {
		self.0.push(Arc::new(service));
	}



	/* USAGE METHODS */
	
	/// Get a cloned iterator.
	pub fn cloned_iter(&self) -> Vec<Arc<dyn WindowRelativeProfileService<ProfileStruct> + Send + Sync>> {
		self.0.iter().map(|arc| arc.clone()).collect()
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