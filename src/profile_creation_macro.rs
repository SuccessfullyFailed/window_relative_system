#[macro_export]
macro_rules! create_profile {
	($id:expr, $title:expr, $process_name:expr, $properties_modifier:expr) => {
		#[ctor::ctor]
		fn add_to_system() {
			let mut profile = crate::WindowRelativeProfile::new($id, $title, $process_name);
			let modifier:fn(crate::WindowRelativeProfile) -> crate::WindowRelativeProfile = $properties_modifier;
			profile = modifier(profile);
			crate::WindowRelativeSystem::add_profile(profile);
		}
	};
}