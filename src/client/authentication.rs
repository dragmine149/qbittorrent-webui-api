use crate::error::Error;

#[derive(Debug)]
pub struct Creddentials {
    username: String,
    password: String,
}

impl Creddentials {
    pub fn new<T: ToString>(username: T, password: T) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub fn quary_string(&self) -> String {
        return format!("username={}&password={}", self.username, self.password);
    }
}

impl super::Client {
    pub async fn login(&self, cred: Creddentials) -> Result<(), Error> {
        let url = self.build_url("/api/v2/auth/login").await?;
        let res = self
            .http_client
            .post(url)
            .body(cred.quary_string())
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

    pub async fn logout(&self) -> Result<(), Error> {
        let url = self.build_url("/api/v2/logout").await?;

        self.http_client.post(url).send().await?;

        Ok(())
    }
}
