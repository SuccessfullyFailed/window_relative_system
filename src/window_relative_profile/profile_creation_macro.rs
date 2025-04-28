#[macro_export]
macro_rules! create_profile {
	($struct_name:ident, $id:expr, $title:expr, $process_name:expr, $profile_dir_path:expr, $properties_modifier:expr) => {
		
		// Struct definition.
		pub struct $struct_name(crate::ProfileProperties);

		// Default instance.
		impl Default for $struct_name {
			fn default() -> Self {
				let modifier:fn(crate::ProfileProperties) -> crate::ProfileProperties = $properties_modifier;
				$struct_name(modifier(crate::ProfileProperties::new($id, $title, $process_name, $profile_dir_path)))
			}
		}

		// Profile traits.
		impl crate::WindowRelativeProfile for $struct_name {
			fn properties(&self) -> &crate::ProfileProperties { &self.0 }
			fn properties_mut(&mut self) -> &mut crate::ProfileProperties { &mut self.0 }
		}

		/// Add this profile to the system.
		#[ctor::ctor]
		fn add_to_system() {
			use crate::WindowRelativeProfile;

			let mut profile = $struct_name::default();
			profile.on_create();
			crate::WindowRelativeSystem::add_profile(profile);
		}
	};
}