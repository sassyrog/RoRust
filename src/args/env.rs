use clap::{Arg, ArgAction, ArgMatches};
use std::io::{self, Write};
// use globalenv::set_var;

fn confirm_prompt(propmt: &str) -> bool {
    println!("{} (y/n): ", propmt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.to_lowercase().contains("y")
}

pub fn set_env_arguemnts() -> Arg {
    Arg::new("env")
        .short('e')
        .long("env")
        .action(ArgAction::Append)
        .required(false)
        .help("Set a new environment variable for your virtual environment e.g. -e ROLE=TEST")
        .value_name("KEY=VALUE")
        .required(false)
        .value_parser(|v: &str| {
            let parts: Vec<&str> = v.split('=').collect();
            if parts.len() != 2 {
                return Err(
                    "Please provide a key value pair separated by an equal sign e.g. ROLE=TEST"
                        .to_string(),
                );
            }
            Ok(v.to_string())
        })
}

pub fn unset_env_arguemnts() -> Arg {
    Arg::new("unenv")
        .long("unenv")
        .action(ArgAction::Append)
        .required(false)
        .help("Unset an environment variable for your virtual environment e.g. -u ROLE")
        .value_name("KEY")
        .required(false)
}

#[allow(dead_code)]
pub fn set_env_handler(matches: &ArgMatches) {
    if let Some(values) = matches.get_many::<String>("env") {
        println!("Environment variables to set:");
        for v in values.clone() {
            println!("  {}", v);
        }
        println!();

        if !confirm_prompt("Are you sure you want to set the following environment variables?") {
            println!("Environment variables not set");
            return;
        }
        for value in values {
            let parts: Vec<&str> = value.split('=').collect();

            let key = parts[0];
            let val = parts[1];

            println!("Key = {}, Value = {}", key, val);
            // print "Success" in green color
            // set_var(key, val);
        }
        println!("\x1b[0;32mSuccess\x1b[0m");
    }
}

#[allow(dead_code)]
pub fn unset_env_handler(matches: &ArgMatches) {
    if let Some(values) = matches.get_many::<String>("unenv") {
        for value in values {
            let parts: Vec<&str> = value.split('=').collect();

            let key = parts[0];

            print!("Key = {}", key);

            // unset_var(key);
        }
    }
}
