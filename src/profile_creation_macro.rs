#[macro_export]
macro_rules! create_profile {

	// No modifiers
	($id:expr, $title:expr, $process_name:expr) => {
		#[ctor::ctor]
		fn add_to_system() {
			let profile = window_relative_system::WindowRelativeProfile::new($id, $title, $process_name);
			window_relative_system::WindowRelativeSystem::add_profile(profile);
		}
	};

	// One modifier
	($id:expr, $title:expr, $process_name:expr, $modifier:expr) => {
		#[ctor::ctor]
		fn add_to_system() {
			let mut profile = window_relative_system::WindowRelativeProfile::new($id, $title, $process_name);
			let modifier: fn(window_relative_system::WindowRelativeProfile) -> window_relative_system::WindowRelativeProfile = $modifier;
			profile = modifier(profile);
			window_relative_system::WindowRelativeSystem::add_profile(profile);
		}
	};

	// Multiple modifiers (comma-separated list)
	($id:expr, $title:expr, $process_name:expr, [$($modifier:expr),+ $(,)?]) => {
		#[ctor::ctor]
		fn add_to_system() {
			let mut profile = window_relative_system::WindowRelativeProfile::new($id, $title, $process_name);
			$(
				let modifier: fn(window_relative_system::WindowRelativeProfile) -> window_relative_system::WindowRelativeProfile = $modifier;
				profile = modifier(profile);
			)+
			window_relative_system::WindowRelativeSystem::add_profile(profile);
		}
	};
}