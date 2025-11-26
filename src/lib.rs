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
use crate as window_relative_system;
#[cfg(test)]
#[window_relative_profile_creator_macro::window_relative_profile(TEST_PROFILE_ID, TEST_PROFILE_TITLE, TEST_PROFILE_PROCESS_NAME)]
pub(crate) struct TestCore {}
#[cfg(test)]
impl TestCore {
	const ID:&str = "TEST_PROFILE_ID";
	const TITLE:&str = "TEST_PROFILE_TITLE";
	const PROCESS_NAME:&str = "TEST_PROFILE_PROCESS_NAME";
}
#[cfg(test)]
impl WindowRelativeProfile for TestCore {}