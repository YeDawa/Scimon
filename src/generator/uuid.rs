use std::fmt;

use rand::{
    rng, 
    RngExt
}; 

pub struct Uuid([u8; 16]);

impl fmt::Display for Uuid {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = &self.0;
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11],
            bytes[12], bytes[13], bytes[14], bytes[15]
        )
    }

}

impl Uuid {

    pub fn new(bytes: [u8; 16]) -> Self {
        Uuid(bytes)
    }

    pub fn v4(&self) -> String {
        let mut r = rng(); 
        let mut uuid_bytes = [0u8; 16];
    
        r.fill(&mut uuid_bytes);   
        uuid_bytes[6] = (uuid_bytes[6] & 0x0F) | 0x40;
        uuid_bytes[8] = (uuid_bytes[8] & 0x3F) | 0x80;
    
        let uuid = Uuid(uuid_bytes);
        uuid.to_string()
    }
    
}