use std::net::IpAddr;

pub struct Locat {
    geoip: maxminddb::Reader<Vec<u8>>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("maxminddb error: {0}")]
    MaxMindDb(#[from] maxminddb::MaxMindDBError),
}

impl Locat {
    pub fn new(geoip_country_db_path: &str, _analytics_db_path: &str) -> Result<Self, Error> {
        Ok(Self { 
            geoip: maxminddb::Reader::open_readfile(geoip_country_db_path)?,
         })
    }

    pub fn ip_to_iso_code(&self, addr: IpAddr) -> Option<&str> {
        self.geoip
            .lookup::<maxminddb::geoip2::Country>(addr)
            .ok()?
            .country?
            .iso_code
    }

    pub fn get_analytics(&self) -> Vec<(String, u64)> {
        Default::default()
    }
}