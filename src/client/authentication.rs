use crate::error::Error;

impl super::Api {
    /// Create a new logedin API instance.
    pub async fn new_logedin(url: &str, username: &str, password: &str) -> Result<Self, Error> {
        let api = Self::new(url)?;

        api.login(username, password).await?;

        Ok(api)
    }

    /// Login client
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
        if res.status().is_success() {
            Ok(())
        } else {
            Err(Error::AuthFailed)
        }
    }

    /// Logout the client instance
    pub async fn logout(&self) -> Result<(), Error> {
        let url = self._build_url("/api/v2/logout").await?;

        self.http_client.post(url).send().await?;

        Ok(())
    }
}
