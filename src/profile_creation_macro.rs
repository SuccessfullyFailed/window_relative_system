#[macro_export]
macro_rules! create_profile {
	($id:expr, $title:expr, $process_name:expr, $($modifier:expr),*) => {
		#[ctor::ctor]
		fn add_to_system() {
			let mut profile = window_relative_system::WindowRelativeProfileCore::new($id, $title, $process_name);
			$(
				let modifier:fn(window_relative_system::WindowRelativeProfileCore) -> window_relative_system::WindowRelativeProfileCore = $modifier;
				profile = modifier(profile);
			)+
			window_relative_system::WindowRelativeSystem::add_profile(profile);
		}
	};
}