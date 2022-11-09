use local_ip_address::local_ip;
use reqwest;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
// https://docs.rs/sysinfo/latest/sysinfo/
pub struct Config {
    public_ip : String,
    private_ip : String,
    cpu_model : String,
    cpu_temp : String,
    cpu_usage : String,
    ram_usage : String,
    disk_usage : String,
    free_disk_space : String,
    hostname : String,
}
impl Config {
    pub fn new() -> Config {
        // Retrieve the info
        let mut sys = System::new_all();
        sys.refresh_all();
        // get the public IP from ifconfig.me
        // We display all disks' information:
        println!("=> disks:");
        for disk in sys.disks() {
            println!("{:?}", disk);
        }

        // Network interfaces name, data received and data transmitted:
        println!("=> networks:");
        for (interface_name, data) in sys.networks() {
            println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
        }

        // Components temperature:
        println!("=> components:");
        for component in sys.components() {
            println!("{:?}", component);
        }

        println!("=> system:");
        // RAM and swap information:
        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());
        println!("total swap  : {} bytes", sys.total_swap());
        println!("used swap   : {} bytes", sys.used_swap());

        // Display system information:
        println!("System name:             {:?}", sys.name());
        println!("System kernel version:   {:?}", sys.kernel_version());
        println!("System OS version:       {:?}", sys.os_version());
        println!("System host name:        {:?}", sys.host_name());

        // Number of CPUs:
        println!("NB CPUs: {}", sys.cpus().len());

        Config {
            public_ip : "127.0.0.1".to_string(),
            private_ip : "127.0.0.1".to_string(),
            cpu_model : "Intel(R) Core(TM) i7-8750H CPU @ 2.20GHz".to_string(),
            cpu_temp : "0".to_string(),
            cpu_usage : "0".to_string(),
            ram_usage : "0".to_string(),
            disk_usage : "0".to_string(),
            free_disk_space : "0".to_string(),
            hostname : "localhost".to_string(),
        } 
    }
    pub fn get_cpu_model(&self) -> String {
        self.cpu_model.clone()
    }
    pub fn get_cpu_temp(&self) -> String {
        self.cpu_temp.clone()
    }
    pub fn get_cpu_usage(&self) -> String {
        self.cpu_usage.clone()
    }
    pub fn get_ram_usage(&self) -> String {
        self.ram_usage.clone()
    }
    pub fn get_disk_usage(&self) -> String {
        self.disk_usage.clone()
    }
    pub fn get_free_disk_space(&self) -> String {
        self.free_disk_space.clone()
    }
    pub fn get_hostname(&self) -> String {
        self.hostname.clone()
    }
}