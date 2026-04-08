use task_syncer::{ TaskScheduler, TaskSystem };
use window_controller::WindowController;
use std::error::Error;



#[derive(PartialEq)]
pub enum WindowRelativeProfileStatus { Uninitialized, Deactivated, Active }
impl Default for WindowRelativeProfileStatus {
	fn default() -> Self {
		WindowRelativeProfileStatus::Uninitialized
	}
}


#[macro_export]
macro_rules! window_relative_profile {
	($type:ident, $name:expr, $process_name:expr) => {
		window_relative_profile!($type);
		impl Default for $type {
			fn default() -> Self {
				$type {
					name: $name,
					process_name: $process_name,
					task_system: window_relative_system::TaskSystem::default(),
					status: window_relative_system::WindowRelativeProfileStatus::default()
				}
			}
		}
	};
	($type:ident) => {
		pub struct $type {
			name:&'static str,
			process_name:&'static str,
			task_system:window_relative_system::TaskSystem,
			status:window_relative_system::WindowRelativeProfileStatus
		}
		window_relative_system::implement_window_relative_profile_essentials!($type);
	};
}
#[macro_export]
macro_rules! implement_window_relative_profile_essentials {
	($type:ty) => {
		impl window_relative_system::WindowRelativeProfileEssentials for $type {
			fn name(&self) -> &str { &self.name }
			fn process_name(&self) -> &str { &self.process_name }
			fn task_system(&self) -> &window_relative_system::TaskSystem { &self.task_system }
			fn task_system_mut(&mut self) -> &mut window_relative_system::TaskSystem { &mut self.task_system }
			fn status(&self) -> &window_relative_system::WindowRelativeProfileStatus { &self.status }
			fn status_mut(&mut self) -> &mut window_relative_system::WindowRelativeProfileStatus { &mut self.status }
		}
	};
}


pub trait WindowRelativeProfileEssentials:Send + Sync + 'static {

	/// Get the name of the profile.
	fn name(&self) -> &str;

	/// Get the process-name of the profile.
	fn process_name(&self) -> &str;

	/// Get a reference to the task system.
	fn task_system(&self) -> &TaskSystem;

	/// Get a mutable reference to the task system.
	fn task_system_mut(&mut self) -> &mut TaskSystem;

	/// Get the status of the profile.
	fn status(&self) -> &WindowRelativeProfileStatus;

	/// Get a mutable reference to the status of the profile.
	fn status_mut(&mut self) -> &mut WindowRelativeProfileStatus;
}
pub trait WindowRelativeProfile:WindowRelativeProfileEssentials {

	/* PROPERTY GETTER METHODS */

	/// Whether or not this profile is the active one.
	#[allow(unused_variables)]
	fn matches_window(&self, active_window:&WindowController, active_process_name:&str, active_process_title:&str) -> bool {
		self.process_name() == active_process_name
	}

	/// Get the task scheduler of this profile.
	fn task_scheduler(&self) -> TaskScheduler {
		self.task_system().scheduler()
	}



	/* HANDLER METHODS */

	/// Executes when the profile is initially opened.
	fn on_open(&mut self) -> Result<(), Box<dyn Error>> { Ok(()) }

	/// Executes when the profile is activated.
	fn on_activate(&mut self) -> Result<(), Box<dyn Error>> { Ok(()) }

	/// Executes when the profile is deactivated.
	fn on_deactivate(&mut self) -> Result<(), Box<dyn Error>> { Ok(()) }

	/// Execute a named event.
	fn execute_event(&mut self, window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {
		self.on_event(window, event_name)
	}

	/// Executes when any named event is triggered.
	/// Includes the 'open', 'activate' and 'deactivate' events.
	#[allow(unused_variables)]
	fn on_event(&mut self, window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}