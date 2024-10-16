use noir_rs::srs::{get_srs, localsrs::LocalSrs, netsrs::NetSrs, Srs};
use serde_json::Value;

fn main() {
    let srs: Srs = NetSrs::new(4194304).to_srs();
    let local_srs = LocalSrs(srs);

    let save_path = "./srs22.dat";
    local_srs.save(Some(&save_path));
}
