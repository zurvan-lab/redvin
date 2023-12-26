use clap::*;
use config::Config;
use std::fs;

fn main() {
    let init_command = command!()
    .name("init")
    .about("Initializing a Redvin node.")
    .arg(
        Arg::new("working-directory")
        .long("working-directory")
        .short('w')
        .help("Working directory is the path for saving relay, peers data and some other stuff for managing node.")
        .aliases(["workdir", "wdir", "workingdirectory", "workingdir", "wdirectory"])
        .required(true)
    );

    let start_command = command!()
        .name("start")
        .about("Starting a Redvin instance by passing a working directory.");

    let root_command = command!()
        .about("Redvin is an IPNN implementation in rust, helping to build decentralized future.")
        .subcommand(init_command)
        .subcommand(start_command)
        .get_matches();

    match root_command.subcommand() {
        Some(subcommand) => {
            if subcommand.0 == "init" {
                let dir_builder = fs::DirBuilder::new();

                let working_directory = subcommand
                    .1
                    .get_one::<String>("working-directory")
                    .expect("invalid working directory path.");

                dir_builder
                    .create(working_directory)
                    .expect("can not create working directory.");

                let config_path = format!("{}/config.toml", working_directory);
                fs::write(config_path, Config::default_file())
                    .expect("can not create config file on working directory.");
            } else if subcommand.0 == "start" {
                println!("not implemented yet!")
            }
        }
        None => {
            println!("please use `redvin --help`")
        }
    }
}
