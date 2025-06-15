use crate::error::Error;

impl super::Api {
    /// Create a new API instance and login to the service.
    pub async fn login(url: &str, username: &str, password: &str) -> Result<Self, Error> {
        let api = Self::_new(url)?;

        let url = api._build_url("/api/v2/auth/login").await?;
        let res = api
            .http_client
            .post(url)
            .body(format!("username={}&password={}", username, password))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("refer", api.base_url.read().await.to_string())
            .send()
            .await?;
        if res.status().is_success() {
            Ok(api)
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
