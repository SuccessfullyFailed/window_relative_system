use crate::WindowRelativeProfileProperties;
use window_controller::WindowController;
use std::{ error::Error, ops::{BitAnd, BitOr} };
use task_syncer::TaskScheduler;



#[derive(Clone, PartialEq, Eq, Debug)]
pub enum WindowRelativeServiceTrigger {
	None,
	Open,
	Activate,
	Deactivate,
	Close,
	All, // Does not include named events.
	NamedEvent(String),
	Combined(Vec<WindowRelativeServiceTrigger>)
}
impl BitOr for WindowRelativeServiceTrigger {
	type Output = WindowRelativeServiceTrigger;

	fn bitor(self, rhs:Self) -> Self::Output {
		match self {
			WindowRelativeServiceTrigger::Combined(mut triggers) => {
				triggers.push(rhs);
				WindowRelativeServiceTrigger::Combined(triggers)
			},
			_ => WindowRelativeServiceTrigger::Combined(vec![self, rhs])
		}
	}
}
impl BitAnd for WindowRelativeServiceTrigger {
	type Output = bool;

	fn bitand(self, rhs:Self) -> Self::Output {
		match self {
			WindowRelativeServiceTrigger::Combined(triggers) => triggers.into_iter().any(|trigger| trigger & rhs.clone()),
			WindowRelativeServiceTrigger::All => match rhs {
				WindowRelativeServiceTrigger::Open => true,
				WindowRelativeServiceTrigger::Activate => true,
				WindowRelativeServiceTrigger::Deactivate => true,
				WindowRelativeServiceTrigger::Close => true,
				_ => false
			},
			WindowRelativeServiceTrigger::NamedEvent(event_l) => match rhs { WindowRelativeServiceTrigger::NamedEvent(event_r) => event_l == event_r, _ => false },
			_ => self == rhs
		}
	}
}



pub trait WindowRelativeProfileService:Send + Sync {

	/// The name of the service.
	fn name(&self) -> &str;

	/// When the service should trigger.
	fn when_to_trigger(&self) -> WindowRelativeServiceTrigger {
		WindowRelativeServiceTrigger::None
	}

	/// Install the service. This function is run when the service is applied to the profile.
	fn install(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler) {
	}

	/// Run the service. Requires 'when_to_trigger' to be implemented to execute.
	fn run(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, _trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}