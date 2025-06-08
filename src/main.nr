use dep::std::hash::keccak256;

fn main(preimageP1: Field, preimageP2: Field, hashP1: pub Field, hashP2: pub Field) {

    let preimageP1_bytes: [u8; 16] = preimageP1.to_be_radix(256);
    let preimageP2_bytes: [u8; 16] = preimageP2.to_be_radix(256);

    let hashP1_bytes: [u8; 16] = hashP1.to_be_radix(256);
    let hashP2_bytes: [u8; 16] = hashP2.to_be_radix(256);

    let mut preimage_bytes: [u8; 32] = [0; 32];
    let mut hash_bytes: [u8; 32] = [0; 32];

    for i in 0..16 {
        preimage_bytes[i] = preimageP1_bytes[i];
        preimage_bytes[16+i] = preimageP2_bytes[i];
        hash_bytes[i] = hashP1_bytes[i];
        hash_bytes[16+i] = hashP2_bytes[i];
    }

    let h: [u8; 32] = keccak256(preimage_bytes, 32);
    println(h);
    println(hash_bytes);
   assert(hash_bytes == h);
}

#[test]
fn test_hash() {

    let preimageP1: Field = 96231036770457542450679237503772262400;
    let preimageP2: Field = 0;
    let hashP1: Field = 128941730223567639394451260844544274277;
    let hashP2: Field = 67977317891967665821774315185063332656;

    main(preimageP1, preimageP2, hashP1, hashP2);
}

#[test]
fn test_hash2() {

    let preimageP1: Field = 96231036770496792094352034415266785651;
    let preimageP2: Field = 43134663917389751815297038278526500864;
    let hashP1: Field = 194263100461326246941821211252298397678;
    let hashP2: Field = 59759175005763961009877796927821517139;

    main(preimageP1, preimageP2, hashP1, hashP2);
}
