use crate::WindowRelativeProfileProperties;
use window_controller::WindowController;
use std::{ error::Error, ops::BitOr };
use task_syncer::TaskScheduler;



#[derive(Clone, PartialEq, Eq, Debug)]
pub enum WindowRelativeServiceTrigger {
	None,
	Open,
	Activate,
	Deactivate,
	Close,
	NamedEvent(String),
	Combined(Vec<WindowRelativeServiceTrigger>),

	StaticNamedEvent(&'static str),
	StaticRef(&'static WindowRelativeServiceTrigger),
	StaticCombined(&'static [WindowRelativeServiceTrigger])
}
impl WindowRelativeServiceTrigger {

	/// Whether or not should trigger on the given trigger.
	pub fn run_on_trigger(&self, rhs:&WindowRelativeServiceTrigger) -> bool {
		match self {
			WindowRelativeServiceTrigger::StaticCombined(triggers) => {
				triggers.into_iter().any(|trigger| trigger.run_on_trigger(rhs))
			},
			WindowRelativeServiceTrigger::StaticRef(trigger) => {
				trigger.run_on_trigger(rhs)
			},
			WindowRelativeServiceTrigger::StaticNamedEvent(event_l) => {
				match rhs {
					WindowRelativeServiceTrigger::NamedEvent(event_r) => event_l == event_r,
					_ => false
				}
			},


			WindowRelativeServiceTrigger::Combined(triggers) => {
				triggers.into_iter().any(|trigger| trigger.run_on_trigger(rhs))
			},
			WindowRelativeServiceTrigger::NamedEvent(event_l) => {
				match rhs {
					WindowRelativeServiceTrigger::NamedEvent(event_r) => event_l == event_r,
					_ => false
				}
			},
			_ => self == rhs
		}
	}
}
impl BitOr for WindowRelativeServiceTrigger {
	type Output = WindowRelativeServiceTrigger;

	fn bitor(self, rhs:Self) -> Self::Output {
		match self {
			WindowRelativeServiceTrigger::StaticRef(trigger) => trigger.clone() | rhs,
			WindowRelativeServiceTrigger::Combined(mut triggers) => {
				triggers.push(rhs);
				WindowRelativeServiceTrigger::Combined(triggers)
			},
			_ => WindowRelativeServiceTrigger::Combined(vec![self, rhs])
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

	/// Update the service once. Runs every time the profile is ran.
	fn update(&mut self) {
	}

	/// Run the service. Requires 'when_to_trigger' to be implemented to execute.
	fn run(&mut self, _properties:&WindowRelativeProfileProperties, _task_scheduler:&TaskScheduler, _window:&WindowController, _trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}

pub trait WindowRelativeProfileServiceFunction:WindowRelativeProfileService + Sized + 'static {

	/// Return the service as a single, runnable function.
	fn as_function(self) -> Box<dyn FnMut(&WindowRelativeProfileProperties, &TaskScheduler, &WindowController) -> Result<(), Box<dyn Error>>> {		
		let mut service:Self = self;
		Box::new(move |properties, task_scheduler, window| {
			service.run(properties, task_scheduler, window, WindowRelativeServiceTrigger::None)
		})
	}
}
impl<T:WindowRelativeProfileService + Sized + 'static> WindowRelativeProfileServiceFunction for T {}

impl<T:Fn(&WindowRelativeProfileProperties, &TaskScheduler, &WindowController) -> Result<(), Box<dyn Error>> + Send + Sync + 'static> WindowRelativeProfileService for T {
	fn name(&self) -> &str { "" }
	fn run(&mut self, properties:&WindowRelativeProfileProperties, task_scheduler:&TaskScheduler, window:&WindowController, _trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
		self(properties, task_scheduler, window)
	}
}