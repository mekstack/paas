use actix_web::error::ErrorForbidden as Forbidden;

#[derive(Clone)]
pub struct Util {
    redirect_urls: Vec<String>,
}

impl Util {
    pub fn new(redirect_urls: Vec<String>) -> Self {
        Util { redirect_urls }
    }

    fn is_valid_redirect_back_url(&self, url: &String) -> bool {
        if url == "/" {
            return true;
        }

        self.redirect_urls
            .iter()
            .any(|allowed_url| url == allowed_url)
    }

    pub fn get_redirect_back_url(&self, url: Option<String>) -> Result<String, actix_web::Error> {
        if let Some(url) = url {
            match self.is_valid_redirect_back_url(&url) {
                true => return Ok(url),
                false => return Err(Forbidden("Invalid redirect_back_url")),
            }
        }

        Ok("/".to_string())
    }
}
