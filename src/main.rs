use std::{env, error::Error};

use awesome_git::cli::{config::init_cli, print_help};
use clap::ArgMatches;
use git2::Repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: ArgMatches = init_cli();

    println!(r#"     __          ________  _____  ____  __  __ ______    _____ _____ _______ "#);
    println!(r#"    /\ \        / /  ____|/ ____|/ __ \|  \/  |  ____|  / ____|_   _|__   __|"#);
    println!(r#"   /  \ \  /\  / /| |__  | (___ | |  | | \  / | |__    | |  __  | |    | |   "#);
    println!(r#"  / /\ \ \/  \/ / |  __|  \___ \| |  | | |\/| |  __|   | | |_ | | |    | |   "#);
    println!(r#" / ____ \  /\  /  | |____ ____) | |__| | |  | | |____  | |__| |_| |_   | |   "#);
    println!(r#"/_/    \_\/  \/   |______|_____/ \____/|_|  |_|______|  \_____|_____|  |_|   "#);
    println!(                                                                                  );
    println!(  "-----------------------------------------------------------------------------" );
    println!(                                                                                  );
    
    if args.contains_id("init") {
        match Repository::init(env::current_dir().expect("Couldn't get working dir.")) {
            Ok(repo) => repo,
            Err(e) => panic!("Failed to initialize local repository: {}", e)
        };

        println!("Initialized empty git repository!");
    } else if args.ids().count() == 0 {
        print_help();
    }

    Ok(())
}
