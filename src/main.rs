use noir_rs::{
    barretenberg::{
        prove::prove_ultra_honk, srs::setup_srs_from_bytecode, utils::get_honk_verification_key,
        verify::verify_ultra_honk,
    },
    native_types::{Witness, WitnessMap},
    FieldElement,
};
use serde_json::{to_string, Value};
use std::time::Instant;
use std::{env, fs};

fn noir_prove(witness_str: String, bytecode: String) -> (String, String) {
    println!("Initializing witness...");
    let initial_witness_vals: Vec<&str> = serde_json::from_str::<Vec<&str>>(&witness_str)
        .expect("Failed to parse initial_witness JSON string");

    let vk = get_honk_verification_key(&bytecode, false).unwrap();

    let mut witness_map = WitnessMap::new();

    for (i, witness) in initial_witness_vals.iter().enumerate() {
        witness_map.insert(
            Witness(i as u32),
            FieldElement::try_from_str(*witness).unwrap_or_default(),
        );
    }

    println!("Generating proof...");
    let proof = prove_ultra_honk(&bytecode, witness_map, false).unwrap();

    let proof_hex = hex::encode(&proof);
    let vk_hex = hex::encode(&vk);

    return (proof_hex, vk_hex);
}

fn main() {
    let data = fs::read_to_string("./target/hello_world.json").expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");
    let bytecode: &str = json["bytecode"]
        .as_str()
        .expect("Unable to extract bytecode");

    let start_srs = Instant::now();
    let srs_size = setup_srs_from_bytecode(&bytecode, None, false)
        .ok()
        .expect("srs_size failed");

    println!(
        "srs_size {:?} in {:?}ms",
        srs_size,
        (Instant::now().duration_since(start_srs).as_millis())
    );

    let witness = Vec::from([
        "96231036770496792094352034415266785651",
        "43134663917389751815297038278526500864",
        "194263100461326246941821211252298397678",
        "59759175005763961009877796927821517139",
    ]);

    let start_prove = Instant::now();
    let (proof_hex, vk_hex) = noir_prove(to_string(&witness).unwrap(), String::from(bytecode));
    println!(
        "ultra_honk Proof Gen Done in {:?}ms",
        (Instant::now().duration_since(start_prove).as_millis())
    );

    let proof = hex::decode(proof_hex).expect("proof Hex Decode Failed");
    let vk = hex::decode(vk_hex).expect("vk Hex Decode Failed");

    let start_verify = Instant::now();
    let verdict = verify_ultra_honk(proof, vk).unwrap();
    assert!(verdict);
    println!(
        "Proof Verification Done in {:?}ms {:?}",
        (Instant::now().duration_since(start_verify).as_millis()),
        verdict
    );
}
