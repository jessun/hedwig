use anyhow::{Ok, Result};
use reqwest::Client;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

pub struct GmailClient {
    client: Client,
}

impl GmailClient {
    pub fn new() -> Self {
        GmailClient {
            client: Client::new(),
        }
    }

    pub async fn feed_atom(&self, username: &str, password: &str) -> Result<String> {
        let url = "https://mail.google.com/mail/feed/atom";

        tracing::info!("http request gmail feed atom");
        let resp = self
            .client
            .get(url)
            .basic_auth(username, Some(password))
            // 2. 优化 Header 写法，直接传常量即可，不需要 .to_string()
            .header(http::header::USER_AGENT.to_string(), USER_AGENT)
            .send()
            .await?
            // 3. 关键：如果没有这一行，密码错误(401)也会被当成成功处理
            .error_for_status()?;

        let text = resp.text().await?;
        Ok(text)
    }
}
