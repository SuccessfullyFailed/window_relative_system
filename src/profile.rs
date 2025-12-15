use std::{ error::Error, sync::Arc, time::Instant };
use window_controller::WindowController;
use crate::{WindowRelativeProfileData, WindowRelativeProfileDataConvertible, WindowRelativeProfileService, WindowRelativeProfileServiceSet};
use task_syncer::TaskSystem;



pub struct WindowRelativeProfile {
	id:String,
	title:String,
	process_name:String,
	is_default_profile:bool,

	task_system:TaskSystem,
	active_checker:Arc<dyn Fn(&WindowRelativeProfile, &WindowController, &str, &str) -> bool + Send + Sync>,
	services:WindowRelativeProfileServiceSet,
	event_handlers:Vec<Arc<dyn Fn(&mut WindowRelativeProfile, &WindowController, &str) -> Result<(), Box<dyn Error>> + Send + Sync>>,
	data:WindowRelativeProfileData,
	
	is_opened:bool,
	is_active:bool,
	is_recursing_scheduler_events:bool
}
impl WindowRelativeProfile {

	/// Create a new profile.
	pub fn new(id:&str, title:&str, process_name:&str) -> WindowRelativeProfile {
		let mut task_system:TaskSystem = TaskSystem::new();
		task_system.pause(&Instant::now());
		
		WindowRelativeProfile {
			id: id.to_string(),
			title: title.to_string(),
			process_name: process_name.to_string(),
			is_default_profile: false,

			task_system,
			active_checker: Arc::new(|profile, _active_window, active_process_name, _active_process_title| active_process_name == profile.process_name),
			services: WindowRelativeProfileServiceSet::new(),
			event_handlers: Vec::new(),
			data: WindowRelativeProfileData::new(),

			is_opened: false,
			is_active: false,
			is_recursing_scheduler_events: false
		}
	}

	/// Create the default-profile. There should always only be one, so do not make it accessible outside of this crate.
	pub(crate) fn default_profile() -> WindowRelativeProfile {
		WindowRelativeProfile {
			is_default_profile: true,
			..WindowRelativeProfile::new("DEFAULT_PROFILE_ID", "DEFAULT_PROFILE_TITLE", "DEFAULT_PROFILE_PROCESS_NAME")
		}
	}



	/* PROPERTY SETTER METHODS */

	/// Return self with the given modification applied.
	pub fn with_modifications<Modification:Fn(WindowRelativeProfile) -> WindowRelativeProfile>(mut self, modifications:Vec<Modification>) -> Self {
		for modification in modifications {
			self = self.with_modification(modification);
		}
		self
	}

	/// Return self with the given modification applied.
	pub fn with_modification<Modification:Fn(WindowRelativeProfile) -> WindowRelativeProfile>(self, modification:Modification) -> Self {
		modification(self)
	}

	/// Return self with a new active checker.
	pub fn with_active_checker<ActiveChecker:Fn(&WindowRelativeProfile, &WindowController, &str, &str) -> bool + Send + Sync + 'static>(mut self, active_checker:ActiveChecker) -> Self {
		self.set_active_checker(active_checker);
		self
	}

	/// Set the active checker of the profile.
	pub fn set_active_checker<ActiveChecker:Fn(&WindowRelativeProfile, &WindowController, &str, &str) -> bool + Send + Sync + 'static>(&mut self, active_checker:ActiveChecker) {
		self.active_checker = Arc::new(active_checker);
	}

	/// Return self with an added service.
	pub fn with_service<Service:WindowRelativeProfileService + 'static>(mut self, service:Service) -> Self {
		self.add_service(service);
		self
	}

	/// Add a new service to the profile.
	pub fn add_service<Service:WindowRelativeProfileService + 'static>(&mut self, service:Service) {
		self.services.push(service);
	}

	/// Return self with an added handler.
	pub fn with_handler<Handler:Fn(&mut WindowRelativeProfile, &WindowController, &str) -> Result<(), Box<dyn Error>> + Send + Sync + 'static>(mut self, handler:Handler) -> Self {
		self.add_handler(handler);
		self
	}

	/// Add a new handler to the profile.
	pub fn add_handler<Handler:Fn(&mut WindowRelativeProfile, &WindowController, &str) -> Result<(), Box<dyn Error>> + Send + Sync + 'static>(&mut self, handler:Handler) {
		self.event_handlers.push(Arc::new(handler));
	}

	/// Get some data from the profile.
	pub fn get_data<T:WindowRelativeProfileDataConvertible>(&self, name:&str) -> Option<T> {
		self.data.get(name)
	}

	/// Set some data from the profile.
	pub fn set_data<T:WindowRelativeProfileDataConvertible>(&mut self, name:&str, value:T) {
		self.data.set(name, value)
	}

	/// Get a handle to the services list.
	#[cfg(test)]
	pub(crate) fn services(&self) -> &WindowRelativeProfileServiceSet {
		&self.services
	}





	/* PROPERTY GETTER METHODS */

	/// Get the id of the profile.
	pub fn id(&self) -> &str {
		&self.id
	}

	/// Get the title of the profile.
	pub fn title(&self) -> &str {
		&self.title
	}

	/// Get the process_name of the profile.
	pub fn process_name(&self) -> &str {
		&self.process_name
	}

	/// Whether or not this is the default profile.
	pub fn is_default_profile(&self) -> bool {
		self.is_default_profile
	}

	/// Whether or not this profile is the active one.
	pub fn is_active(&self, active_window:&WindowController, active_process_name:&str, active_process_title:&str) -> bool {
		(self.active_checker.clone())(self, active_window, active_process_name, active_process_title)
	}

	/// Get a handle to the task system.
	pub fn task_system(&self) -> &TaskSystem {
		&self.task_system
	}

	/// Get a mutable handle to the task system.
	pub fn task_system_mut(&mut self) -> &mut TaskSystem {
		&mut self.task_system
	}



	/* EVENT METHODS */

	/// Trigger an event.
	pub fn trigger_event(&mut self,  window:&WindowController, event_name:&str) -> Result<(), Box<dyn Error>> {

		// Handle pending task-system events profile-wide.
		if !self.is_recursing_scheduler_events {
			let task_system_events:Vec<String> = self.task_system.task_scheduler().pending_event_names();
			if !task_system_events.is_empty() {
				self.is_recursing_scheduler_events = true;
				for scheduler_event_name in task_system_events {
					self.trigger_event(window, &scheduler_event_name)?;
				}
				self.is_recursing_scheduler_events = false;
			}
		}

		// Handle built-in event handlers for active profile.
		match event_name {
			"open" => {
				self.is_opened = true;
			},
			"activate" => {
				if !self.is_opened {
					self.trigger_event(window, "open")?;
				}
				self.is_active = true;

				let now:Instant = Instant::now();
				self.task_system.resume(&now);
				self.task_system.run_once(&now);
			},
			"update" => {
				self.task_system.run_once(&Instant::now());
			},
			_ => {}
		}

		// Run services and handlers.
		self.services.run(self.task_system.task_scheduler(), window, event_name)?;
		for handler in self.event_handlers.clone() {
			handler(self, window, event_name)?;
		}

		// Handle built-in event handlers for inactive profile.
		match event_name {
			"deactivate" => {
				let now:Instant = Instant::now();
				self.task_system.run_once(&now);
				self.task_system.pause(&now);

				self.is_active = false;
				if !window.exists() || !window.is_visible() {
					self.trigger_event(window, "close")?;
				}
			},
			"close" => {
				self.is_opened = false;
			},
			_ => {}
		}

		// Return success.
		Ok(())
	}
}