use std::{ thread::sleep, sync::{ Mutex, MutexGuard }, time::{ Duration, Instant } };
use crate::{ window_hook, WindowRelativeProfile };
use window_controller::WindowController;



static SYSTEM_INST:Mutex<Option<WindowRelativeSystem>> = Mutex::new(None);
static RUN_LOCK:Mutex<bool> = Mutex::new(false);
pub(crate) const DEFAULT_ERROR_HANDLER:&dyn Fn(&WindowRelativeProfile, &str, &str) = &|profile, event_name, error| eprintln!("Profile {} panicked in {} event: {}", profile.id(), event_name, error);



pub struct WindowRelativeSystem {
	profiles:Vec<WindowRelativeProfile>,
	active_profile_index:usize,
	error_handler:Box<dyn Fn(&WindowRelativeProfile, &str, &str) + Send>,
	interval:Duration
}
impl WindowRelativeSystem {
	
	/* EVENT METHODS */

	/// Run the system. Installs the hook that triggers events in profiles.
	pub fn run() {

		// Get run lock, ensuring no async running of the system.
		let mut run_lock_handle:MutexGuard<'_, bool> = RUN_LOCK.lock().unwrap();
		if *run_lock_handle {
			eprintln!("Cannot run WindowRelativeSystem twice at the same time.");
			return;
		}
		*run_lock_handle = true;
		drop(run_lock_handle);

		// Ensure system existence.
		window_hook::install(true);
		Self::execute_on_system(|_| {});

		// Repeat indefinitely.
		loop {
			let mut sleep_time:Option<Duration> = None;
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
					sleep_time = Some(loop_end - now);
				}
			}

			// Sleep designated sleep-time.
			if let Some(sleep_time) = sleep_time {
				sleep(sleep_time);
			}
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
	pub fn update_profile(previous_window:WindowController, current_window:WindowController) {
		Self::execute_on_system(|system| {

			// Find active profile.
			let active_process_name:String = current_window.process_name().unwrap_or_default();
			let active_process_title:String = current_window.title();
			let active_profile_index:usize = system.profiles.iter().position(|profile| profile.is_active(&current_window, &active_process_name, &active_process_title)).unwrap_or(0);

			// Profile change.
			if active_profile_index != system.active_profile_index {
				if let Err(error) = system.profiles[system.active_profile_index].trigger_deactivate_event(&previous_window) {
					(system.error_handler)(&system.profiles[system.active_profile_index], "deactivate", &error.to_string());
				}
				system.active_profile_index = active_profile_index;
				if let Err(error) = system.profiles[system.active_profile_index].trigger_activate_event(&current_window) {
					(system.error_handler)(&system.profiles[system.active_profile_index], "activate", &error.to_string());
					system.profiles[system.active_profile_index].task_system.resume();
					system.profiles[system.active_profile_index].task_system.run_once(&Instant::now());
				}
			}
		});
	}



	/* USAGE METHODS */

	/// Execute an action on a locked system instance.
	pub(crate) fn execute_on_system<T, U>(action:T) -> Option<U> where T:Fn(&mut WindowRelativeSystem) -> U {

		// Get lock on system.
		let mut system_guard:MutexGuard<'_, Option<WindowRelativeSystem>> = SYSTEM_INST.lock().unwrap();
		if system_guard.is_none() {
			*system_guard = Some(WindowRelativeSystem::default());
		}
		let system:&mut WindowRelativeSystem = (*system_guard).as_mut().unwrap();

		// If no profiles exist, DefaultProfile hasn't initialized, abort further actions.
		if system.profiles.is_empty() {
			return None;
		}

		// Execute the action on the system.
		Some(action(system))
	}

	/// Execute an action on the current profile.
	pub fn execute_on_current_profile<T, U>(action:T) -> Option<U> where T:Fn(&mut WindowRelativeProfile) -> U {
		Self::execute_on_system(|system| {
			action(&mut system.profiles[system.active_profile_index])
		})
	}

	/// Execute an action on a profile by id. Uses system error handler if profile cannot be found.
	pub fn execute_on_profile_by_id<T, U>(profile_id:&str, action:T) -> Option<U> where T:Fn(&mut WindowRelativeProfile) -> U {
		Self::execute_on_system(|system| {
			match system.profiles.iter_mut().find(|profile| profile.id() == profile_id) {
				Some(profile) => Some(action(profile)),
				None => {
					(system.error_handler)(&system.profiles[system.active_profile_index], "action on profile by id", &format!("Could not find profile by id '{profile_id}'."));
					None
				}
			}
		}).flatten()
	}

	/// Execute an action on all profiles. Excludes the DefaultProfile.
	pub fn execute_on_all_profiles<T, U>(action:T) -> Vec<U> where T:Fn(&mut WindowRelativeProfile) -> U {
		Self::execute_on_system(|system| {
			(1..system.profiles.len()).map(|profile_index| action(&mut system.profiles[profile_index])).collect()
		}).unwrap_or_default()
	}

	/// Execute an action on all profiles. Includes the DefaultProfile.
	pub fn execute_on_all_profiles_and_default<T, U>(action:T) -> Vec<U> where T:Fn(&mut WindowRelativeProfile) -> U {
		Self::execute_on_system(|system| {
			(0..system.profiles.len()).map(|profile_index| action(&mut system.profiles[profile_index])).collect()
		}).unwrap_or_default()
	}
}
impl Default for WindowRelativeSystem {
	fn default() -> Self {
		WindowRelativeSystem {
			profiles: vec![
				WindowRelativeProfile::new("DEFAULT_PROFILE_ID", "DEFAULT_PROFILE_TITLE", "DEFAULT_PROFILE_PROCESS_NAME").with_active_checker(|_, _, _, _| false).with_is_default_profile()
			],
			active_profile_index: 0,
			error_handler: Box::new(|profile, event_name, error| DEFAULT_ERROR_HANDLER(profile, event_name, error)),
			interval: Duration::from_millis(1000 / 60)
		}
	}
}