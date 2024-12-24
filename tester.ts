import { keccak256 } from "viem";

function uint8ArrayToBigInt(u8Arr: Uint8Array) {
  let result = 0n;
  for (const byte of u8Arr) {
    // Shift the current result left by 8 bits, then add the new byte.
    result = (result << 8n) + BigInt(byte);
  }
  return result;
}

function keccack(data: Uint8Array) {
  function hexToUint8Array(hexString: string | `0x${string}`) {
    if (hexString.startsWith("0x")) hexString = hexString.slice(2);
    // Ensure the hex string has an even number of characters
    if (hexString.length % 2 !== 0) {
      throw new Error("Hex string must have an even length");
    }

    // Create a Uint8Array of the appropriate length
    const byteArray = new Uint8Array(hexString.length / 2);

    // Convert every two hex characters to a byte
    for (let i = 0; i < hexString.length; i += 2) {
      byteArray[i / 2] = parseInt(hexString.substring(i, i + 2), 16);
    }

    return byteArray;
  }

  let hash = keccak256(data);
  return new Uint8Array(hexToUint8Array(hash));
}

const enc = new TextEncoder();
const inputData = enc.encode("Hello");
// const inputData = new Uint8Array([11, 12]);
const outputHash = keccack(inputData);

console.log({
  inputData,
  preimage: uint8ArrayToBigInt(inputData).toString(),
  arrP1: outputHash.slice(0, 16),
  arrP2: outputHash.slice(16, 32),
  hashP1: uint8ArrayToBigInt(outputHash.slice(0, 16)).toString(),
  hashP2: uint8ArrayToBigInt(outputHash.slice(16, 32)).toString(),
  rust: [
    "0x" + uint8ArrayToBigInt(inputData).toString(16),
    "0x" + uint8ArrayToBigInt(outputHash.slice(0, 16)).toString(16),
    "0x" + uint8ArrayToBigInt(outputHash.slice(16, 32)).toString(16),
  ],
});
