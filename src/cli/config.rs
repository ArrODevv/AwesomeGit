use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, ArgMatches, ArgAction, Command};

pub fn init_cli() -> ArgMatches {
    let cli = Command::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        /*.arg(
            Arg::new("help")
                .required(false)
                .short('h')
                .action(ArgAction::Help)
                .help("Print a help text.")
        )
        .arg(
            Arg::new("version")
                .required(false)
                .short('v')
                .action(ArgAction::Version)
                .help("Prints the current version of awesome_git and your local git install.")
        )*/
        .arg(
            Arg::new("init")
                .help("Initializes an empty git repository in the current working directory.")
                .action(ArgAction::Set)
        );

        cli.get_matches()
}