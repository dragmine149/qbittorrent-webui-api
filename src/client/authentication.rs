use crate::error::Error;

impl super::Api {
    /// Create a new API instance and login to the service.
    ///
    /// # Arguments
    /// * `url` - The base URL of the API service.
    /// * `username` - The username for authentication.
    /// * `password` - The password for authentication.
    pub async fn new_login(url: &str, username: &str, password: &str) -> Result<Self, Error> {
        let api = Self::_new(url)?;

        api.login(username, password).await?;

        Ok(api)
    }

    /// Login to the service.
    ///
    /// # Arguments
    /// * `username` - The username for authentication.
    /// * `password` - The password for authentication.
    pub async fn login(&self, username: &str, password: &str) -> Result<(), Error> {
        let url = self._build_url("/api/v2/auth/login").await?;
        let res = self
            .http_client
            .post(url)
            .body(format!("username={}&password={}", username, password))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("refer", self.base_url.read().await.to_string())
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(Error::AuthFailed);
        }

        // Checks if the result from the API is one of a success.
        let login = res.text().await?;
        if login.to_lowercase() == "fails." {
            return Err(Error::AuthFailed);
        }

        Ok(())
    }

    /// Logout the client instance
    pub async fn logout(&self) -> Result<(), Error> {
        let url = self._build_url("/api/v2/logout").await?;

        self.http_client.post(url).send().await?;

        Ok(())
    }
}
