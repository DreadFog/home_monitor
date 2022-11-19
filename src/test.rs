use std::sync::Mutex;
fn main() {
    let m = Mutex::new(5);

    {
        let mut num : MutexGuard = m.lock().unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        });
        *num = 6;
    }

    println!("m = {:?}", m);
}