use tiny_keccak::{Hasher, Keccak};

pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(data);
    hasher.finalize(&mut output);
    output
}

pub fn selector(signature: &str) -> [u8; 4] {
    let hash = keccak256(signature.as_bytes());
    hash[..4].try_into().unwrap()
}

pub fn push_value(code: &mut Vec<u8>, value: &[u8]) {
    let n = value.len();
    assert!(
        (1..=32).contains(&n),
        "PUSH value must be 1-32 bytes, got {}",
        n
    );
    code.push(0x5f + n as u8);
    code.extend_from_slice(value);
}

pub fn push_u64(code: &mut Vec<u8>, value: u64) {
    if value == 0 {
        push_value(code, &[0x00]);
        return;
    }
    let bytes = value.to_be_bytes();
    let first_nonzero = bytes.iter().position(|&b| b != 0).unwrap();
    push_value(code, &bytes[first_nonzero..]);
}
