use std::error::Error;

use crate::consts::addons::Addons;

pub struct Entropy;

impl Entropy {
    
    pub fn calculate(&self, data: &[u8]) -> Result<(f64, bool), Box<dyn Error>> {
        let mut freq = [0; 256];

        for &byte in data {
            freq[byte as usize] += 1;
        }

        let mut entropy = 0.0;
        let len = data.len() as f64;

        for &count in &freq {
            if count > 0 {
                let p = count as f64 / len;
                entropy -= p * p.log2();
            }
        }

        Ok((entropy, entropy > Addons::MAX_SAFE_ENTROPY))
    }

}