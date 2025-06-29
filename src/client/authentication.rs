use crate::{error::Error, models::Credentials};

impl super::Api {
    /// Create a new API instance and login to the service.
    ///
    /// # Arguments
    /// * `url` - The base URL of the API service.
    /// * `credentials` - The credentials required for authentication.
    pub async fn new_login(url: &str, credentials: Credentials) -> Result<Self, Error> {
        let api = Self::new(url)?;

        api.login(credentials).await?;

        Ok(api)
    }

    /// Create a new API instance and login to the service with username and password.
    ///
    /// # Arguments
    /// * `url` - The base URL of the API service.
    /// * `username` - The username for authentication.
    /// * `password` - The password for authentication.
    pub async fn new_login_with_username_password(
        url: &str,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<Self, Error> {
        let credentials = Credentials::new(username, password);

        Self::new_login(url, credentials).await
    }

    /// Login to the service.
    ///
    /// # Arguments
    /// * `credentials` - The credentials required for authentication.
    pub async fn login(&self, credentials: Credentials) -> Result<(), Error> {
        let url = self._build_url("/api/v2/auth/login").await?;
        let res = self
            .http_client
            .post(url)
            .body(credentials.to_string())
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
