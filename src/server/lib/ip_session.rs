#[derive(Debug, Clone)]
pub struct IpSession {
    ip_map: mini_moka::sync::Cache<String, u8>,
}

impl IpSession {
    pub fn increment_ip(&self, ip: String) {
        let mut value = 0u8;

        if let Some(current) = self.ip_map.get(&ip) {
            value = current.saturating_add(1);
        }

        self.ip_map.insert(ip, value);
    }

    pub fn reset_ip(&self, ip: &String) {
        self.ip_map.invalidate(ip);
    }

    pub fn ip_blocked(&self, ip: &String) -> bool {
        if let Some(current) = self.ip_map.get(ip) {
            return current >= 5;
        }

        false
    }
}

impl Default for IpSession {
    fn default() -> Self {
        Self {
            ip_map: mini_moka::sync::CacheBuilder::new(10_000)
                .time_to_idle(std::time::Duration::from_secs(15 * 60))
                .build(),
        }
    }
}
