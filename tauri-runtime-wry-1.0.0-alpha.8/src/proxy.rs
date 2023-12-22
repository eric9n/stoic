use url::Url;
use wry::{ProxyConfig, ProxyEndpoint};

#[derive(Debug, Clone)]
pub struct ProxySettings(ProxyConfig);

impl ProxySettings {
    pub fn new(url_str: &str) -> Result<Self, &'static str> {
        let url = Url::parse(url_str).map_err(|_| "Invalid URL")?;

        let host = url.host_str().ok_or("Missing host")?.to_string();
        let port = url.port().ok_or("Missing port").map(|p| p.to_string())?;

        let proxy_endpoint = ProxyEndpoint { host, port };

        // 根据 URL 的协议确定代理类型
        let config = match url.scheme() {
            "http" | "https" => ProxyConfig::Http(proxy_endpoint),
            "socks5" => ProxyConfig::Socks5(proxy_endpoint),
            _ => return Err("Unsupported proxy type"),
        };

        Ok(ProxySettings(config))
    }

    pub fn proxy_config(&self) -> ProxyConfig {
        self.0.clone()
    }
}
