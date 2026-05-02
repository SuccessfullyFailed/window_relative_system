use window_relative_system::{ TaskSystem, WindowRelativeProfile, WindowRelativeSystem };
use window_controller::WindowController;



pub struct ProfileBareBones {
	name:String,
	process_name:String,
	task_system:TaskSystem
}
impl ProfileBareBones {
	fn new(name:&str, process_name:&str) -> ProfileBareBones {
		ProfileBareBones {
			name: name.to_string(),
			process_name: process_name.to_string(),
			task_system: TaskSystem::new()
		}
	}
}
impl WindowRelativeProfile for ProfileBareBones {
	fn name(&self) -> &str {
		&self.name
	}

	fn task_system(&self) -> &TaskSystem {
		&self.task_system
	}

	fn task_system_mut(&mut self) -> &mut TaskSystem {
		&mut self.task_system
	}

	fn matches_window(&self, _window:&WindowController, process_name:&str, _process_title:&str) -> bool {
		process_name == &self.process_name
	}
}



pub fn main() {
	let mut system:WindowRelativeSystem<ProfileBareBones> = WindowRelativeSystem::new(ProfileBareBones::new("default_profile", "default_process_name"));
	system.add_profile(ProfileBareBones::new("active_process", &WindowController::active().process_name().unwrap_or_default()));
	system.run();
}