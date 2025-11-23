use task_syncer::{ TaskLike, TaskScheduler, TaskSystem };
use window_controller::WindowController;
use std::{ error::Error, time::Instant };
use crate::{WindowRelativeProfileService, WindowRelativeServiceTrigger};



type EventHandlerResponse = Result<(), Box<dyn Error>>;
type EventHandler = dyn Fn(&mut WindowRelativeProfileProperties, &TaskScheduler, &WindowController) -> EventHandlerResponse + Send + Sync;
type EventHandlerList = Vec<Box<EventHandler>>;



pub trait WindowRelativeProfile:Send + Sync + 'static {

	/// Get the core of the profile.
	fn core(&self) -> &WindowRelativeProfileCore;

	/// Get the core of the profile mutable.
	fn core_mut(&mut self) -> &mut WindowRelativeProfileCore;

	/// Gets ran with the event name whenever an event is triggered.
	fn on_event(&mut self, _event_name:&str) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	/// Gets ran whenever the profile is updated once.
	fn on_update(&mut self) {
	}



	/* PROPERTY GETTER METHODS */

	/// Get the properties of the profile.
	fn properties(&self) -> &WindowRelativeProfileProperties {
		self.core().properties()
	}

	/// Get the ID of the profile.
	fn id(&self) -> &str {
		self.core().id()
	}

	/// Get the title of the profile.
	fn title(&self) -> &str {
		self.core().title()
	}

	/// Get the process-name of the profile.
	fn process_name(&self) -> &str {
		self.core().process_name()
	}

	/// Whether or not this is the default profile.
	fn is_default_profile(&self) -> bool {
		self.core().is_default_profile()
	}

	/// Get the task system.
	fn task_system(&self) -> &TaskSystem {
		self.core().task_system()
	}

	/// Get the task system mutable.
	fn task_system_mut(&mut self) -> &mut TaskSystem {
		self.core_mut().task_system_mut()
	}



	/* USAGE METHODS */

	/// Run the profile events once.
	fn run_once(&mut self, now:&Instant) {
		self.task_system_mut().run_once(now);
		for service in &mut self.core_mut().services {
			service.update();
		}
		self.on_update();
	}

	/// Trigger an event in the task-system.
	fn trigger_event(&mut self, event_name:&str) -> Result<(), Box<dyn Error>> {
		self.trigger_event_with_window(event_name, &WindowController::active())
	}

	/// Trigger an event in the task-system using the given window.
	fn trigger_event_with_window(&mut self, event_name:&str, window:&WindowController) -> Result<(), Box<dyn Error>> {
		self.on_event(event_name)?;
		self.core_mut().run_services_by_trigger(&window, WindowRelativeServiceTrigger::NamedEvent(event_name.to_string()))?;
		self.task_system_mut().trigger_event(event_name);
		Ok(())
	}

	/// Check if this is the active profile.
	fn is_active(&self, window:&WindowController, active_process_name:&str, active_process_title:&str) -> bool {
		(self.core().properties.active_checker)(&self.core().properties, &window, active_process_name, active_process_title)
	}
}


pub trait WindowRelativeProfileModifiers:WindowRelativeProfile {

	/// Set the active checker function.
	fn set_active_checker<T>(&mut self, active_checker:T) where T:Fn(&WindowRelativeProfileProperties, &WindowController, &str, &str) -> bool + Send + Sync + 'static {
		self.core_mut().properties.active_checker = Box::new(active_checker);
	}

	/// Add an additional profile open event handler.
	fn add_open_handler<T>(&mut self, handler:T) where T:Fn(&mut WindowRelativeProfileProperties, &TaskScheduler, &WindowController) -> EventHandlerResponse + Send + Sync + 'static {
		self.core_mut().event_handlers.on_open.push(Box::new(handler));
	}

	/// Add an additional profile activate event handler.
	fn add_activate_handler<T>(&mut self, handler:T) where T:Fn(&mut WindowRelativeProfileProperties, &TaskScheduler, &WindowController) -> EventHandlerResponse + Send + Sync + 'static {
		self.core_mut().event_handlers.on_activate.push(Box::new(handler));
	}

	/// Add an additional profile deactivate event handler.
	fn add_deactivate_handler<T>(&mut self, handler:T) where T:Fn(&mut WindowRelativeProfileProperties, &TaskScheduler, &WindowController) -> EventHandlerResponse + Send + Sync + 'static {
		self.core_mut().event_handlers.on_deactivate.push(Box::new(handler));
	}

	/// Add an additional profile close event handler.
	fn add_close_handler<T>(&mut self, handler:T) where T:Fn(&mut WindowRelativeProfileProperties, &TaskScheduler, &WindowController) -> EventHandlerResponse + Send + Sync + 'static {
		self.core_mut().event_handlers.on_close.push(Box::new(handler));
	}

	/// Add a task to the task-system.
	fn add_task<T:TaskLike + Send + Sync + 'static>(&mut self, task:T) {
		self.core_mut().task_system.add_task(task);
	}

	/// Apply a service to the profile.
	fn add_service<T:WindowRelativeProfileService + Send + Sync + 'static>(&mut self, mut service:T) {
		service.install(&self.core().properties, self.task_system().task_scheduler());
		self.core_mut().services.push(Box::new(service));
	}

	/// Add a service that overrides all existing ones with the same name.
	fn add_service_override<T:WindowRelativeProfileService + Send + Sync + 'static>(&mut self, service:T) {
		self.core_mut().services.retain(|existing_service| existing_service.name() != service.name());
		self.add_service(service);
	}

	/// Apply a service to the profile.
	fn add_services<T:WindowRelativeProfileService + Send + Sync + 'static>(&mut self, services:Vec<T>) {
		for service in services {
			self.add_service(service);
		}
	}
}
impl<T:WindowRelativeProfile + ?Sized> WindowRelativeProfileModifiers for T {}



pub struct WindowRelativeProfileCore {
	properties:WindowRelativeProfileProperties,
	event_handlers:WindowRelativeProfileEventHandlers,
	services:Vec<Box<dyn WindowRelativeProfileService>>,
	task_system:TaskSystem
}
impl WindowRelativeProfileCore {

	/* CONSTRUCTOR METHODS */

	/// Create a new profile.
	pub fn new(id:&str, title:&str, process_name:&str) -> WindowRelativeProfileCore {
		let mut task_system:TaskSystem = TaskSystem::new();
		task_system.pause();
		WindowRelativeProfileCore {
			properties: WindowRelativeProfileProperties {
				id: id.to_string(),
				title: title.to_string(),
				process_name: process_name.to_string(),
				is_default_profile: false,

				active_checker: Box::new(|_self, _window, active_process_name, _active_process_title| active_process_name == _self.process_name),
				is_opened: false,
				is_active: false
			},
			event_handlers: WindowRelativeProfileEventHandlers {
				on_open: Vec::new(),
				on_activate: Vec::new(),
				on_deactivate: Vec::new(),
				on_close: Vec::new()
			},
			services: Vec::new(),
			task_system
		}
	}

	/// Add a small tag indicating that this is the default profile and not linked to any actual process.
	pub fn with_is_default_profile(mut self) -> Self {
		self.properties.is_default_profile = true;
		self
	}

	/// Replace the active checker and return self.
	pub fn with_active_checker<T>(mut self, active_checker:T) -> Self where T:Fn(&WindowRelativeProfileProperties, &WindowController, &str, &str) -> bool + Send + Sync + 'static {
		self.set_active_checker(active_checker);
		self
	}

	/// Return self with an additional profile open event handler.
	pub fn with_open_handler<T>(mut self, handler:T) -> Self where T:Fn(&mut WindowRelativeProfileProperties, &TaskScheduler, &WindowController) -> EventHandlerResponse + Send + Sync + 'static {
		self.add_open_handler(handler);
		self
	}

	/// Return self with an additional profile activate event handler.
	pub fn with_activate_handler<T>(mut self, handler:T) -> Self where T:Fn(&mut WindowRelativeProfileProperties, &TaskScheduler, &WindowController) -> EventHandlerResponse + Send + Sync + 'static {
		self.add_activate_handler(handler);
		self
	}

	/// Return self with an additional profile deactivate event handler.
	pub fn with_deactivate_handler<T>(mut self, handler:T) -> Self where T:Fn(&mut WindowRelativeProfileProperties, &TaskScheduler, &WindowController) -> EventHandlerResponse + Send + Sync + 'static {
		self.add_deactivate_handler(handler);
		self
	}

	/// Return self with an additional profile close event handler.
	pub fn with_close_handler<T>(mut self, handler:T) -> Self where T:Fn(&mut WindowRelativeProfileProperties, &TaskScheduler, &WindowController) -> EventHandlerResponse + Send + Sync + 'static {
		self.add_close_handler(handler);
		self
	}

	/// Return self with an additional task-syncer task.
	pub fn with_task<T:TaskLike + Send + Sync + 'static>(mut self, task:T) -> Self {
		self.add_task(task);
		self
	}

	/// Return self with an applied service.
	pub fn with_service<T:WindowRelativeProfileService + 'static>(mut self, service:T) -> Self {
		self.add_service(service);
		self
	}

	/// Return self with a service that overrides all existing ones with the same name.
	pub fn with_service_override<T:WindowRelativeProfileService + 'static>(mut self, service:T) -> Self {
		self.add_service_override(service);
		self
	}



	/* PROPERTY GETTER METHODS */

	/// Get the properties of this profile.
	pub fn properties(&self) -> &WindowRelativeProfileProperties {
		&self.properties
	}

	/// Get the ID of the profile.
	pub fn id(&self) -> &str {
		self.properties.id()
	}

	/// Get the title of the profile.
	pub fn title(&self) -> &str {
		self.properties.title()
	}

	/// Get the process-name of the profile.
	pub fn process_name(&self) -> &str {
		self.properties.process_name()
	}

	/// Whether or not this is the default profile.
	pub fn is_default_profile(&self) -> bool {
		self.properties.is_default_profile()
	}

	/// Get the task system.
	pub fn task_system(&self) -> &TaskSystem {
		&self.task_system
	}

	/// Get the task system mutable.
	pub fn task_system_mut(&mut self) -> &mut TaskSystem {
		&mut self.task_system
	}



	/* EVENT HANDLER METHODS */

	/// The profile was opened.
	pub(crate) fn trigger_open_event(&mut self, new_window:&WindowController) -> EventHandlerResponse {
		self.properties.is_opened = true;
		for handler in &self.event_handlers.on_open {
			handler(&mut self.properties, self.task_system.task_scheduler(), new_window)?;
		}
		self.run_services_by_trigger(new_window, WindowRelativeServiceTrigger::Open)?;
		Ok(())
	}

	/// The profile was activated.
	pub(crate) fn trigger_activate_event(&mut self, new_window:&WindowController) -> EventHandlerResponse {
		if !self.properties.is_opened {
			self.trigger_open_event(new_window)?;
		}
		self.properties.is_active = true;
		self.task_system.resume();
		for handler in &self.event_handlers.on_activate {
			handler(&mut self.properties, self.task_system.task_scheduler(), new_window)?;
		}
		self.run_services_by_trigger(new_window, WindowRelativeServiceTrigger::Activate)?;
		self.task_system.run_once(&Instant::now());
		Ok(())
	}

	/// The profile was deactivated.
	pub(crate) fn trigger_deactivate_event(&mut self, deactivated_window:&WindowController) -> EventHandlerResponse {
		self.properties.is_active = false;
		for handler in &self.event_handlers.on_deactivate {
			handler(&mut self.properties, self.task_system.task_scheduler(), deactivated_window)?;
		}
		self.run_services_by_trigger(deactivated_window, WindowRelativeServiceTrigger::Deactivate)?;
		if !deactivated_window.is_visible() {
			self.trigger_close_event(deactivated_window)?;
		}
		self.task_system.run_once(&Instant::now());
		self.task_system.pause();
		Ok(())
	}

	/// The profile was closed.
	pub(crate) fn trigger_close_event(&mut self, deactivated_window:&WindowController) -> EventHandlerResponse {
		self.properties.is_opened = false;
		for handler in &self.event_handlers.on_close {
			handler(&mut self.properties, self.task_system.task_scheduler(), deactivated_window)?;
		}
		self.run_services_by_trigger(deactivated_window, WindowRelativeServiceTrigger::Close)?;
		Ok(())
	}

	/// Run all services that have a specific trigger.
	pub(crate) fn run_services_by_trigger(&mut self, window:&WindowController, trigger:WindowRelativeServiceTrigger) -> Result<(), Box<dyn Error>> {
		for service in &mut self.services {
			if service.when_to_trigger().run_on_trigger(&trigger) {
				service.run(&self.properties, self.task_system.task_scheduler(), window, trigger.clone())?;
			}
		}
		Ok(())
	}
}
impl WindowRelativeProfile for WindowRelativeProfileCore {
	fn core(&self) -> &WindowRelativeProfileCore {
		self
	}
	fn core_mut(&mut self) -> &mut WindowRelativeProfileCore {
		self
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



pub struct WindowRelativeProfileEventHandlers {
	on_open:EventHandlerList,
	on_activate:EventHandlerList,
	on_deactivate:EventHandlerList,
	on_close:EventHandlerList
}