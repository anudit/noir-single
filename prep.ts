import { keccak256 } from "viem";

const bytesToHex = (bytes: Uint8Array): string =>
  "0x" +
  Array.from(bytes)
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");


export function prepareSplitCircuitInputs(preimageString: string): {
  preimageP1: string;
  preimageP2: string;
  hashP1: string;
  hashP2: string;
} {
  const encoder = new TextEncoder();
  const originalPreimageBytes = encoder.encode(preimageString);

  if (originalPreimageBytes.length > 32) {
    throw new Error(
      `Invalid preimage: Must be 32 bytes or less, but received ${originalPreimageBytes.length} bytes for string "${preimageString}".`
    );
  }

  // The Uint8Array is initialized with zeros, so this effectively right-pads with zeros.
  const paddedPreimage = new Uint8Array(32);
  paddedPreimage.set(originalPreimageBytes);

  const fullHashHex = keccak256(paddedPreimage);

  // Split the 32-byte padded preimage into two 16-byte chunks.
  const preimagePart1Bytes = paddedPreimage.slice(0, 16);
  const preimagePart2Bytes = paddedPreimage.slice(16, 32);

  // Split the 32-byte (64-character) hex hash into two 16-byte parts.
  const hashP1 = "0x" + fullHashHex.substring(2, 34);
  const hashP2 = "0x" + fullHashHex.substring(34);

  return {
    preimageP1: BigInt(bytesToHex(preimagePart1Bytes)).toString(),
    preimageP2: BigInt(bytesToHex(preimagePart2Bytes)).toString(),
    hashP1: BigInt(hashP1).toString(),
    hashP2: BigInt(hashP2).toString(),
    rust: [
      "0x" + BigInt(bytesToHex(preimagePart1Bytes)).toString().toString(16),
      "0x" + BigInt(bytesToHex(preimagePart2Bytes)).toString().toString(16),
      "0x" + BigInt(hashP1).toString(16),
      "0x" + BigInt(hashP2).toString(16),
    ],
  };
}

function main() {
  const ip = prepareSplitCircuitInputs("Hello");
  console.log(ip);
}

main()
