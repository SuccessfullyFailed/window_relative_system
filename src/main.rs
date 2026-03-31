use window_controller::WindowController;
use window_relative_system::{WindowRelativeProfileCore, WindowRelativeSystem};

pub fn main() {
	let mut system:WindowRelativeSystem = WindowRelativeSystem::new(WindowRelativeProfileCore::new("default_profile", "default_process_name"));
	system.add_profile(WindowRelativeProfileCore::new("active_process", &WindowController::active().process_name().unwrap_or_default()));
	system.run();
}