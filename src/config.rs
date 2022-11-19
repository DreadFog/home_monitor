use local_ip_address::local_ip;
use std::collections::HashMap;
use sysinfo::*;
// necessary for serializing and deserializing
use serde::{Serialize, Deserialize};
use regex::Regex;
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    private_ip : String,
    hostname : String,
    os : (String, String), // (os_name, os_version)
    cpu_model : (String, String),
    cpu_count : u8,
    cpu_temps : String, // Only one temperature for now 
    cpu_usages : Vec<String>, 
    ram_usage : (String, String), // (used, total)
    disks_usage : HashMap<String, (String, String)>, // mount point -> (available, total)
}
impl Config {
    pub fn new() -> Config {
        // Retrieve the info
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();

        // Get the private ip
        let private_ip = local_ip().unwrap_or_else(|e | {
            eprintln!("Error getting the local IP: {}", e);
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(0,0,0,0))
        }).to_string();

        // Retrieve the hostname
        let hostname = sys.host_name().unwrap_or_else(|| {
            "Unknown".to_string()
        });

        // Retrieve the OS name and version
        let os = (sys.name().unwrap_or_else(|| {
            "Unknown".to_string()
        }), sys.os_version().unwrap_or_else(|| {
            "Unknown".to_string()
        }));

        // CPU data
        let cpu_count: u8 = sys.cpus().len()
                            .try_into().unwrap_or_else(|e| {eprintln!("error in cpu count: {}", e); 0});
        let cpu_model = (sys.cpus()[0].name().to_string(), sys.cpus()[0].vendor_id().to_string());

        // Components temperature:
        for component in sys.components() {
            println!("{:?}", component);
        }
        // RAM information:
        let ram_usage = (sys.used_memory().to_string(), sys.total_memory().to_string());

        // disks
        // for WSL or Linux
        // only take default disks : mounted on /mnt/x, filter out the /mnt/wsl
        // for Windows, take all the disks looking like C:\\
        let re = Regex::new(r"^/mnt/[a-z]$|^.:\\$").unwrap();
        let list_disks = sys.disks().iter().filter(|x| re.is_match(x.mount_point().to_str().unwrap_or("")));
        let mut disks_usage = HashMap::new();
        for disk in list_disks {
            println!("{:?}", disk);
            disks_usage.insert(disk.mount_point().to_str().unwrap_or("").to_string(), (disk.available_space().to_string(), disk.total_space().to_string()));
        }

        Config {
            private_ip,
            hostname,
            os,
            cpu_model,
            cpu_count,
            cpu_temps : "".to_string(), // TODO
            cpu_usages : vec![], // TODO
            ram_usage,
            disks_usage,
        }
    }
    pub fn get_private_ip(&self) -> &str {
        &self.private_ip
    }
}