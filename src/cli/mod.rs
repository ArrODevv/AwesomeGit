use std::env;

/// # Config
/// 
/// Everything needed to start the cli and configure stuff beforehand.
pub mod config;

/// Prints the help for AwesomeGit.
pub fn print_help() -> () {
    let exe_name = match env::current_exe() {
        Ok(path) => {
            match path.file_name() {
                Some(name) => String::from(match name.to_str() {
                    Some(s) => s,
                    None => "awesome_git"
                }),
                None => String::from("awesome_git")
            }
        },
        Err(e) => panic!("Couldn't get current exe name: {e}")
    };

    println!("Usage: {exe_name} (command) [subcommand(s)] [flag(s)]", );
    println!("Commands:");
    println!("  - init: Initializes an empty repository.");
    println!("Flags:");
    println!("  - \"--version\", \"-v\": Prints the version of awesome_git and your local git install.");
    println!("  - \"--help\", \"-?\": Prints this help.");
}
