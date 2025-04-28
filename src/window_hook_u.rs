#[cfg(test)]
mod tests {
	use std::{ thread::sleep, time::Duration };
	use crate::window_hook;


	#[test]
	fn test_hook_installation() {

		// Simply test if the hook can be installed without panicing.
		window_hook::install(true);
		sleep(Duration::from_millis(500));
	}
}