use window_relative_system::{ ProfileStatus, TaskSystem, WindowRelativeProfile, WindowRelativeProfileEssentials, WindowRelativeSystem, implement_window_relative_profile_essentials };
use window_controller::WindowController;



struct ProfileBareBones {
	name:String,
	process_name:String,
	task_system:TaskSystem,
	status:ProfileStatus
}
implement_window_relative_profile_essentials!(ProfileBareBones);
impl WindowRelativeProfile for ProfileBareBones {
	fn on_activate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		println!("\tprofile {} was activated.", &self.name);
		Ok(())
	}
}
impl ProfileBareBones {
	fn new(name:&str, process_name:&str) -> ProfileBareBones {
		ProfileBareBones {
			name: name.to_string(),
			process_name: process_name.to_string(),
			task_system: TaskSystem::new(),
			status: ProfileStatus::default()
		}
	}
}



pub fn main() {
	let mut system:WindowRelativeSystem = WindowRelativeSystem::new(ProfileBareBones::new("default_profile", "default_process_name"));
	system.add_profile(ProfileBareBones::new("active_process", &WindowController::active().process_name().unwrap_or_default()));
	system.run();
}