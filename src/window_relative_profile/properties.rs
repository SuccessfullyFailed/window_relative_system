pub struct ProfileProperties {
	pub(crate) id:String,
	pub(crate) title:String,
	pub(crate) process_name:String,
	pub(crate) profile_dir_path:String
}
impl ProfileProperties {

	/* CONSTRUCTOR METHODS */

	/// Create a new dataset.
	pub fn new(id:&str, title:&str, process_name:&str, profile_dir_path:&str) -> ProfileProperties {
		ProfileProperties {
			id: id.to_string(),
			title: title.to_string(),
			process_name: process_name.to_string(),
			profile_dir_path: profile_dir_path.to_string()
		}
	}
}