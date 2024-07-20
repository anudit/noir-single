use noir_rs::AcirField;
use noir_rs::{
    native_types::{Witness, WitnessMap},
    prove::prove,
    srs::netsrs::NetSrs,
    verify::verify,
    FieldElement,
};
use serde_json::Value;
use std::time::Instant;
use std::{env, fs};

fn halo2_prove(bytecode: String) -> (String, String) {
    println!("Initializing witness...");
    let mut initial_witness = WitnessMap::new();
    initial_witness.insert(Witness(0), FieldElement::from(2_i128));
    initial_witness.insert(Witness(1), FieldElement::from(3_i128));

    println!("Generating proof...");
    let (proof, vk) = prove(String::from(bytecode), initial_witness, None).unwrap();

    let proofHex = hex::encode(&proof);
    let vkHex = hex::encode(&vk);

    dbg!(String::from(&proofHex));
    dbg!(String::from(&vkHex));

    return (proofHex, vkHex);
}

fn main() {
    env::set_var("RUST_LOG", "trace");

    let data = fs::read_to_string("./target/hello_world.json").expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");
    let bytecode: &str = json["bytecode"]
        .as_str()
        .expect("Unable to extract bytecode");

    let start = Instant::now();
    let (proofHex, vkHex) = halo2_prove(String::from(bytecode));
    println!("Proof Gen Done in {:?}", (Instant::now() - start));

    let proof = hex::decode(proofHex).expect("proof Hex Decode Failed");
    let vk = hex::decode(vkHex).expect("vk Hex Decode Failed");

    let start2 = Instant::now();
    let verdict = verify(String::from(bytecode), proof, vk, Some("./srs.dat")).unwrap();
    assert!(verdict);
    println!(
        "Proof Verification Done in {:?} {:?}",
        (Instant::now() - start),
        verdict
    );
}
