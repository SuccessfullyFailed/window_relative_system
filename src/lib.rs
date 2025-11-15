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
struct CoreWrapper(pub WindowRelativeProfileCore);
#[cfg(test)]
impl WindowRelativeProfile for CoreWrapper {
	fn core(&self) -> &WindowRelativeProfileCore { &self.0 }
	fn core_mut(&mut self) -> &mut WindowRelativeProfileCore { &mut self.0 }
}