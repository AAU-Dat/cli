use clap::command;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let _matches = command!()
        .author("Daniel Runge Petersen <dpet20@student.aau.dk>")
        .subcommand(subcommand::setup())
        .subcommand(subcommand::project())
        .get_matches();

    Ok(Config { })
}

mod subcommand {
    use clap::{arg, Arg, Command};

    pub fn project() -> Command<'static> {
        Command::new("project")
            .about("Manage the project")
            .long_about("Manage the project.")
            .subcommand(
                Command::new("init")
                    .display_order(1)
                    .about("Create a new aau project in an existing directory")
                    .long_about("Create a new aau project in an existing directory. \n\nTo create a repository interactively, use `aau project create` with no arguments.")
                    .arg(
                        Arg::new("path")
                            .display_order(1)
                            .takes_value(true)
                            .value_name("PATH")
                            .default_value(".")
                        ))
                    
            .subcommand(
                Command::new("create")
                    .display_order(1)
                    .about("Create a new aau project")
                    .long_about("Create a new aau project at [PATH]. \n\nTo create a repository interactively, use `aau project create` with no arguments.")
                    .arg(
                        Arg::new("path")
                            .display_order(1)
                            .takes_value(true)
                            .value_name("PATH")
                            ))
    }

    pub fn setup() -> Command<'static> {
        let semesters = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];

        Command::new("setup")
            .about("Setup WSL for an aau project")
            .long_about("Setup WSL for an aau project.\n\nThe following programs are installed with this command: TO INSERT SMARTLY")
            .arg(
                Arg::new("semester")
                    .value_name("SEMESTER")
                    .possible_values(semesters),
            )
            .arg(
                arg!(--"no-github" "Setup without github CLI")
                    .display_order(2)
                    .takes_value(false),
            )
            .arg(
                arg!(--"no-vscode" "Setup without Visual Studio Code")
                    .display_order(2)
                    .takes_value(false),
            )
    }
}
