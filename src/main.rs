use std::default;

use clap::Command;
mod args;
mod subcommands;
mod utils;

fn main() {
    let cmd: Command = Command::new("Venvit")
	.version("0.1.0")
	.about("A simple CLI tool to manage python virtual environments")
	.after_help("This is a simple CLI tool to manage python virtual environments for different python versions.
	It is a work in progress and is not yet ready for production use.");

    let matches = cmd
        .arg(args::env::set_env_arguemnts())
        .arg(args::env::unset_env_arguemnts())
        .subcommand(subcommands::vi_command::vi_command())
        .subcommand(subcommands::vn_command::vn_command())
        .get_matches();

    // process env argument matches
    args::env::set_env_handler(&matches);
    args::env::unset_env_handler(&matches);

    // process subcommand matches
    if let Some(matches) = matches.subcommand_matches("vi") {
        subcommands::vi_command::process_vi_command(matches);
    }

    let mut default_installation_folder = utils::python::get_default_installation_folder();
    default_installation_folder = default_installation_folder.join("Venvit").join("Python");

    utils::python::download_and_extract_python("3.9.7", &default_installation_folder);

    // subcommands::load_python_versions().unwrap();

    // unsafe {
    //     println!("{:?}", subcommands::NODE_VERSIONS);
    // }

    // print!("{:?}", matches);
}
