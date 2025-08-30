# Window Relative System

A Rust framework for creating process-specific automation profiles. Each profile corresponds to a process (e.g., `Discord.exe`, `Code.exe`, etc.) and can define event handlers, tasks, and operations that react to window events or user-defined triggers.

---

## ✨ Features

- **Profile creation**  
  Create profiles to handle actions for a specific process.

- **Run targeted actions**  
  Execute an action on one specific profile or all profiles in the system.

- **Schedule actions**  
  For each profile, schedule handlers for events, [task_syncer](https://github.com/SuccessfullyFailed/task_syncer) Tasks and functions by name.
---

## 📦 Installation

Add **Window Relative System** to your `Cargo.toml`:

```toml
[dependencies]
window_relative_system = { git="https://github.com/SuccessfullyFailed/window_relative_system" }
```

Create profiles using one of these methods:

```rust
WindowRelativeSystem::add_profile(WindowRelativeProfile::new("profile_id", "profile_title", "process_name.exe", |profile| profile.with_profile_modifications()));

// Note that the `create_profile` macro creates a function that automatically adds it to the system. This means that this macro can only be used once per context as using it twice would generate two functions with the same name.
create_profile!("profile_id", "profile_title", "process_name.exe", |profile| profile.with_profile_modifications());
```

Run the system using:

```rust
WindowRelativeSystem::run();
```

---

## Example Workspace Layout

Your workspace might look like this. Keeping a separate crate for all profile ensures only modified profiles have to be recompiled.

```
my_project/
│
├── main.rs	# Main application
├── Cargo.toml
└── profiles/
    ├── discord_profile/	# Discord profile crate
    ├── xbox_party_profile/	# Xbox Party profile crate
    └── vs_code_profile/	# VS Code profile crate
```

---

## Example Crates

### Main Application (`main.rs`)

```rust
use window_relative_system::WindowRelativeSystem;

pub fn main() {

	// Example hotkey: ALT+Z triggers "send_code_to_callers" on the "vs_code" profile
	set_hotkey(KEY_ALT + KEY_Z, || {
		WindowRelativeSystem::execute_named_operation_on_profile_by_id("vs_code", "send_code_to_callers");
	});

	// Run the window-relative system.
	WindowRelativeSystem::run();
}
```

### Discord Profile (`profiles/discord_profile`)

```rust
use window_relative_system::create_profile;
use task_syncer::Task;



create_profile!(
	"discord",
	"Discord",
	"Discord.exe",
	|profile| profile
		.with_activate_handler(|_profile_properties| {
			let window_title:String = get_window_title();
			let callers_title:&str = window_title.replace(" - Discord", "");

			// Extract caller information from the window title
			CallerStorage::store("discord", match &callers_title[..1] {

				// Direct user.
				"@" => vec![callers_title[1..].to_string()],

				// Server.
				"#" => vec![format!("[server] {}", &callers_title[1..])],

				// Group of users
				_ => {
					if callers_title.contains(',') {
						callers_title.split(',').map(|name| name.trim().to_string()).collect::<Vec<String>>()
					} else {
						Vec::new()
					}
				}
			});
			Ok(())
		})
		.with_task(Task::new("auto_save", |event| {
			const INTERVAL:Duration = Duration::from_secs(60);
			press_hotkey(KEY_CONTROL + KEY_S);
			event.reschedule(INTERVAL);
		}))
);
```

### Xbox Party Profile (`profiles/xbox_party_profile`)

```rust
use window_relative_system::create_profile;



create_profile!(
	"xbox_party_window",
	"Xbox PartyWindow",
	"ApplicationFrameHost.exe",
	|profile| profile

		// A customized method to check if this profile is active as ApplicationFrameHost covers a lot of applications.
		.with_active_checker(|profile_properties, active_process_name, active_process_title| {
			active_process_name == profile_properties.process_name() && active_process_title.contains("party")
		})
		.with_activate_handler(|_profile_properties| {
			let callers:Vec<String> = get_xbox_callers_from_screenshot();
			CallerStorage::store("xbox_party", callers);
			Ok(())
		})
);
```

### VS Code Profile (`profiles/vs_code_profile`)

```rust
use window_relative_system::create_profile;



create_profile!(
	"vs_code",
	"VS Code",
	"Code.exe",
	|profile| profile
		.with_named_operation("send_code_to_callers", || {
			let current_callers:Vec<String> = CallerStorage::all_callers();
			let code:String = get_code_in_current_file()?;
			let file_path:String = format!("Z:/SharedCode/{}/{}.txt", current_callers.join(", "), Instant::now());
			upload_code_to_network_drive(file_path, code);
			Ok(())
		})
);
```


## 📝 License

Licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)  
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

---