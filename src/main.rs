use std::env;
mod master_functions;
mod slave_functions;
mod config;
use master_functions::handle_connection;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Arg[1] is equal to 1 when the master process is running
    // Arg[1] is equal to 0 otherwise
    let role = &args[1].parse::<i8>().unwrap_or_else( |e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    if *role == 1 {
        println!("Master process");
        // create two sub processes
        // the first one handles the connections from the slave processes
        let mut child1 = std::process::Command::new("cargo")
            .arg("run")
            .arg("0")
            .spawn()
            .expect("Failed to spawn child process");
        // the second one handles the webpage requests from the browser
        let mut child2 = std::process::Command::new("cargo")
            .arg("run")
            .arg("0")
            .spawn()
            .expect("Failed to spawn child process");
        } else {
        println!("Slave process");
        // generate the config struct
        let config = config::Config::new();
}
