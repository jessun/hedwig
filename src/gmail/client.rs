use anyhow::Result;
use reqwest::{Client, Proxy};

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

pub struct GmailClient {
    client: Client,
}

impl GmailClient {
    pub fn new(proxy_url: Option<String>) -> Result<Self> {
        let mut builder = Client::builder();

        if let Some(url) = proxy_url
            && !url.is_empty()
        {
            tracing::info!("proxy set {:?}", url);
            let proxy = Proxy::all(url)?;
            builder = builder.proxy(proxy);
        } else {
            tracing::info!("no proxy");
        }

        let client = builder.build()?;

        Ok(GmailClient { client })
    }

    pub async fn feed_atom(&self, username: &str, password: &str) -> Result<String> {
        let url = "https://mail.google.com/mail/feed/atom";

        tracing::info!("http request gmail feed atom");
        let resp = self
            .client
            .get(url)
            .basic_auth(username, Some(password))
            .header(http::header::USER_AGENT.to_string(), USER_AGENT)
            .send()
            .await?
            .error_for_status()?;

        let text = resp.text().await?;
        Ok(text)
    }
}
