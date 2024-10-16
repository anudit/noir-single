use noir_rs::{
    native_types::{Witness, WitnessMap},
    prove::prove,
    prove::prove_honk,
    srs::setup_srs,
    verify::verify,
    verify::verify_honk,
    FieldElement,
};
use serde_json::{to_string, Value};
use std::time::Instant;
use std::{env, fs};

fn noir_prove(witness_str: String, bytecode: String, srs_size: u32) -> (String, String) {
    println!("Initializing witness...");
    let initial_witness_vals: Vec<String> =
        serde_json::from_str::<Vec<String>>(&witness_str.to_string())
            .expect("Failed to parse initial_witness JSON string");

    let initial_witness_vec: Vec<FieldElement> = initial_witness_vals
        .into_iter()
        .filter_map(|hex_str| i128::from_str_radix(hex_str.trim_start_matches("0x"), 16).ok())
        .map(FieldElement::from)
        .collect();

    let mut initial_witness = WitnessMap::new();
    for (i, witness) in initial_witness_vec.into_iter().enumerate() {
        initial_witness.insert(Witness(i as u32), witness);
    }

    println!("Generating proof...");
    let (proof, vk) = prove(String::from(bytecode), initial_witness, srs_size).unwrap();

    let proof_hex = hex::encode(&proof);
    let vk_hex = hex::encode(&vk);

    return (proof_hex, vk_hex);
}

fn noir_prove2(bytecode: String) -> (String, String) {
    println!("Initializing witness...");
    let mut initial_witness = WitnessMap::new();
    initial_witness.insert(Witness(0), FieldElement::from(2_i128));
    initial_witness.insert(Witness(1), FieldElement::from(3_i128));

    println!("Generating proof...");
    let (proof, vk) = prove_honk(String::from(bytecode), initial_witness).unwrap();

    let proof_hex = hex::encode(&proof);
    let vk_hex = hex::encode(&vk);

    // dbg!(String::from(&proof_hex));
    // dbg!(String::from(&vk_hex));

    return (proof_hex, vk_hex);
}

fn main() {
    env::set_var("RUST_LOG", "trace");

    let data = fs::read_to_string("./target/hello_world.json").expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");
    let bytecode: &str = json["bytecode"]
        .as_str()
        .expect("Unable to extract bytecode");

    let start_srs = Instant::now();
    let srs_size = setup_srs(bytecode.to_string(), None)
        .ok()
        .expect("srs_size failed");

    println!(
        "srs_size {:?} in {:?}ms",
        srs_size,
        (Instant::now().duration_since(start_srs).as_millis())
    );

    let start_prove = Instant::now();
    let witness = Vec::from(["0x1", "0x1"]);
    let (proof_hex, vk_hex) = noir_prove(
        to_string(&witness).unwrap(),
        String::from(bytecode),
        srs_size.clone(),
    );
    println!(
        "Plonk Proof Gen Done in {:?}ms",
        (Instant::now().duration_since(start_prove).as_millis())
    );

    // let (proof_hex2, vk_hex2) = noir_prove2(String::from(bytecode));
    // println!("Honk Proof Gen Done in {:?}", (Instant::now() - start));

    let proof = hex::decode(proof_hex).expect("proof Hex Decode Failed");
    let vk = hex::decode(vk_hex).expect("vk Hex Decode Failed");

    let start_verify = Instant::now();
    let verdict = verify(proof, vk, srs_size).unwrap();
    assert!(verdict);
    println!(
        "Proof Verification Done in {:?}ms {:?}",
        (Instant::now().duration_since(start_verify).as_millis()),
        verdict
    );
}
