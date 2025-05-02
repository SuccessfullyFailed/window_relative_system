use super::ServerController;



pub struct ProfileProperties {
	pub(crate) id:String,
	pub(crate) title:String,
	pub(crate) process_name:String,
	pub(crate) profile_dir_path:String,

	pub(crate) server_controller:Option<ServerController>
}
impl ProfileProperties {

	/* CONSTRUCTOR METHODS */

	/// Create a new dataset.
	pub fn new(id:&str, title:&str, process_name:&str, profile_dir_path:&str) -> ProfileProperties {
		ProfileProperties {
			id: id.to_string(),
			title: title.to_string(),
			process_name: process_name.to_string(),
			profile_dir_path: profile_dir_path.to_string(),

			server_controller: None
		}
	}



	/* SUBSERVICE METHODS */

	/// Return self with a server controller.
	pub fn with_server_controller(mut self, server_controller:ServerController) -> Self {
		self.server_controller = Some(server_controller);
		self
	}

	/// Get the profile's server controller.
	pub fn server_controller(&self) -> &Option<ServerController> {
		&self.server_controller
	}
}