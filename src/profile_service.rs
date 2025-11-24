use crate::WindowRelativeProfileProperties;
use window_controller::WindowController;
use std::{ error::Error };
use task_syncer::TaskScheduler;



pub trait WindowRelativeProfileService<ProfileStruct>:Send + Sync {

	/// Install the service. This function is run when the service is applied to the profile.
	fn install(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
	}

	/// Update the service once. Runs every time the profile is ran.
	fn update(&mut self, _profile:&mut ProfileStruct) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	/// When the service should trigger.
	fn trigger_event_names(&self) -> &[&str] {
		&[]
	}

	/// Run the service. Requires 'when_to_trigger' to be implemented to execute.
	fn run(&mut self, _profile:&mut ProfileStruct, _window:&WindowController, _event_name:&str) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}
impl<ProfileStruct, T:Fn(&mut ProfileStruct, &WindowController, &str) -> Result<(), Box<dyn Error>> + Send + Sync + 'static> WindowRelativeProfileService<ProfileStruct> for T {
	fn run(&mut self, profile:&mut ProfileStruct, window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {
		self(profile, window, event_name)
	}
}