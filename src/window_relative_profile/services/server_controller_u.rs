#[cfg(test)]
mod tests {
	use crate::{ ProfileProperties, ServerController };
	use std::sync::Mutex;



	#[test]
	fn test_server_controller_full_test() {
		static FUNCTION_EXECUTION_VALIDATOR:Mutex<u8> = Mutex::new(0);

		// Create controller.
		let server_controller:ServerController = ServerController::new(|| { *FUNCTION_EXECUTION_VALIDATOR.lock().unwrap() = 1; Ok(()) })
									.with_instance_existance_checker(|| *FUNCTION_EXECUTION_VALIDATOR.lock().unwrap() > 0)
									.with_command_executor(|num_str| { *FUNCTION_EXECUTION_VALIDATOR.lock().unwrap() = num_str.parse().unwrap(); Ok(()) })
									.with_commands(&[("set_num_255","255")]);

		// Validate actions handled properly.
		assert_eq!(*FUNCTION_EXECUTION_VALIDATOR.lock().unwrap(), 0);
		server_controller.start().unwrap();
		assert!(server_controller.instance_exists());
		assert_eq!(*FUNCTION_EXECUTION_VALIDATOR.lock().unwrap(), 1);
		assert!(server_controller.start().is_err(), "Second start was successful. Second start should fail due to the first instance existing.");
		assert_eq!(*FUNCTION_EXECUTION_VALIDATOR.lock().unwrap(), 1);
		server_controller.execute_command("12").unwrap();
		assert_eq!(*FUNCTION_EXECUTION_VALIDATOR.lock().unwrap(), 12);
		server_controller.execute_command("set_num_255").unwrap();
		assert_eq!(*FUNCTION_EXECUTION_VALIDATOR.lock().unwrap(), 255);
	}

	#[test]
	fn test_server_controller_accesible_in_proifile_properties() {
		static FUNCTION_EXECUTION_VALIDATOR:Mutex<u8> = Mutex::new(0);

		let profile_properties:ProfileProperties = ProfileProperties::new("", "", "", "").with_server_controller(ServerController::new(|| { *FUNCTION_EXECUTION_VALIDATOR.lock().unwrap() = 1; Ok(()) }));
		assert!(profile_properties.server_controller.is_some());
		profile_properties.server_controller.unwrap().start().unwrap();
		assert_eq!(*FUNCTION_EXECUTION_VALIDATOR.lock().unwrap(), 1)
	}
}