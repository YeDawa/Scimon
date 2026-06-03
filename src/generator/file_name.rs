use rand::RngExt;

pub struct FileName {
    ext: String,
    length: usize,
    pattern: String,
}

impl FileName {

    pub fn new(length: usize, ext: &str) -> Self {
        Self {
            length,
            ext: ext.to_string(),
            pattern: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        }
    }
    
    pub fn gen(&self) -> String {
        let charset_len: usize = self.pattern.len();

        let mut rng = rand::rng(); 
        let random_string: String = (0..self.length)
            .map(|_| {
                // 3. `gen_range()` is now `random_range()`
                let idx = rng.random_range(0..charset_len);
                self.pattern.chars().nth(idx).unwrap()
            })
            .collect();
    
        format!("{}.{}", random_string, self.ext)
    }

}
