use dep::std;

fn main(preimage: Field, hashP1: pub Field, hashP2: pub Field) {

    let preimage_bytes: [u8; 5] = preimage.to_be_radix(256);
    // let h: [u8; 32] = std::hash::keccak256(preimage_bytes, 5);

    let hashP1_bytes: [u8; 16] = hashP1.to_be_radix(256);
    let hashP2_bytes: [u8; 16] = hashP2.to_be_radix(256);

    let mut hash_bytes: [u8; 32] = [0; 32];

    for i in 0..16 {
        hash_bytes[i] = hashP1_bytes[i];
        hash_bytes[16+i] = hashP2_bytes[i];
    }

   // assert(hash_bytes == h);
}

#[test]
fn test_main() {

    let preimage: Field = 310939249775;
    let hashP1: Field = 8909325935247956566886129074037753319;
    let hashP2: Field = 268167857309995084666739127329335872722;

    main(preimage, hashP1, hashP2);
}
