mod cli;
mod init;
mod start;

fn main() {
    let command = cli::command();

    match command.subcommand() {
        Some(subcommand) => {
            if subcommand.0 == "init" {
                init::init(
                    subcommand
                        .1
                        .get_one::<String>("working_directory")
                        .expect("invalid working directory."),
                )
            } else if subcommand.0 == "start" {
                start::start("");
            }
        }
        None => {
            println!("please use `redvin --help`")
        }
    }
}
