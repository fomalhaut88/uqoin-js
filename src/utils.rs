use wasm_bindgen::prelude::*;
use uqoin_core::utils::{U256, hash_of_u256};
use uqoin_core::coin::{coin_symbol, coin_random, coin_order};


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


/// Represent string as bytes array with fixed size.
pub fn str_to_bytes<const N: usize>(s: &str) -> [u8; N] {
    let bytes = s.as_bytes();
    let size = std::cmp::min(bytes.len(), N);
    let mut buffer = [0u8; N];
    buffer[..size].clone_from_slice(&bytes[..size]);
    buffer
}


/// Get coin symbol by order.
#[wasm_bindgen]
pub fn coinSymbol(order: usize) -> String {
    coin_symbol(order as u64)
}


/// Mine a coin for the specific miner (given as wallet) starting from the order
/// `minOrder` in `attempts` attempts.
#[wasm_bindgen]
pub fn mineCoin(wallet: &str, minOrder: u32, 
                attempts: usize) -> Option<String> {
    let miner = U256::from_hex(wallet);
    let mut rng = rand::rng();
    for _ in 0..attempts {
        let coin = coin_random(&mut rng, &miner);
        if coin_order(&coin, &miner) >= minOrder as u64 {
            return Some(coin.to_hex());
        }
    }
    None
}


/// Get coin order by number and miner.
#[wasm_bindgen]
pub fn coinOrder(coin: &str, miner: &str) -> usize {
    let coin = U256::from_hex(coin);
    let miner = U256::from_hex(miner);
    coin_order(&coin, &miner) as usize
}


/// Get XOR hash of coins.
#[wasm_bindgen]
pub fn coinsXor(coins: Vec<String>) -> String {
    coins.iter().fold(U256::from(0), |x, s| &x ^ &U256::from_hex(s)).to_hex()
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
