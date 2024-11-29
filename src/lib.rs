use std::net::IpAddr;

pub struct Locat {}

impl Locat {
    pub fn new(_geoip_country_db_path: &str, _analytics_db_path: &str) -> Self {
        Self {  }
    }

    pub fn ip_to_iso_code(&self, _addr: IpAddr) -> Option<&str> {
        None
    }

    pub fn get_analytics(&self) -> Vec<(String, u64)> {
        Default::default()
    }
}