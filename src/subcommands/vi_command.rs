use clap::{Arg, Command};


pub fn vi_command() -> Command {
		
		Command::new("vi")
		.about("Initializes a new python virtual environment")
		.after_help("This command initializes a new python virtual environment for the specified python version.")
		.arg(
			Arg::new("python_version")
			.short('p')
			.long("python")
			.help("The python version to use for the virtual environment")
			.value_name("PYTHON_VERSION")
			.required(true)
			
		)
	}


pub fn process_vi_command(matches: &clap::ArgMatches) {
	// subcommands::load_python_versions().unwrap();
	// let python_version = matches.value_of("python_version").unwrap();
	// println!("Creating virtual environment for python version: {}", python_version);
}