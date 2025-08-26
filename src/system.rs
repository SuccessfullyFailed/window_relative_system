use std::{ error::Error, sync::{ Mutex, MutexGuard }, thread::sleep, time::{Duration, Instant} };
use crate::{ window_hook, WindowRelativeProfile };
use window_controller::WindowController;



static SYSTEM_INST:Mutex<Option<WindowRelativeSystem>> = Mutex::new(None);
pub(crate) const DEFAULT_ERROR_HANDLER:&dyn Fn(&WindowRelativeProfile, &str, &dyn Error) = &|profile, event_name, error| eprintln!("Profile {} panicked in {} event: {}", profile.id(), event_name, error);



pub struct WindowRelativeSystem {
	profiles:Vec<WindowRelativeProfile>,
	active_profile_index:usize,
	error_handler:Box<dyn Fn(&WindowRelativeProfile, &str, &dyn Error) + Send>,
	interval:Duration
}
impl WindowRelativeSystem {
	
	/* EVENT METHODS */

	/// Run the system. Installs the hook that triggers events in profiles.
	pub fn run() {
		window_hook::install(true);

		// Ensure system existence.
		Self::execute_on_system(|_| {});

		// Repeat indefinitely.
		loop {
			let mut sleep_time:Duration = Duration::from_millis(0);
			{
				let loop_start:Instant = Instant::now();

				// Get system lock.
				let mut system_guard:MutexGuard<'_, Option<WindowRelativeSystem>> = SYSTEM_INST.lock().unwrap();
				let system:&mut WindowRelativeSystem = (*system_guard).as_mut().unwrap();

				// Update system.
				system.profiles[system.active_profile_index].task_system.run_once(&loop_start);

				// Wait until loop end target instant.
				let loop_end:Instant = loop_start + system.interval;
				let now:Instant = Instant::now();
				if now < loop_end {
					sleep_time = loop_end - now;
				}
			}

			// Sleep designated sleep-time.
			sleep(sleep_time);
		}
	}

	/// Add a profile to the system.
	pub fn add_profile(profile:WindowRelativeProfile) {

		// Get lock on system.
		let mut system_guard:MutexGuard<'_, Option<WindowRelativeSystem>> = SYSTEM_INST.lock().unwrap();
		if system_guard.is_none() {
			*system_guard = Some(WindowRelativeSystem::default());
		}
		let system:&mut WindowRelativeSystem = (*system_guard).as_mut().unwrap();

		// Add profile.
		system.profiles.push(profile);
	}

	/// Update the current profile.
	pub fn update_profile(current_window:WindowController) {
		Self::execute_on_system(|system| {

			// Find active profile.
			let active_process_name:String = current_window.process_name().unwrap_or_default();
			let active_process_title:String = current_window.title();
			let active_profile_index:usize = system.profiles.iter().position(|profile| profile.is_active(&active_process_name, &active_process_title)).unwrap_or(0);

			// Profile change.
			if active_profile_index != system.active_profile_index {
				if let Err(error) = system.profiles[system.active_profile_index].trigger_deactivate_event() {
					(system.error_handler)(&system.profiles[system.active_profile_index], "activate", &*error);
				}
				system.active_profile_index = active_profile_index;
				if let Err(error) = system.profiles[system.active_profile_index].trigger_activate_event() {
					(system.error_handler)(&system.profiles[system.active_profile_index], "deactivate", &*error);
				}
			}
		});
	}



	/* USAGE METHODS */

	/// Execute an action on a locked system instance.
	pub(crate) fn execute_on_system<T>(action:T) where T:Fn(&mut WindowRelativeSystem) {

		// Get lock on system.
		let mut system_guard:MutexGuard<'_, Option<WindowRelativeSystem>> = SYSTEM_INST.lock().unwrap();
		if system_guard.is_none() {
			*system_guard = Some(WindowRelativeSystem::default());
		}
		let system:&mut WindowRelativeSystem = (*system_guard).as_mut().unwrap();

		// If no profiles exist, DefaultProfile hasn't initialized, abort further actions.
		if system.profiles.is_empty() {
			return;
		}

		// Execute the action on the system.
		action(system)
	}

	/// Execute an action on the current profile.
	pub fn execute_on_current_profile<T>(action:T) where T:Fn(&WindowRelativeProfile) {
		Self::execute_on_system(|system|
			action(&system.profiles[system.active_profile_index])
		);
	}
}
impl Default for WindowRelativeSystem {
	fn default() -> Self {
		WindowRelativeSystem {
			profiles: vec![
				WindowRelativeProfile::new("DEFAULT_PROFILE_ID", "DEFAULT_PROFILE_TITLE", "DEFAULT_PROFILE_PROCESS_NAME").with_active_checker(|_, _, _| false)
			],
			active_profile_index: 0,
			error_handler: Box::new(|profile, event_name, error| DEFAULT_ERROR_HANDLER(profile, event_name, error)),
			interval: Duration::from_millis(1000 / 60)
		}
	}
}