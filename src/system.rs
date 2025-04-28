use crate::{ DefaultProfile, WindowRelativeProfile };
use std::sync::{ Mutex, MutexGuard };
use window_controller::WindowController;



static SYSTEM_INST:Mutex<Option<WindowRelativeSystem>> = Mutex::new(None);



pub struct WindowRelativeSystem {
	profiles:Vec<Box<dyn WindowRelativeProfile>>,
	active_profile_index:usize
}
impl WindowRelativeSystem {
	
	/* USAGE METHODS */

	/// Add a profile to the system.
	pub fn add_profile<T>(profile:T) where T:WindowRelativeProfile + 'static {

		// Get lock on system.
		let mut system_guard:MutexGuard<'_, Option<WindowRelativeSystem>> = SYSTEM_INST.lock().unwrap();
		if system_guard.is_none() {
			*system_guard = Some(WindowRelativeSystem::default());
		}
		let system:&mut WindowRelativeSystem = (*system_guard).as_mut().unwrap();

		// Add profile.
		system.profiles.push(Box::new(profile));
	}

	/// Update the current profile.
	pub fn update_profile(current_window:WindowController) {

		// Get lock on system.
		let mut system_guard:MutexGuard<'_, Option<WindowRelativeSystem>> = SYSTEM_INST.lock().unwrap();
		if system_guard.is_none() {
			*system_guard = Some(WindowRelativeSystem::default());
		}
		let system:&mut WindowRelativeSystem = (*system_guard).as_mut().unwrap();

		// Find active profile.
		let active_process_name:String = current_window.process_name().unwrap_or_default();
		let active_process_title:String = current_window.title();
		let active_profile_index:usize = system.profiles.iter().position(|profile| profile.is_active(&active_process_name, &active_process_title)).unwrap_or(0);

		// Profile change.
		if active_profile_index != system.active_profile_index {
			system.profiles[system.active_profile_index].on_deactivate();
			system.active_profile_index = active_profile_index;
			system.profiles[system.active_profile_index].on_activate();
		}
	}
}
impl Default for WindowRelativeSystem {
	fn default() -> Self {
		WindowRelativeSystem {
			profiles: vec![Box::new(DefaultProfile::default())],
			active_profile_index: 0
		}
	}
}