use crate::WindowRelativeProfile;



pub trait WindowRelativeProfileService:Sized {

	/// Return the given profile with the service applied to it.
	fn apply_to_profile<T:WindowRelativeProfile + 'static>(self, mut profile:T) -> T {
		self.apply_to_profile_ref(&mut profile);
		profile
	}

	/// Apply the service to the given profile using a mutable reference.
	fn apply_to_profile_ref(self, profile:&mut dyn WindowRelativeProfile);
}