use clap::Parser;
use http::Uri;

#[derive(Parser, Debug, Clone)]
pub struct Args {
    /// gNMI target, e.g. 127.0.0.1:6030 or [::1]:6030
    #[arg(long, default_value = "arista://[::1]:6030")]
    pub device_url: Uri,


    #[arg(long)]
    pub username: String,

    #[arg(long)]
    pub password: String,

    /// Upstream interface for DHCP client
    #[arg(long)]
    pub upstream: String,

    /// SVI mappings: --iface vlan100:0 --iface vlan200:1
    #[arg(long = "iface")]
    pub ifaces: Vec<String>,

    /// Requested PD prefix length (hint only; ISP may ignore)
    #[arg(long, default_value_t = 56)]
    pub requested_pd_len: u8,

    /// Host ID within each /64 (default 1 => ::1). Must be non-zero.
    #[arg(long, default_value_t = 1)]
    pub addr_host: u64,

    /// Poll interval (seconds) for PD refresh/reconcile loop (stubbed for now)
    #[arg(long, default_value_t = 30)]
    pub interval_s: u64,
}

pub fn normalize_ifname(s: &str) -> String {
    if let Some(rest) = s.strip_prefix("vlan") {
        format!("Vlan{}", rest)
    } else if let Some(rest) = s.strip_prefix("Vlan") {
        format!("Vlan{}", rest)
    } else {
        s.to_string()
    }
}

pub fn parse_iface_map(s: &str) -> anyhow::Result<(String, u32)> {
    let (ifname, off) = s
        .split_once(':')
        .ok_or_else(|| anyhow::anyhow!("Bad --iface '{s}': expected name:offset"))?;
    let offset: u32 = off
        .parse()
        .map_err(|_| anyhow::anyhow!("Bad offset in --iface '{s}'"))?;
    Ok((normalize_ifname(ifname), offset))
}
