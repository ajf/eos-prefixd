use anyhow::Result;
use std::net::Ipv6Addr;
use async_trait::async_trait;

use http::Uri;

use crate::arista_session::AristaSession;

#[async_trait]
pub trait DeviceSession: Send {
    //    async fn set_svi_ipv6_addr(&mut self, ifname: &str, addr: &str, plen: u32) -> Result<()>;
    //    async fn delete_svi_ipv6_addr(&mut self, ifname: &str, addr: &str) -> Result<()>;
    async fn get_addresses(&mut self, ifname: &str) -> anyhow::Result<Vec<Ipv6Addr>>;
}

/// Type-erased session wrapper so the daemon doesn’t care about implementation.
pub struct Session {
    inner: Box<dyn DeviceSession>,
}

impl Session {
    pub async fn connect(url: &Uri, username: String, password: String) -> Result<Self> {
        let inner: Box<dyn DeviceSession> = match url.scheme_str().unwrap() {
            "arista" => Box::new(AristaSession::connect(url, username, password).await?),
            other => anyhow::bail!("unsupported device protocol '{other}'"),
        };
        Ok(Self { inner })
    }

    pub async fn get_addresses(&mut self, ifname: &str) -> Result<Vec<Ipv6Addr>, anyhow::Error> {
        self.inner.get_addresses(ifname).await
    }

    //    pub async fn set_svi_ipv6_addr(&mut self, ifname: &str, addr: &str, plen: u32) -> Result<()> {
    //        self.inner.set_svi_ipv6_addr(ifname, addr, plen).await
    //    }
    //
    //    pub async fn delete_svi_ipv6_addr(&mut self, ifname: &str, addr: &str) -> Result<()> {
    //        self.inner.delete_svi_ipv6_addr(ifname, addr).await
    //    }
}
