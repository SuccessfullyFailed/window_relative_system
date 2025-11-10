use std::{ error::Error, ops::{ BitAnd, BitOr } };
use crate::WindowRelativeProfileProperties;
use task_syncer::TaskScheduler;
use window_controller::WindowController;



#[derive(Clone, Copy, PartialEq, Debug)]
pub struct WindowRelativeServiceTrigger(u8);
impl WindowRelativeServiceTrigger {
	pub const NONE:WindowRelativeServiceTrigger = WindowRelativeServiceTrigger(0);
	pub const OPEN:WindowRelativeServiceTrigger = WindowRelativeServiceTrigger(1);
	pub const ACTIVATE:WindowRelativeServiceTrigger = WindowRelativeServiceTrigger(2);
	pub const DEACTIVATE:WindowRelativeServiceTrigger = WindowRelativeServiceTrigger(4);
	pub const CLOSE:WindowRelativeServiceTrigger = WindowRelativeServiceTrigger(8);
	pub const ALL:WindowRelativeServiceTrigger = WindowRelativeServiceTrigger(0xFF);
}
impl BitAnd for WindowRelativeServiceTrigger {
	type Output = WindowRelativeServiceTrigger;

	fn bitand(self, rhs:Self) -> Self::Output {
		WindowRelativeServiceTrigger(self.0 & rhs.0)
	}
}
impl BitOr for WindowRelativeServiceTrigger {
	type Output = WindowRelativeServiceTrigger;

	fn bitor(self, rhs:Self) -> Self::Output {
		WindowRelativeServiceTrigger(self.0 | rhs.0)
	}
}



pub trait WindowRelativeProfileService:Send + Sync {

	/// The name of the service.
	fn name(&self) -> &str;

	/// When the service should trigger.
	fn when_to_trigger(&self) -> WindowRelativeServiceTrigger {
		WindowRelativeServiceTrigger::NONE
	}

	/// Install the service. This function is run when the service is applied to the profile.
	fn install(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
	}

	/// Run the service. Requires 'when_to_trigger' to be implemented to execute.
	fn run(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, _trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}