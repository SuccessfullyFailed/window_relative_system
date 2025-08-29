use task_syncer::{ Task, TaskSystem };
use std::error::Error;



type WindowRelativeEventResponse = Result<(), Box<dyn Error>>;
type WindowRelativeEventHandler = dyn Fn(&mut WindowRelativeProfileProperties) -> WindowRelativeEventResponse + Send;
type WindowRelativeEventHandlers = Vec<Box<WindowRelativeEventHandler>>;
type NamedOperationReturnType = Result<(), Box<dyn Error>>;



pub struct WindowRelativeProfile {
	properties: WindowRelativeProfileProperties,
	event_handlers:WindowRelativeProfileEventHandlers,
	pub(crate) task_system:TaskSystem,
	pub(crate) named_operations:Vec<(String, Box<dyn Fn() -> NamedOperationReturnType + Send>)>
}
pub struct WindowRelativeProfileProperties {
	id:String,
	title:String,
	process_name:String,
	is_default_profile:bool,
	
	active_checker:Box<dyn Fn(&WindowRelativeProfileProperties, &str, &str) -> bool + Send>,
	is_opened:bool,
	is_active:bool
}
pub struct WindowRelativeProfileEventHandlers {
	on_open:WindowRelativeEventHandlers,
	on_activate:WindowRelativeEventHandlers,
	on_deactivate:WindowRelativeEventHandlers
}
impl WindowRelativeProfile {

	/* CONSTRUCTOR METHODS */

	/// Create a new profile.
	pub fn new(id:&str, title:&str, process_name:&str) -> WindowRelativeProfile {
		let mut task_system:TaskSystem = TaskSystem::new();
		task_system.pause();
		WindowRelativeProfile {
			properties: WindowRelativeProfileProperties {
				id: id.to_string(),
				title: title.to_string(),
				process_name: process_name.to_string(),
				is_default_profile: false,

				active_checker: Box::new(|_self, active_process_name, _active_process_title| active_process_name == _self.process_name),
				is_opened: false,
				is_active: false
			},
			event_handlers: WindowRelativeProfileEventHandlers {
				on_open: Vec::new(),
				on_activate: Vec::new(),
				on_deactivate: Vec::new()
			},
			task_system,
			named_operations: Vec::new()
		}
	}

	/// Add a small tag indicating that this is the default profile and not linked to any actual process.
	pub fn with_is_default_profile(mut self) -> Self {
		self.properties.is_default_profile = true;
		self
	}

	/// Replace the active checker and return self.
	pub fn with_active_checker<T>(mut self, active_checker:T) -> Self where T:Fn(&WindowRelativeProfileProperties, &str, &str) -> bool + Send + 'static {
		self.properties.active_checker = Box::new(active_checker);
		self
	}

	/// Return self with an additional profile open event handler.
	pub fn with_open_handler<T>(mut self, handler:T) -> Self where T:Fn(&mut WindowRelativeProfileProperties) -> WindowRelativeEventResponse + Send + 'static {
		self.event_handlers.on_open.push(Box::new(handler));
		self
	}

	/// Return self with an additional profile activate event handler.
	pub fn with_activate_handler<T>(mut self, handler:T) -> Self where T:Fn(&mut WindowRelativeProfileProperties) -> WindowRelativeEventResponse + Send + 'static {
		self.event_handlers.on_activate.push(Box::new(handler));
		self
	}

	/// Return self with an additional profile deactivate event handler.
	pub fn with_deactivate_handler<T>(mut self, handler:T) -> Self where T:Fn(&mut WindowRelativeProfileProperties) -> WindowRelativeEventResponse + Send + 'static {
		self.event_handlers.on_deactivate.push(Box::new(handler));
		self
	}

	/// Return self with an additional task-syncer task.
	pub fn with_task(mut self, task:Task) -> Self {
		self.task_system.add_task(task);
		self
	}

	/// Return self with an additional named operation.
	pub fn with_named_operation<T>(mut self, name:&str, operation:T) -> Self where T:Fn() -> NamedOperationReturnType + Send + 'static {
		self.add_named_operation(name, operation);
		self
	}



	/* PROPERTY GETTER METHODS */

	/// Get the ID of the profile.
	pub fn id(&self) -> &str {
		&self.properties.id
	}

	/// Get the title of the profile.
	pub fn title(&self) -> &str {
		&self.properties.title
	}

	/// Get the process-name of the profile.
	pub fn process_name(&self) -> &str {
		&self.properties.process_name
	}

	/// Whether or not this is the default profile.
	pub fn is_default_profile(&self) -> bool {
		self.properties.is_default_profile
	}



	/* USAGE METHODS */

	/// Check if this is the active profile.
	pub fn is_active(&self, active_process_name:&str, active_process_title:&str) -> bool {
		(self.properties.active_checker)(&self.properties, active_process_name, active_process_title)
	}

	/// Add a task to the task-system.
	pub fn schedule_task(&mut self, task:Task) {
		self.task_system.add_task(task);
	}

	/// Add a named operation.
	pub fn add_named_operation<T>(&mut self, name:&str, operation:T) where T:Fn() -> NamedOperationReturnType + Send + 'static {
		self.named_operations.push((name.to_string(), Box::new(operation)));
	}

	/// Execute an operation by its name.
	pub fn execute_named_operation(&mut self, name:&str) -> Option<NamedOperationReturnType> {
		self.named_operations.iter().find(|(operation_name, _)| name == operation_name).map(|(_, operation)| operation())
	}



	/* EVENT HANDLER METHODS */

	/// The profile was opened.
	pub(crate) fn trigger_open_event(&mut self) -> WindowRelativeEventResponse {
		self.properties.is_opened = true;
		for handler in &self.event_handlers.on_open {
			handler(&mut self.properties)?;
		}
		Ok(())
	}

	/// The profile was activated.
	pub(crate) fn trigger_activate_event(&mut self) -> WindowRelativeEventResponse {
		if !self.properties.is_opened {
			self.trigger_open_event()?;
		}
		self.properties.is_active = true;
		self.task_system.resume();
		for handler in &self.event_handlers.on_activate {
			handler(&mut self.properties)?;
		}
		Ok(())
	}

	/// The profile was deactivated.
	pub(crate) fn trigger_deactivate_event(&mut self) -> WindowRelativeEventResponse {
		self.properties.is_active = false;
		self.task_system.pause();
		for handler in &self.event_handlers.on_deactivate {
			handler(&mut self.properties)?;
		}
		Ok(())
	}
}