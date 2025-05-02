use std::error::Error;



pub struct ServerController {
	starter:Box<dyn Fn() -> Result<(), Box<dyn Error>> + Send>,
	settings_modifier:Option<Box<dyn Fn() -> Result<(), Box<dyn Error>> + Send>>,
	instance_existance_checker:Option<Box<dyn Fn() -> bool + Send>>,
	command_executor:Option<Box<dyn Fn(String) -> Result<(), Box<dyn Error>> + Send>>,
	commands:Vec<(String, String)>
}
impl ServerController {

	/* CONSTRCUTOR METHODS */

	pub fn new<T>(starter:T) -> ServerController where T:Fn() -> Result<(), Box<dyn Error>> + Send + 'static {
		ServerController {
			starter: Box::new(starter),
			settings_modifier: None,
			instance_existance_checker: None,
			command_executor: None,
			commands: Vec::new()
		}
	}


	/* BUILDER METHODS */

	/// Return self with a settings modifier.
	pub fn with_settings_modifier<T>(mut self, modifier:T) -> Self where T:Fn() -> Result<(), Box<dyn Error>> + Send + 'static {
		self.settings_modifier = Some(Box::new(modifier));
		self
	}

	/// Return self with an instance existance checker.
	pub fn with_instance_existance_checker<T>(mut self, checker_function:T) -> Self where T:Fn() -> bool + Send + 'static {
		self.instance_existance_checker = Some(Box::new(checker_function));
		self
	}

	/// Return self with a command executor.
	pub fn with_command_executor<T>(mut self, executor:T) -> Self where T:Fn(String) -> Result<(), Box<dyn Error>> + Send + 'static {
		self.command_executor = Some(Box::new(executor));
		self
	}

	/// Return self with a list of commands. The first entry is the name of the command, the second is the actual command. If the command contains '$i' where i acts as an index, arguments can be passed to the command.
	pub fn with_commands(mut self, commands:&[(&str, &str)]) -> Self {
		self.commands.extend(commands.iter().map(|(name, command)| (name.to_string(), command.to_string())).collect::<Vec<(String, String)>>());
		self
	}



	/* USAGE METHODS */

	/// Start the server.
	pub fn start(&self) -> Result<(), Box<dyn Error>> {
		if self.instance_exists() {
			return Err("Could not start server, server already running.".into());
		}
		(self.starter)()
	}

	/// Check if an instance of the server exists.
	pub fn instance_exists(&self) -> bool {
		self.instance_existance_checker.as_ref().map(|checker| checker()).unwrap_or(false)
	}

	/// Run a command.
	pub fn execute_command<T>(&self, command_reference:T) -> Result<(), Box<dyn Error>> where T:CommandReference {
		match &self.command_executor {
			Some(executor) => {
				let raw_command:String = command_reference.get_command(&self.commands);
				executor(raw_command)
			},
			None => Err("Could not execute command, command executor does not exist.".into())
		}
	}
}



pub trait CommandReference {
	fn get_command(&self, commands_list:&[(String, String)]) -> String;
}
impl CommandReference for &str {
	fn get_command(&self, commands_list:&[(String, String)]) -> String {
		if let Some((_, command)) = commands_list.iter().find(|(name, _)| name == self) {
			return command.to_string();
		}
		self.to_string()
	}
}
impl CommandReference for String {
	fn get_command(&self, commands_list:&[(String, String)]) -> String {
		self.as_str().get_command(commands_list)
	}
}
impl<T> CommandReference for (T, &[&str]) where T:CommandReference {
	fn get_command(&self, commands_list:&[(String, String)]) -> String {
		let mut original_command:String = self.0.get_command(commands_list);
		for (argument_index, argument_value) in self.1.iter().enumerate().rev() { // Parsing smallest first would parse $10 to the value of $1 with a trailing 0.
			original_command = original_command.replace(&format!("${argument_index}"), &argument_value);
		}
		original_command
	}
}