use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // let query = &args[1];
    // let file_path = &args[2];

    // let (query, file_path) = parse_config(&args);
    let config =   Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {err}");
        process::exit(1);
    });



    println!("Searching for the query '{}'", config.query);
    println!("In file '{}'", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}





