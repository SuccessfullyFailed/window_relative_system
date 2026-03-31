use task_syncer::{ TaskScheduler, TaskSystem };
use window_controller::WindowController;
use std::error::Error;



#[derive(PartialEq)]
pub enum ProfileStatus { Uninitialized, Deactivated, Active }



pub trait WindowRelativeProfile:Send + Sync + 'static {

	/* METHODS TO IMPLEMENT */

	/// Get the name of the profile.
	fn name(&self) -> &str;

	/// Get the process-name of the profile.
	fn process_name(&self) -> &str;

	/// Get a reference to the task system.
	fn task_system(&self) -> &TaskSystem;

	/// Get a mutable reference to the task system.
	fn task_system_mut(&mut self) -> &mut TaskSystem;

	/// Get the status of the profile.
	fn status(&self) -> &ProfileStatus;

	/// Get a mutable reference to the status of the profile.
	fn status_mut(&mut self) -> &mut ProfileStatus;



	/* PROPERTY GETTER METHODS */

	/// Whether or not this profile is the active one.
	#[allow(unused_variables)]
	fn matches_window(&self, active_window:&WindowController, active_process_name:&str, active_process_title:&str) -> bool {
		self.process_name() == active_process_name
	}

	/// Get the task scheduler of this profile.
	fn task_scheduler(&self) -> &TaskScheduler {
		self.task_system().scheduler()
	}



	/* HANDLER METHODS */

	/// Executes when the profile is initially opened.
	fn on_open(&mut self) {}

	/// Executes when the profile is activated.
	fn on_activate(&mut self) {}

	/// Executes when the profile is deactivated.
	fn on_deactivate(&mut self) {}

	/// Executes when any named event is triggered.
	/// Includes the 'open', 'activate' and 'deactivate' events.
	#[allow(unused_variables)]
	fn on_event(&mut self, window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {
		Ok(())
	}
}



pub struct WindowRelativeProfileCore {
	name:String,
	process_name:String,
	status:ProfileStatus,
	task_system:TaskSystem
}
impl WindowRelativeProfileCore {
	pub fn new(name:&str, process_name:&str) -> WindowRelativeProfileCore {
		WindowRelativeProfileCore {
			name: name.to_string(),
			process_name: process_name.to_string(),
			status: ProfileStatus::Deactivated,
			task_system: TaskSystem::new()
		}
	}
}
pub trait HasWindowRelativeProfileCore {
	fn window_relative_profile_core(&self) -> &WindowRelativeProfileCore;
	fn window_relative_profile_core_mut(&mut self) -> &mut WindowRelativeProfileCore;
}
impl<T:HasWindowRelativeProfileCore + Send + Sync + 'static> WindowRelativeProfile for T {
	fn name(&self) -> &str { &self.window_relative_profile_core().name }
	fn process_name(&self) -> &str { &self.window_relative_profile_core().process_name }
	fn status(&self) -> &ProfileStatus { &self.window_relative_profile_core().status }
	fn status_mut(&mut self) -> &mut ProfileStatus { &mut self.window_relative_profile_core_mut().status }
	fn task_system(&self) -> &TaskSystem { &self.window_relative_profile_core().task_system }
	fn task_system_mut(&mut self) -> &mut TaskSystem { &mut self.window_relative_profile_core_mut().task_system }
}
impl HasWindowRelativeProfileCore for WindowRelativeProfileCore {
	fn window_relative_profile_core(&self) -> &WindowRelativeProfileCore { self }
	fn window_relative_profile_core_mut(&mut self) -> &mut WindowRelativeProfileCore { self }
}