mod config;

use eos_prefixd::Session;

use anyhow::Context;
use clap::Parser;
use config::{Args, parse_iface_map};
use ipnet::Ipv6Net;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let args = Args::parse();
    anyhow::ensure!(
        !args.ifaces.is_empty(),
        "provide at least one --iface vlanX:offset"
    );
    anyhow::ensure!(args.addr_host != 0, "--addr-host 0 is not allowed");

    // Parse iface mappings
    let mut mappings = Vec::new();
    for s in &args.ifaces {
        mappings.push(parse_iface_map(s)?);
    }

    // TODO: Replace this with your on-box DHCPv6 IA_PD client result.
    // For now, hardcode an example delegated prefix:
    let delegated: Ipv6Net = "2001:db8:abcd:1200::/56"
        .parse()
        .context("bad delegated prefix literal")?;

    info!("delegated prefix (stub): {}", delegated);

    let mut sess = Session::connect(
        &args.device_url,
        args.username, args.password
    )
    .await?;

    sess.get_addresses("Vlan100").await.unwrap();

    Ok(())
    // Minimal reconcile loop: compute desired /64 + set ::1 on each SVI.
    // Later we’ll add: persist last-applied, delete old, handle renew, etc.
    //loop {
    //    for (ifname, offset) in &mappings {
    //        let p64 = ipv6_alloc::child64(delegated, *offset)?;
    //        let addr = ipv6_alloc::addr_from_host(p64, args.addr_host)?;
    //        let addr_str = addr.to_string();

    //        info!("setting {} -> {} (from {} offset {})", ifname, p64, delegated, offset);

    //        // Apply address (this should implicitly drive RA prefix advertisement on EOS)
    //        if let Err(e) = sess.set_svi_ipv6_addr(ifname, &addr_str, 64).await {
    //            warn!("failed to set addr on {}: {:?}", ifname, e);
    //        }
    //    }

    //    tokio::time::sleep(std::time::Duration::from_secs(args.interval_s)).await;
    //}
}
