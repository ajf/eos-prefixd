use std::net::Ipv6Addr;

use anyhow::Context;
use http::Uri;
use http::uri::{Parts, Scheme};
use tonic::transport::{Channel, Endpoint};
use tonic::{Request, metadata::MetadataValue};

use crate::session::DeviceSession;

use crate::gnmi::{GetRequest, SetRequest};
use crate::gnmi::g_nmi_client::GNmiClient;

pub struct AristaSession {
    client: GNmiClient<Channel>,
    username: String,
    password: String,
}

impl AristaSession {
    pub async fn connect(uri: &Uri, username: String, password: String) -> anyhow::Result<Self> {
        let new_uri = Uri::builder()
            .path_and_query("/")
            .scheme(
                match uri.scheme().unwrap().as_str() {
                    "arista" => "http",
                    "aristas" => "https",
                    _ => panic!("wrong scheme"),
                }
                .parse::<Scheme>()
                .unwrap(),
            )
            .authority(uri.authority().unwrap().clone())
            .build().unwrap();

        let endpoint = Endpoint::from_shared(format!("{new_uri}"))
            .context("bad endpoint")?;
        let channel = endpoint.connect().await.context("connect failed")?;

        Ok(Self {
            client: GNmiClient::new(channel),
            username,
            password,
        })
    }

    fn authed<T>(&self, mut req: Request<T>) -> Request<T> {
        req.metadata_mut().insert("username", MetadataValue::try_from(self.username.as_str()).unwrap());
        req.metadata_mut().insert("password", MetadataValue::try_from(self.password.as_str()).unwrap());
        req
    }
}

#[async_trait::async_trait]
impl DeviceSession for AristaSession {
    async fn get_addresses(&mut self, ifname: &str) -> anyhow::Result<Vec<Ipv6Addr>> {
		let addresses = crate::paths::path_svi_entry(ifname);

        let req = GetRequest { path: vec![addresses], ..Default::default() };

        let response = self.client.get(self.authed(Request::new(req))).await.unwrap().into_inner();


        // TODO: parse the response to get the list of Ipv6addrs

        Ok(vec![Ipv6Addr::LOCALHOST])
    }
    //    async fn set_svi_ipv6_addr(&mut self, ifname: &str, addr: &str, plen: u32) -> anyhow::Result<()> {
    //        let updates = paths::updates_set_svi_addr(ifname, addr, plen);
    //        let req = SetRequest { update: updates, ..Default::default() };
    //        self.client.set(self.authed(Request::new(req))).await?;
    //        Ok(())
    //    }
    //
    //    async fn delete_svi_ipv6_addr(&mut self, ifname: &str, addr: &str) -> anyhow::Result<()> {
    //        let deletes = vec![paths::path_svi_addr_entry(ifname, addr)];
    //        let req = SetRequest { delete: deletes, ..Default::default() };
    //        self.client.set(self.authed(Request::new(req))).await?;
    //        Ok(())
    //    }
}
