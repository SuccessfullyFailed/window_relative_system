use modifications_queue::{ModificationsQueue, ModificationsQueueRemote};
use crate::{ ProfileStatus, WindowRelativeProfile, window_hook };
use std::{ error::Error, sync::Arc };
use window_controller::WindowController;



pub struct WindowRelativeSystem {
	profiles:Vec<Box<dyn WindowRelativeProfile>>,
	default_profile:Box<dyn WindowRelativeProfile>,
	active_profile_index:Option<usize>,
	error_handler:Arc<dyn Fn(&str, Box<dyn Error>) + Send + Sync + 'static>,

	modifications_queue:ModificationsQueue<WindowRelativeSystem>,
	hook_remote_registered:bool
}
impl WindowRelativeSystem {

	/* CONSTRUCTOR METHODS */
	
	/// Create a new system.
	pub fn new<Profile:WindowRelativeProfile + 'static>(default_profile:Profile) -> Self {
		WindowRelativeSystem {
			profiles: Vec::new(),
			default_profile: Box::new(default_profile),
			active_profile_index: None,
			error_handler: Arc::new(|profile_name, error| eprintln!("WindowRelativeSystem error on profile {}: {:?}", profile_name, error)),

			modifications_queue: ModificationsQueue::new(),
			hook_remote_registered: false
		}
	}

	/// Return with a custom error-handler.
	/// The arguments given to the handler are the name of the profile and the error that was thrown
	pub fn with_error_handler<ErrorHandler:Fn(&str, Box<dyn Error>) + Send + Sync + 'static>(mut self, error_handler:ErrorHandler) -> Self {
		self.set_error_handler(error_handler);
		self
	}

	/// Set the error handler.
	/// The arguments given to the handler are the name of the profile and the error that was thrown.
	pub fn set_error_handler<ErrorHandler:Fn(&str, Box<dyn Error>) + Send + Sync + 'static>(&mut self, error_handler:ErrorHandler) {
		self.error_handler = Arc::new(error_handler);
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

	/// Get a remote control to the system.
	/// Allows triggering events and making changes to the system from somewhere else.
	pub fn create_remote(&self) -> WindowRelativeSystemRemoteControl {
		WindowRelativeSystemRemoteControl(self.modifications_queue.create_remote())
	}

	/// Run the system.
	/// Listens to window-change events indefinitely.
	pub fn run(&mut self) {

		// Launch window-hook if it does not exist yet.
		if !self.hook_remote_registered {
			window_hook::register_remote(self.create_remote());
			self.hook_remote_registered = true;
		}

		// Keep running the modifications from the queue indefinitely.
		loop {
			for modification in self.modifications_queue.await_change() {
				modification(self);
			}
		}
	}

	/// Set a specific window as active.
	/// Will activate the according window-relative profile.
	pub fn set_active_window(&mut self, previous_window:&Option<WindowController>, current_window:&WindowController) {
		let error_handler:Arc<dyn Fn(&str, Box<dyn Error + 'static>) + Send + Sync> = Arc::clone(&self.error_handler);
		
		// Find the active profile index.
		let active_process_name:String = current_window.process_name().unwrap_or_default();
		let active_process_title:String = current_window.title();
		let mut next_active_profile_index:Option<usize> = None;
		for (profile_index, profile) in self.profiles.iter().enumerate() {
			if profile.matches_window(current_window, &active_process_name, &active_process_title) {
				next_active_profile_index = Some(profile_index);
				break;
			}
		}
		if next_active_profile_index == self.active_profile_index {
			return;
		}

		// Handle previous profile deactivation.
		// This code is a bit messy, but makes sure the events and errors are handled when they occur.
		if let Some(previous_window) = previous_window {
			let previous_profile:&mut dyn WindowRelativeProfile = self.profile_with_index_mut(self.active_profile_index);
			if let Err(error) = previous_profile.on_deactivate() {
				error_handler(previous_profile.name(), error);
			}
			if let Err(error) = previous_profile.on_event(previous_window, "deactivate") {
				error_handler(previous_profile.name(), error);
			}
			//previous_profile.task_system_mut().stop();
			*previous_profile.status_mut() = ProfileStatus::Deactivated;
		}

		// Handle switch to new profile.
		self.active_profile_index = next_active_profile_index;

		// Handle new profile activation.
		// This code is a bit messy, but makes sure the events and errors are handled when they occur.
		let new_profile:&mut dyn WindowRelativeProfile = self.profile_with_index_mut(self.active_profile_index);
		if new_profile.status() == &ProfileStatus::Uninitialized {
			if let Err(error) = new_profile.on_open() {
				error_handler(new_profile.name(), error);
			}
			if let Err(error) = new_profile.on_event(current_window, "open") {
				error_handler(new_profile.name(), error);
			}
		}
		*new_profile.status_mut() = ProfileStatus::Active;
		if let Err(error) = new_profile.on_activate() {
			error_handler(new_profile.name(), error);
		}
		if let Err(error) = new_profile.on_event(current_window, "activate") {
			error_handler(new_profile.name(), error);
		}
		//new_profile.task_system_mut().start();
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

	/// Execute an event on the active profile.
	pub fn trigger_event(&mut self, event_name:&str) {
		let error_handler:Arc<dyn Fn(&str, Box<dyn Error + 'static>) + Send + Sync> = Arc::clone(&self.error_handler);
		let current_profile:&mut (dyn WindowRelativeProfile + 'static) = self.profile_with_index_mut(self.active_profile_index);
		if let Err(error) = current_profile.execute_event(&WindowController::active(), event_name) {
			error_handler(current_profile.name(), error);
		}
	}

	/// Execute an action on all profiles.
	/// Includes the default profile.
	pub fn execute_on_all_profiles<Action:Fn(&mut dyn WindowRelativeProfile) -> ReturnType, ReturnType>(&mut self, action:Action) -> Vec<ReturnType> {
		vec![
			vec![action(&mut *self.default_profile)],
			self.profiles.iter_mut().map(|profile| action(&mut **profile)).collect()
		].into_iter().flatten().collect()
	}

	/// Execute an action on the currently activated profile.
	/// Uses the default profile if no profile is active.
	pub fn execute_on_current_profile<Action:FnOnce(&mut dyn WindowRelativeProfile) -> ReturnType, ReturnType>(&mut self, action:Action) -> ReturnType {
		action(self.profile_with_index_mut(self.active_profile_index))
	}

	/// Execute an action on the default profile.
	pub fn execute_on_default_profile<Action:FnOnce(&mut dyn WindowRelativeProfile) -> ReturnType, ReturnType>(&mut self, action:Action) -> ReturnType {
		action(&mut *self.default_profile)
	}

	/// Execute an action on the profile with the given name.
	/// Does nothing if the profile does not exist.
	pub fn execute_on_profile_with_name<Action:FnOnce(&mut dyn WindowRelativeProfile) -> ReturnType, ReturnType>(&mut self, name:&str, action:Action) -> Option<ReturnType> {
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



pub struct WindowRelativeSystemRemoteControl(ModificationsQueueRemote<WindowRelativeSystem>);
impl WindowRelativeSystemRemoteControl {

	/* STATE CHANGING METHODS */

	/// Handle a window-change.
	pub fn handle_window_change(&self, previous_window:&Option<WindowController>, current_window:&WindowController) {

		// Using raw pointers for a windows that might not exist anymore is safe, but unreliable.
		// Most functions the now invalid HWND__ is used for will likely be able to handle invalid HWND__'s.
		let previous_window_pointer:Option<u64> = previous_window.as_ref().map(|window| window.hwnd() as u64);
		let current_window_pointer:u64 = current_window.hwnd() as u64;
		self.0.add(move |system| {
			system.set_active_window(
				&previous_window_pointer.map(|pointer| WindowController::from_hwnd(pointer as *mut _)),
				&WindowController::from_hwnd(current_window_pointer as *mut _)
			);
		});
	}



	/* EXECUTION METHODS */

	/// Execute an event on the active profile.
	pub fn trigger_event(&self, event_name:&str) {
		let event_name:String = event_name.to_string();
		self.0.add(move |system| {
			system.trigger_event(&event_name);
		});
	}

	/// Execute an action on all profiles.
	/// Includes the default profile.
	pub fn execute_on_all_profiles<Action:Fn(&mut dyn WindowRelativeProfile) + Send + Sync + 'static>(&self, action:Action) {
		self.0.add(move |system| {
			system.execute_on_all_profiles(action);
		});
	}

	/// Execute an action on the currently activated profile.
	/// Uses the default profile if no profile is active.
	pub fn execute_on_current_profile<Action:FnOnce(&mut dyn WindowRelativeProfile) + Send + Sync + 'static>(&self, action:Action) {
		self.0.add(move |system| {
			action(system.profile_with_index_mut(system.active_profile_index));
		});
	}

	/// Execute an action on the default profile.
	pub fn execute_on_default_profile<Action:FnOnce(&mut dyn WindowRelativeProfile) + Send + Sync + 'static>(&self, action:Action) {
		self.0.add(move |system| {
			action(&mut *system.default_profile);
		});
	}

	/// Execute an action on the profile with the given name.
	/// Does nothing if the profile does not exist.
	pub fn execute_on_profile_with_name<Action:Fn(&mut dyn WindowRelativeProfile) + Send + Sync + 'static>(&self, name:&str, action:Action){
		let name:String = name.to_string();
		self.0.add(move |system| {
			if name == system.default_profile.name() {
				action(&mut *system.default_profile);
			} else {
				for profile in &mut system.profiles {
					if profile.name() == name {
						action(&mut **profile);
					}
				}
			}
		});
	}
}