use crate::WindowRelativeProfileProperties;
use std::ops::{ BitAnd, BitOr };
use task_syncer::TaskScheduler;



#[derive(Clone, Copy, PartialEq, Debug)]
pub struct WindowRelativeServiceTrigger(u8);
impl WindowRelativeServiceTrigger {
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
	fn when_to_trigger(&self) -> WindowRelativeServiceTrigger;

	/// Run the service.
	fn run(&mut self, properties:&WindowRelativeProfileProperties, task_scheduler:&TaskScheduler, trigger:WindowRelativeServiceTrigger);
}