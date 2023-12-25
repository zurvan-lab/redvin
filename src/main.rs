use clap::*;

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

    let _root_command = command!()
        .about("Redvin is an IPNN implementation in rust, helping to build decentralized future.")
        .subcommand(init_command)
        .subcommand(start_command)
        .get_matches();
}
