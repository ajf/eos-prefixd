use super::gnmi::{Path, PathElem, Update, TypedValue};
use super::gnmi::typed_value::Value;

fn pe(name: &str, keys: &[(&str, &str)]) -> PathElem {
    let mut key = std::collections::HashMap::new();
    for (k, v) in keys {
        key.insert((*k).to_string(), (*v).to_string());
    }
    PathElem { name: name.to_string(), key }
}

fn path(elems: Vec<PathElem>) -> Path {
    Path { origin: "".into(), target: "".into(), elem: elems, element: vec![] }
}

pub fn path_svi_addr_entry(ifname: &str, addr: &str) -> Path {
    path(vec![
        pe("interfaces", &[]),
        pe("interface", &[("name", ifname)]),
        pe("routed-vlan", &[]),
        pe("ipv6", &[]),
        pe("addresses", &[]),
        pe("address", &[("ip", addr)]),
    ])
}
pub fn path_svi_entry(ifname: &str) -> Path {
    path(vec![
        pe("interfaces", &[]),
        pe("interface", &[("name", ifname)]),
        pe("routed-vlan", &[]),
        pe("ipv6", &[]),
        pe("addresses", &[]),
        pe("address", &[]),
    ])
}

fn path_svi_addr_ip(ifname: &str, addr: &str) -> Path {
    path(vec![
        pe("interfaces", &[]),
        pe("interface", &[("name", ifname)]),
        pe("routed-vlan", &[]),
        pe("ipv6", &[]),
        pe("addresses", &[]),
        pe("address", &[("ip", addr)]),
        pe("config", &[]),
        pe("ip", &[]),
    ])
}

fn path_svi_addr_plen(ifname: &str, addr: &str) -> Path {
    path(vec![
        pe("interfaces", &[]),
        pe("interface", &[("name", ifname)]),
        pe("routed-vlan", &[]),
        pe("ipv6", &[]),
        pe("addresses", &[]),
        pe("address", &[("ip", addr)]),
        pe("config", &[]),
        pe("prefix-length", &[]),
    ])
}

fn str_update(p: Path, v: &str) -> Update {
    Update {
        path: Some(p),
        val: Some(TypedValue { value: Some(Value::StringVal(v.to_string())) }),
        ..Default::default()
    }
}

fn u32_update(p: Path, v: u32) -> Update {
    Update {
        path: Some(p),
        val: Some(TypedValue { value: Some(Value::UintVal(v as u64)) }),
        ..Default::default()
    }
}

pub fn updates_set_svi_addr(ifname: &str, addr: &str, plen: u32) -> Vec<Update> {
    vec![
        str_update(path_svi_addr_ip(ifname, addr), addr),
        u32_update(path_svi_addr_plen(ifname, addr), plen),
    ]
}
