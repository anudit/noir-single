use noir_rs::{
    native_types::{Witness, WitnessMap},
    FieldElement,
};
use serde_json::Value;
use std::{env, fs};

fn main() {
    env::set_var("RUST_LOG", "trace");

    let data = fs::read_to_string("./target/hello_world.json").expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");
    let bytecode: &str = json["bytecode"]
        .as_str()
        .expect("Unable to extract bytecode");

    println!("Initializing witness...");
    let mut initial_witness = WitnessMap::new();
    initial_witness.insert(Witness(0), FieldElement::zero());
    initial_witness.insert(Witness(1), FieldElement::from(1_i128));

    dbg!(String::from(bytecode));

    println!("Generating proof...");
    let (proof, vk) = noir_rs::prove(String::from(bytecode), initial_witness).unwrap();
    println!("Verifying proof...");
    let verdict = noir_rs::verify(String::from(bytecode), proof, vk).unwrap();
    assert!(verdict);
    println!("Proof correct");
}
