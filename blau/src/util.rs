use actix_web::error::ErrorForbidden as Forbidden;

#[derive(Clone)]
pub struct Util {
    redirect_urls: Vec<String>,
}

impl Util {
    pub fn new(redirect_urls: Vec<String>) -> Self {
        Util { redirect_urls }
    }

    fn get_allowed_redirect_back_url(&self, url: &String) -> Option<String> {
        if url == "/" {
            return Some(url.clone());
        }

        self.redirect_urls
            .iter()
            .find(|allowed_url| url.starts_with(*allowed_url))
            .map(|s| s.clone())
    }

    pub fn get_redirect_back_url(&self, url: Option<String>) -> Result<String, actix_web::Error> {
        if let Some(url) = url {
            match self.get_allowed_redirect_back_url(&url) {
                Some(allowed_url) => return Ok(allowed_url),
                None => return Err(Forbidden("Invalid redirect_back_url")),
            }
        }

        Ok("/".to_string())
    }
}
