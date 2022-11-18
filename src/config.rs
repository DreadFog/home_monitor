use local_ip_address::local_ip;
use std::collections::HashMap;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
// ,ecessary for serializing and deserializing
use serde::{Deserialize, Serialize, Serializer,Deserializer, ser::SerializeStruct};
// https://docs.rs/sysinfo/latest/sysinfo/
pub struct Config {
    private_ip : String,
    hostname : String,
    os : (String, String), // (os_name, os_version)
    cpu_model : String,
    cpu_temps : String, // Only one temperature for now 
    cpu_usages : Vec<String>, 
    ram_usage : (String, String), // (used, total)
    disks_usage : HashMap<String, (String, String)>, // ID -> (used, total)
}
impl Config {
    pub fn new() -> Config {
        // Retrieve the info
        let mut sys = System::new_all();
        sys.refresh_all();

        // Get the private ip
        let private_ip = local_ip().unwrap_or_else(|e | {
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

        // CPU model
        println!("NB CPUs: {}", sys.cpus().len());
        println!("=> disks:");
        for disk in sys.disks() {
            println!("{:?}", disk);
        }

        // Components temperature:
        println!("=> components:");
        for component in sys.components() {
            println!("{:?}", component);
        }
        println!("=> system:");
        // RAM information:
        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());


        Config {
            private_ip,
            hostname,
            os,
            cpu_model : "".to_string(),
            cpu_temps : "".to_string(),
            cpu_usages : vec![],
            ram_usage : ("".to_string(), "".to_string()),
            disks_usage : HashMap::new(),
        }
    }
}

impl Serialize for Config {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Config", 7)?;
        state.serialize_field("private_ip", &self.private_ip)?;
        state.serialize_field("hostname", &self.hostname)?;
        state.serialize_field("os", &self.os)?;
        state.serialize_field("cpu_model", &self.cpu_model)?;
        state.serialize_field("cpu_temps", &self.cpu_temps)?;
        state.serialize_field("cpu_usages", &self.cpu_usages)?;
        state.serialize_field("ram_usage", &self.ram_usage)?;
        state.serialize_field("disks_usage", &self.disks_usage)?;
        state.end()
    }
}