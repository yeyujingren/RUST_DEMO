use std::{ env, process };
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing argument: \n {}", err);
            process::exit(1);
        });
    println!("In file {}", config.filename);

    // run(config)
    //     .unwrap_or_else(|err| {
    //         println!("There has something wrong: \n{}", err);
    //         process::exit(1);
    //     });

    if let Err(e) = minigrep::run(config) {
        eprintln!("There has something wrong: \n {}", e);
        process::exit(1);
    }
}