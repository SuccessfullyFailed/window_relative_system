use crate::{ WindowRelativeProfileHandler, WindowRelativeProfileHandlerSet, WindowRelativeProfileService, WindowRelativeProfileServiceSet };
use window_relative_profile_creator_macro::window_relative_profile;
use std::{ any::Any, error::Error, time::Instant };
use window_controller::WindowController;
use task_syncer::TaskSystem;



pub trait WindowRelativeProfile:Send + Sync + 'static {
	fn properties(&self) -> &WindowRelativeProfileProperties;
	fn properties_mut(&mut self) -> &mut WindowRelativeProfileProperties;
	fn task_system(&mut self) -> &TaskSystem;
	fn task_system_mut(&mut self) -> &mut TaskSystem;
	fn services(&mut self) -> &mut WindowRelativeProfileServiceSet;
	fn as_any_mut(&mut self) -> &mut dyn Any;
	fn run_handlers(&mut self, _window:&WindowController, _event_name:&str) -> Result<(), Box<dyn Error>>;
	


	/* SUB-PROPERTY GETTER METHODS */

	/// Get the ID of the profile.
	fn id(&self) -> &str {
		self.properties().id()
	}

	/// Get the title of the profile.
	fn title(&self) -> &str {
		self.properties().title()
	}

	/// Get the process-name of the profile.
	fn process_name(&self) -> &str {
		self.properties().process_name()
	}

	/// Whether or not this is the default profile.
	fn is_default_profile(&self) -> bool {
		self.properties().is_default_profile()
	}

	/// Wether or not this is the active profile.
	fn is_active(&self, window:&WindowController, active_process_name:&str, active_process_title:&str) -> bool {
		(self.properties().active_checker)(self.properties(), window, active_process_name, active_process_title)
	}
	


	/* EVENT HANDLER METHODS */

	/// Trigger an event in the profile.
	fn trigger_event(&mut self, event_name:&str) -> Result<(), Box<dyn Error>> {
		self.trigger_event_with_window(event_name, &WindowController::active())
	}

	/// Trigger an event in the profile using the given window.
	fn trigger_event_with_window(&mut self, event_name:&str, window:&WindowController) -> Result<(), Box<dyn Error>> {

		// Handle manual event handlers.
		match event_name {
			"open" => {
				self.properties_mut().is_opened = true;
			},
			"activate" => {
				if !self.properties().is_opened {
					self.trigger_event_with_window("open", window)?;
				}
				self.properties_mut().is_active = true;
				self.task_system_mut().resume();
				self.task_system_mut().run_once(&Instant::now());
			},
			"update" => {
				self.task_system_mut().run_once(&Instant::now());
			},
			"deactivate" => {
				self.properties_mut().is_active = false;
				self.task_system_mut().pause();
			},
			"close" => {
				self.properties_mut().is_opened = false;
			},
			_ => {}
		};
		
		// Handle event in task-system.
		self.task_system_mut().trigger_event(event_name);

		// Handle events in services and handlers.
		self.run_handlers(window, event_name)?;
		self.services().run(window, event_name)?;

		// 'close' event should trigger after 'deactivate' is fully handled.
		if event_name == "deactivate" && !window.is_active() {
			self.trigger_event_with_window("close", window)?;
		}

		// Return success.
		Ok(())
	}
	


	/* MODIFICATION METHODS */
	
	/// Add a service to the list.
	fn add_service_literal(&mut self, service:Box<dyn WindowRelativeProfileService>) {
		self.services().add_service_literal(service);
	}
}



//pub type WindowRelativeProfileHandler<T> = Arc<dyn Fn(&mut T, &WindowController, &str) -> Result<(), Box<dyn Error>> + Send + Sync>;
pub trait WindowRelativeProfileSized:WindowRelativeProfile + Sized {
	fn handlers(&self) -> &WindowRelativeProfileHandlerSet<Self>;
	fn handlers_mut(&mut self) -> &mut WindowRelativeProfileHandlerSet<Self>;



	/* MODIFICATION METHODS */
	
	/// Return self with a new active checker.
	fn with_active_checker<T:Fn(&WindowRelativeProfileProperties, &WindowController, &str, &str) -> bool + Send + Sync + 'static>(mut self, checker:T) -> Self {
		self.properties_mut().active_checker = Box::new(checker);
		self
	}

	/// Return self with an added service.
	fn with_service<T:WindowRelativeProfileService + 'static>(mut self, service:T) -> Self {
		self.add_service(service);
		self
	}
	
	/// Return self with an added handler.
	fn with_handler<T:WindowRelativeProfileHandler<Self> + 'static>(mut self, handler:T) -> Self {
		self.add_handler(handler);
		self
	}
	
	/// Add a service to the list.
	fn add_service<T:WindowRelativeProfileService + 'static>(&mut self, service:T) {
		self.services().add_service(service);
	}

	/// Add a handler to the list.
	fn add_handler<T:WindowRelativeProfileHandler<Self> + 'static>(&mut self, handler:T) {
		self.handlers_mut().add_service(handler);
	}
}



pub struct WindowRelativeProfileProperties {
	id:String,
	title:String,
	process_name:String,
	is_default_profile:bool,
	
	active_checker:Box<dyn Fn(&WindowRelativeProfileProperties, &WindowController, &str, &str) -> bool + Send + Sync>,
	is_opened:bool,
	is_active:bool
}
impl WindowRelativeProfileProperties {

	/* CONSTRUCTOR METHODS */

	/// Create a new properties set.
	pub fn new(id:&str, title:&str, process_name:&str) -> WindowRelativeProfileProperties {
		WindowRelativeProfileProperties {
			id: id.to_string(),
			title: title.to_string(),
			process_name: process_name.to_string(),
			is_default_profile: false,

			active_checker: Box::new(|properties, _window, active_process_name, _active_process_title| active_process_name == properties.process_name()),
			is_opened: false,
			is_active: false
		}
	}

	/// Return self with the default profile flag set to true.
	pub fn with_is_default(mut self) -> Self {
		self.is_default_profile = true;
		self
	}

	/// Return self with a new active checker.
	pub fn with_active_checker<T:Fn(&WindowRelativeProfileProperties, &WindowController, &str, &str) -> bool + Send + Sync + 'static>(mut self, checker:T) -> Self {
		self.active_checker = Box::new(checker);
		self
	}



	/* PROPERTY GETTER METHODS */

	/// Get the ID of the profile.
	pub fn id(&self) -> &str {
		&self.id
	}

	/// Get the title of the profile.
	pub fn title(&self) -> &str {
		&self.title
	}

	/// Get the process-name of the profile.
	pub fn process_name(&self) -> &str {
		&self.process_name
	}

	/// Whether or not this is the default profile.
	pub fn is_default_profile(&self) -> bool {
		self.is_default_profile
	}
}



use crate as window_relative_system;
#[window_relative_profile]
pub(crate) struct WindowRelativeDefaultProfile {}
impl Default for WindowRelativeDefaultProfile {
	fn default() -> Self {
		WindowRelativeDefaultProfile {
			properties: WindowRelativeProfileProperties::new("DEFAULT_PROFILE_ID", "DEFAULT_PROFILE_TITLE", "DEFAULT_PROFILE_PROCESS_NAME").with_is_default(),
			task_system: TaskSystem::new(),
			services: WindowRelativeProfileServiceSet::new(),
			handlers: WindowRelativeProfileHandlerSet::new()
		}
	}
}