use noir_rs::{
    barretenberg::{prove::prove_ultra_honk, srs::setup_srs, verify::verify_ultra_honk},
    native_types::{Witness, WitnessMap},
    witness::from_vec_str_to_witness_map,
    FieldElement,
};
use serde_json::{to_string, Value};
use std::time::Instant;
use std::{env, fs};

fn noir_prove(witness_str: String, bytecode: String) -> (String, String) {
    println!("Initializing witness...");
    let initial_witness_vals: Vec<&str> = serde_json::from_str::<Vec<&str>>(&witness_str)
        .expect("Failed to parse initial_witness JSON string");

    // let initial_witness_vec: Vec<FieldElement> = initial_witness_vals
    //     .into_iter()
    //     .filter_map(|hex_str| i128::from_str_radix(hex_str.trim_start_matches("0x"), 16).ok())
    //     .map(FieldElement::from)
    //     .collect();

    // let mut initial_witness = WitnessMap::new();
    // for (i, witness) in initial_witness_vec.into_iter().enumerate() {
    //     initial_witness.insert(Witness(i as u32), witness);
    // }

    let initial_witness = from_vec_str_to_witness_map(initial_witness_vals).unwrap();

    println!("Generating proof...");
    let (proof, vk) = prove_ultra_honk(&bytecode, initial_witness, false).unwrap();

    let proof_hex = hex::encode(&proof);
    let vk_hex = hex::encode(&vk);

    return (proof_hex, vk_hex);
}

fn main() {
    env::set_var("RUST_LOG", "trace");

    // const BYTECODE: &str = "H4sIAAAAAAAA/62QQQqAMAwErfigpEna5OZXLLb/f4KKLZbiTQdCQg7Dsm66mc9x00O717rhG9ico5cgMOfoMxJu4C2pAEsKioqisnslysoaLVkEQ6aMRYxKFc//ZYQr29L10XfhXv4jB52E+OpMAQAA";
    // setup_srs(BYTECODE, None, false).unwrap();
    // let initial_witness = from_vec_str_to_witness_map(vec!["5", "6", "0x1e"]).unwrap();
    // let start = std::time::Instant::now();
    // let (proof, vk) = prove_ultra_honk(BYTECODE, initial_witness, false).unwrap();
    // println!("Proof generation time: {:?}", start.elapsed());
    // let verdict = verify_ultra_honk(proof, vk).unwrap();
    // println!("Proof verification verdict: {}", verdict);

    let data = fs::read_to_string("./target/hello_world.json").expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");
    let bytecode: &str = json["bytecode"]
        .as_str()
        .expect("Unable to extract bytecode");

    let start_srs = Instant::now();
    let srs_size = setup_srs(&bytecode, None, false)
        .ok()
        .expect("srs_size failed");

    println!(
        "srs_size {:?} in {:?}ms",
        srs_size,
        (Instant::now().duration_since(start_srs).as_millis())
    );

    let start_prove = Instant::now();
    let witness = Vec::from([
        "310939249775",
        "8909325935247956566886129074037753319",
        "268167857309995084666739127329335872722",
    ]);

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
