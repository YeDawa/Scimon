pub struct Encoders;

impl Encoders {

    pub fn percent_decode(&self, input: &str) -> String {
        let bytes = input.as_bytes();
        let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i] == b'%' && i + 2 < bytes.len() {
                let hi = (bytes[i + 1] as char).to_digit(16);
                let lo = (bytes[i + 2] as char).to_digit(16);

                if let (Some(hi), Some(lo)) = (hi, lo) {
                    out.push((hi * 16 + lo) as u8);
                    i += 3;
                    continue;
                }
            }

            out.push(bytes[i]);
            i += 1;
        }

        String::from_utf8_lossy(&out).to_string()
    }

    pub fn percent_encode(&self, input: &str) -> String {
        let mut out = String::with_capacity(input.len());

        for &byte in input.as_bytes() {
            match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    out.push(byte as char);
                }
                
                _ => out.push_str(&format!("%{:02X}", byte)),
            }
        }

        out
    }

}