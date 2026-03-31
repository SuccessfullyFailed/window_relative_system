use std::{ error::Error, sync::{ Arc, Condvar, Mutex, MutexGuard } };
use crate::{ ProfileStatus, WindowRelativeProfile, window_hook };
use window_controller::WindowController;
use winapi::shared::windef::HWND__;



pub struct WindowRelativeSystem {
	profiles:Vec<Box<dyn WindowRelativeProfile>>,
	default_profile:Box<dyn WindowRelativeProfile>,
	active_profile_index:Option<usize>,
	window_change_trigger:Arc<(Mutex<(Option<u64>, u64)>, Condvar)>,
	error_handler:Box<dyn Fn(&dyn WindowRelativeProfile, &dyn Error) + Send + Sync + 'static>
}
impl WindowRelativeSystem {

	/* CONSTRUCTOR METHODS */
	
	/// Create a new system.
	pub fn new<Profile:WindowRelativeProfile + 'static>(default_profile:Profile) -> Self {
		WindowRelativeSystem {
			profiles: Vec::new(),
			default_profile: Box::new(default_profile),
			active_profile_index: None,
			window_change_trigger: window_hook::signal_trigger(),
			error_handler: Box::new(|profile, error| eprintln!("WindowRelativeSystem error on profile {}: {:?}", profile.name(), error))
		}
	}

	/// Return with a custom error-handler.
	pub fn with_error_handler<ErrorHandler:Fn(&dyn WindowRelativeProfile, &dyn Error) + Send + Sync + 'static>(mut self, error_handler:ErrorHandler) -> Self {
		self.set_error_handler(error_handler);
		self
	}

	/// Set the error handler.
	pub fn set_error_handler<ErrorHandler:Fn(&dyn WindowRelativeProfile, &dyn Error) + Send + Sync + 'static>(&mut self, error_handler:ErrorHandler) {
		self.error_handler = Box::new(error_handler);
	}

	/// Return self with a profile.
	pub fn with_profile<Profile:WindowRelativeProfile + 'static>(mut self, profile:Profile) -> Self {
		self.add_profile(profile);
		self
	}

	/// Add a profile to the system.
	pub fn add_profile<Profile:WindowRelativeProfile + 'static>(&mut self, profile:Profile) {
		self.profiles.push(Box::new(profile));
	}


	
	/* USAGE METHODS */

	/// Run the system.
	/// Listens to window-change events indefinitely.
	pub fn run(&mut self) {

		// Launch window-hook if it does not exist yet.
		window_hook::launch_hook_if_not_exist();

		// Loop indefinitely.
		let mut last_seen:(Option<u64>, u64) = (None, 0);
		loop {
			// Wait until window-change trigger.
			let window_change_hwnds = {
				let (lock, cvar) = &*self.window_change_trigger;
				let mut window_change_hwnds:MutexGuard<'_, (Option<u64>, u64)> = lock.lock().unwrap();
				while *window_change_hwnds == last_seen {
					window_change_hwnds = cvar.wait(window_change_hwnds).unwrap();
				}
				last_seen = *window_change_hwnds;
				*window_change_hwnds
			};

			// Handle window-change.
			let active_window:WindowController = WindowController::from_hwnd(window_change_hwnds.1 as *mut HWND__);
			let previous_active_window:Option<WindowController> = window_change_hwnds.0.map(|pointer| WindowController::from_hwnd(pointer as *mut HWND__));
			if let Err(error) = self.set_active_window(&previous_active_window, &active_window) {
				(self.error_handler)(self.profile_with_index(self.active_profile_index), &*error)
			}
		}
	}

	/// Set a specific window as active.
	/// Will activate the according window-relative profile.
	pub fn set_active_window(&mut self, previous_window:&Option<WindowController>, current_window:&WindowController) -> Result<(), Box<dyn Error>> {
		
		// Find the active profile index.
		let active_process_name:String = current_window.process_name()?;
		let active_process_title:String = current_window.title();
		let mut next_active_profile_index:Option<usize> = None;
		for (profile_index, profile) in self.profiles.iter().enumerate() {
			if profile.matches_window(current_window, &active_process_name, &active_process_title) {
				next_active_profile_index = Some(profile_index);
				break;
			}
		}
		if next_active_profile_index == self.active_profile_index {
			return Ok(());
		}

		// Handle previous profile deactivation.
		if let Some(previous_window) = previous_window {
			let previous_profile:&mut dyn WindowRelativeProfile = self.profile_with_index_mut(self.active_profile_index);
			previous_profile.on_deactivate();
			previous_profile.on_event(previous_window, "deactivate")?;
			previous_profile.task_system_mut().stop();
			*previous_profile.status_mut() = ProfileStatus::Deactivated;
		}

		// Handle switch to new profile.
		self.active_profile_index = next_active_profile_index;

		// Handle new profile activation.
		let new_profile_registry:&mut dyn WindowRelativeProfile = self.profile_with_index_mut(self.active_profile_index);
		if new_profile_registry.status() == &ProfileStatus::Uninitialized {
			new_profile_registry.on_open();
			new_profile_registry.on_event(current_window, "open")?;
		}
		*new_profile_registry.status_mut() = ProfileStatus::Active;
		new_profile_registry.on_activate();
		new_profile_registry.on_event(current_window, "activate")?;
		new_profile_registry.task_system_mut().start();

		// Return success.
		Ok(())
	}

	/// Get a window-relative profile with the given index.
	/// Will return the default profile on None.
	fn profile_with_index(&self, index:Option<usize>) -> &dyn WindowRelativeProfile {
		if let Some(index) = index {
			if index < self.profiles.len() {
				return &*self.profiles[index];
			}
		}
		&*self.default_profile
	}

	/// Get a mutable window-relative profile with the given index.
	/// Will return the default profile on None.
	fn profile_with_index_mut(&mut self, index:Option<usize>) -> &mut dyn WindowRelativeProfile {
		if let Some(index) = index {
			if index < self.profiles.len() {
				return &mut *self.profiles[index];
			}
		}
		&mut *self.default_profile
	}


	/* EXECUTION METHODS */

	/// Execute an action on all profiles.
	/// Includes the default profile.
	pub fn execute_on_all_profiles<Action:FnMut(&mut dyn WindowRelativeProfile) -> ReturnType, ReturnType>(&mut self, mut action:Action) -> Vec<ReturnType> {
		vec![
			vec![action(&mut *self.default_profile)],
			self.profiles.iter_mut().map(|profile| action(&mut **profile)).collect()
		].into_iter().flatten().collect()
	}

	/// Execute an action on the currently activated profile.
	/// Uses the default profile if no profile is active.
	pub fn execute_on_current_profile<Action:FnMut(&mut dyn WindowRelativeProfile) -> ReturnType, ReturnType>(&mut self, mut action:Action) -> ReturnType {
		action(self.profile_with_index_mut(self.active_profile_index))
	}

	/// Execute an action on the default profile.
	pub fn execute_on_default_profile<Action:FnMut(&mut dyn WindowRelativeProfile) -> ReturnType, ReturnType>(&mut self, mut action:Action) -> ReturnType {
		action(&mut *self.default_profile)
	}

	/// Execute an action on the profile with the given name.
	/// Does nothing if the profile does not exist.
	pub fn execute_on_profile_with_name<Action:FnMut(&mut dyn WindowRelativeProfile) -> ReturnType, ReturnType>(&mut self, name:&str, mut action:Action) -> Option<ReturnType> {
		if name == self.default_profile.name() {
			return Some(action(&mut *self.default_profile));
		}
		for profile in &mut self.profiles {
			if profile.name() == name {
				return Some(action(&mut **profile));
			}
		}
		None
	}
}