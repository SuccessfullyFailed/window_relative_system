use task_syncer::{ TaskScheduler, TaskSystem };
use window_controller::WindowController;
use std::error::Error;



#[macro_export]
macro_rules! window_relative_profile {
	($type:ident, $name:expr, $process_name:expr, $($implementation_body:item)*) => {
		pub struct $type {
			task_system:window_relative_system::TaskSystem
		}
		impl Default for $type {
			fn default() -> Self {
				$type {
					task_system: window_relative_system::TaskSystem::default()
				}
			}
		}
		impl window_relative_system::WindowRelativeProfile for $type {
			fn name(&self) -> &str { $name }
			fn task_system(&self) -> &window_relative_system::TaskSystem { &self.task_system }
			fn task_system_mut(&mut self) -> &mut window_relative_system::TaskSystem { &mut self.task_system }
			fn matches_window(&self, _window:&window_relative_system::WindowController, process_name:&str, _process_title:&str) -> bool { process_name == $process_name }
			$($implementation_body)*
		}
	};
}



pub trait WindowRelativeProfile:Send + Sync + 'static {

	/* IMPLEMENT METHODS */

	/// Get the name of the profile.
	fn name(&self) -> &str;

	/// Get a reference to the task system.
	fn task_system(&self) -> &TaskSystem;

	/// Get a mutable reference to the task system.
	fn task_system_mut(&mut self) -> &mut TaskSystem;
	
	/// Whether or not this profile is linked to the given window.
	fn matches_window(&self, window:&WindowController, process_name:&str, process_title:&str) -> bool;




	/* PROPERTY GETTER METHODS */

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

impl<T:WindowRelativeProfile + ?Sized> WindowRelativeProfile for Box<T> {
	fn name(&self) -> &str {
		let unboxed:&T = &**self;
		unboxed.name()
	}
	fn task_system(&self) -> &TaskSystem {
		let unboxed:&T = &**self;
		unboxed.task_system()
	}
	
	fn task_system_mut(&mut self) -> &mut TaskSystem {
		let unboxed:&mut T = &mut **self;
		unboxed.task_system_mut()
	}
	
	fn matches_window(&self, window:&WindowController, process_name:&str, process_title:&str) -> bool {
		let unboxed:&T = &**self;
		unboxed.matches_window(window, process_name, process_title)
	}
}