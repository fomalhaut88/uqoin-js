use wasm_bindgen::prelude::*;
use uqoin_core::utils::{U256, hash_of_u256};


/// Hash a sequence of U256 numbers given as their concatenated HEX string.
/// The length must be multiple of 64.
#[wasm_bindgen]
pub fn hashOfU256(elems: &str) -> String {
    // Parse string into U256 numbers
    let nums = elems.chars().collect::<Vec<char>>().chunks(64)
        .map(|chunk| U256::from_hex(&chunk.iter().collect::<String>()))
        .collect::<Vec<U256>>();

    // Apply hash and convert it into a HEX string
    hash_of_u256(nums.iter()).to_hex()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashOfU256() {
        let a = U256::from_hex(
            "E7646626CB303A9EEBAAD078ACD5632862232A27EF6426CC7D7A92251FBFEE94"
        );
        let b = U256::from_hex(
            "E7646626CB303A9EEBAAD078ACD56328DC4BFFC745FD5063738D9E10BF513204"
        );

        let elems: String = [a.to_hex(), b.to_hex()].concat();

        let hash = hashOfU256(&elems);

        assert_eq!(
            hash,
            "0000001462535B76AFA05824673FA8A3AEDC030B7D3BB354B1A7463191134609"
        );
    }
}
