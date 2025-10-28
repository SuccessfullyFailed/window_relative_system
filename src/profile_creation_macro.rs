#[macro_export]
macro_rules! create_profile {
	($id:expr, $title:expr, $process_name:expr, $($modifier:expr),*) => {
		#[ctor::ctor]
		fn add_to_system() {
			let mut profile = window_relative_system::WindowRelativeProfile::new($id, $title, $process_name);
			$(
				let modifier:fn(window_relative_system::WindowRelativeProfile) -> window_relative_system::WindowRelativeProfile = $modifier;
				profile = modifier(profile);
			)+
			window_relative_system::WindowRelativeSystem::add_profile(profile);
		}
	};
}