use super::ProfileProperties;



pub trait WindowRelativeProfile:Send {

	/* PROFILE PROPERTY METHODS */

	/// Get the profile properties.
	fn properties(&self) -> &ProfileProperties;

	/// Get the profile properties mutable.
	fn properties_mut(&mut self) -> &mut ProfileProperties;
	
	/// Check if this profile fits the current process.
	fn is_active(&self, active_process_name:&str, _active_process_title:&str) -> bool {
		self.properties().process_name == active_process_name
	}

	

	/* STATUS UPDATE HANDLERS */

	/// Handler for when the profile is created.
	fn on_create(&mut self) {
	}

	/// Handler for when the profile activates.
	fn on_activate(&mut self) {
	}

	/// Handler for when the profile deactivates.
	fn on_deactivate(&mut self) {
	}

	/// Handler for when the window is initially opened.
	fn on_open(&mut self) {
	}

	/// Handler for then the window is closed.
	fn on_exit(&mut self) {
	}
}