#[derive(Clone)]
pub struct Equipement {
    addr: String,
    port: u16
}

impl Equipement {

    pub fn new(ad: String, p: u16) -> Self {
        Self {
            addr: ad,
            port: p
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn address(&self) -> String {
        self.addr.clone()
    }
}
