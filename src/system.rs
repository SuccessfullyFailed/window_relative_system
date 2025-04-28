use crate::{ DefaultProfile, WindowRelativeProfile };
use std::sync::{ Mutex, MutexGuard };



static SYSTEM_INST:Mutex<Option<WindowRelativeSystem>> = Mutex::new(None);



pub struct WindowRelativeSystem {
	profiles:Vec<Box<dyn WindowRelativeProfile>>,
	default_profile:DefaultProfile,
	active_profile_index:Option<usize>
}
impl WindowRelativeSystem {
	
	/* USAGE METHODS */

	/// Add a profile to the system.
	pub fn add_profile<T>(profile:T) where T:WindowRelativeProfile + 'static {

		// Fetch system.
		let mut system_guard:MutexGuard<'_, Option<WindowRelativeSystem>> = SYSTEM_INST.lock().unwrap();
		if system_guard.is_none() {
			*system_guard = Some(WindowRelativeSystem::default());
		}
		let system:&mut WindowRelativeSystem = (*system_guard).as_mut().unwrap();

		// Add profile.
		system.profiles.push(Box::new(profile));
	}
}
impl Default for WindowRelativeSystem {
	fn default() -> Self {
		WindowRelativeSystem {
			profiles: Vec::new(),
			default_profile: DefaultProfile::default(),
			active_profile_index: None
		}
	}
}