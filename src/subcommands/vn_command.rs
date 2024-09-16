use clap::{Arg, Command};

pub fn vn_command() -> Command {
    Command::new("vn")
        .about("creates a new virtual environment")
        .after_help(
            "This command creates a new virtual environment for the specified python version.",
        )
        .arg(
            Arg::new("python_version")
                .short('p')
                .long("python")
                .help("The python version to use for the virtual environment")
                .value_name("PYTHON_VERSION")
                .required(true),
        )
}
