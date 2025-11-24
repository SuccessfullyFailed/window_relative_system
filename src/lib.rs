mod system;
mod system_u;
mod profile;
mod profile_u;
mod profile_service;
mod profile_service_u;
mod window_hook;
mod window_hook_u;

pub use system::*;
pub use profile::*;
pub use profile_service::*;

pub use window_controller::WindowController;
pub use task_syncer::*;



#[cfg(test)]
pub(crate) struct TestCore {
	properties:WindowRelativeProfileProperties,
	task_system:TaskSystem
}
#[cfg(test)]
impl WindowRelativeProfile for TestCore {
	fn properties(&self) -> &WindowRelativeProfileProperties { &self.properties }
	fn properties_mut(&mut self) -> &mut WindowRelativeProfileProperties { &mut self.properties }
	fn task_system(&mut self) -> &TaskSystem { &self.task_system }
	fn task_system_mut(&mut self) -> &mut TaskSystem { &mut self.task_system }
	fn is_active(&self, _window:&WindowController, _active_process_name:&str, _active_process_title:&str) -> bool {
		false
	}
}
#[cfg(test)]
impl Default for TestCore {
	fn default() -> Self {
		TestCore {
			properties: WindowRelativeProfileProperties::new("DEFAULT_PROFILE_ID", "DEFAULT_PROFILE_TITLE", "DEFAULT_PROFILE_PROCESS_NAME").with_is_default(),
			task_system: TaskSystem::new()
		}
	}
}