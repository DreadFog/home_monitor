use std::env;
use std::thread;
mod master_functions;
mod slave_functions;
mod config;
use master_functions::handle_communication;
use master_functions::webpage_display;
fn main() {
    let args: Vec<String> = env::args().collect();
    // Arg[1] is equal to 1 when the master process is running
    // Arg[1] is equal to 0 otherwise
    let role = &args[1].parse::<i8>().unwrap_or_else( |e| {
        eprintln!("The argument must be an integer: {}", e);
        std::process::exit(1);
    });
    if *role == 1 {
        println!("Master process");
        // Create a first thread that will handle communication with the slaves.
        let comm_thread = thread::spawn(|| {
            handle_communication();
        });   
        let web_thread = thread::spawn(|| {
            webpage_display();
        });
        // keep the spawned threads alive
        web_thread.join().unwrap(); 
        comm_thread.join().unwrap();
        // Create a second thread that will handle the webpage display   
        } else {
        println!("Slave process");
        // generate the config struct
        //let config = config::Config::new();
    }
}