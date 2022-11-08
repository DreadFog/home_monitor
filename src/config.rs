use local_ip_address::local_ip;
use reqwest;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
pub struct Config {
    public_ip : String,
    private_ip : String,
    cpu_model : String,
    cpu_temp : String,
    cpu_usage : String,
    ram_usage : String,
    disk_usage : String,
    free_disk_space : String,
    os : Info,
    hostname : String,
}
impl Config {
    pub fn new() -> Config {
        // get the public IP from ifconfig.me
        let public_ip = reqwest::get("https://ifconfig.me/ip")
            .expect("Failed to get the public IP")
            .text()
            .expect("Failed to convert the public IP to a string").to_string();
        let private_ip = local_ip().unwrap().to_string();
        let sys_info = sys_info::cpu_info().unwrap();

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
    pub fn get_os(&self) -> String {
        self.os.clone()
    }
    pub fn get_hostname(&self) -> String {
        self.hostname.clone()
    }
}