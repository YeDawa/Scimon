use crate::consts::uris::Uris;

pub struct SciHub {
    url: String,
}

impl SciHub {

    pub fn new(url: &str) -> Self {
        Self {
            url: format!("{}{}", Uris::SCIHUB_PROXY_REQUEST_PDF, url.to_string())
        }
    }

    pub fn get_url(&self) -> String {
        return self.url.clone();
    }

    pub fn name(&self) -> (String, String) {
        let mut url_str = self.url.replace(Uris::SCIHUB_PROXY_REQUEST_PDF, "");
        url_str = url_str.replace("https://", "").replace("http://", "").replace(Uris::PROVIDERS_DOMAINS[7], "");

        let last_slices = url_str
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        (self.url.clone(), last_slices[1].to_string() + ".pdf")
    }

}