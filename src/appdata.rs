use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use aes::Aes128;
use aes::cipher::{BlockEncrypt, BlockDecrypt, KeyInit};
use aes::cipher::generic_array::GenericArray;
use uqoin_core::utils::U256;
use uqoin_core::seed::{Seed, Mnemonic};
use uqoin_core::schema::Schema;
use uqoin_core::transaction::Transaction;

use crate::utils::str_to_bytes;


#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct AppData {
    seed: U256,
    wallets_map: HashMap<String, U256>,
    wallets_seq: Vec<String>,
    validators: Vec<String>,
}


impl AppData {
    pub fn new(seed: U256) -> Self {
        Self {
            seed,
            wallets_map: HashMap::new(),
            wallets_seq: Vec::new(),
            validators: Vec::new(),
        }
    }
}


#[wasm_bindgen]
impl AppData {
    pub fn createEmpty() -> Self {
        Self::new(U256::from(0))
    }

    pub fn createRandom() -> Self {
        let mut rng = rand::rng();
        let seed = Seed::random(&mut rng);
        Self::new(seed.value())
    }

    pub fn fromMnemonic(phrase: String) -> Self {
        let words = phrase.split_whitespace()
                          .map(|s| s.to_string())
                          .collect::<Vec<String>>();
        let mnemonic: Mnemonic = words.try_into().unwrap();
        let seed = Seed::from_mnemonic(&mnemonic);
        Self::new(seed.value())
    }

    pub fn load(encrypted: &[u8], password: &str) -> Option<Self> {
        // Initialize cipher with the password
        let key = GenericArray::from(str_to_bytes::<16>(password));
        let cipher = Aes128::new(&key);

        // Prepare blocks to decrypt
        let blocks: Vec<_> = encrypted.chunks(16)
            .map(|chunk| GenericArray::from(
                TryInto::<[u8; 16]>::try_into(chunk).unwrap()
            )).collect();

        // Decrypt the blocks
        let mut decrypted = blocks.clone();
        cipher.decrypt_blocks(&mut decrypted);

        // Concatenate data
        let data = String::from_utf8(decrypted.concat())
            .unwrap_or("".to_string());

        // If data was not decoded with UTF-8, likely the password is wrong
        if !data.is_empty() {
            // Deserialize the structure
            if let Ok(instance) = serde_json::from_str(&data.trim_end_matches('\0')) {
                Some(instance)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn dump(&self, password: &str) -> Vec<u8> {
        // Initialize cipher with the password
        let key = GenericArray::from(str_to_bytes::<16>(password));
        let cipher = Aes128::new(&key);

        // Prepare data as bytes
        let mut data: Vec<u8> = serde_json::to_string(self).unwrap()
            .bytes().collect();
        let size = data.len().next_multiple_of(16);
        data.resize(size, 0);

        // Prepare blocks to encrypt
        let blocks: Vec<_> = data.chunks(16)
            .map(|chunk| GenericArray::from(
                TryInto::<[u8; 16]>::try_into(chunk).unwrap()
            )).collect();

        // Encrypt the blocks
        let mut encrypted = blocks.clone();
        cipher.encrypt_blocks(&mut encrypted);

        // Return bytes
        encrypted.concat()
    }

    pub fn isEmpty(&self) -> bool {
        self.seed == U256::from(0)
    }

    pub fn fromJSON(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }

    pub fn toJSON(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn getMnemonic(&self) -> String {
        let seed = Seed::from_value(&self.seed);
        let mnemonic = seed.mnemonic();
        mnemonic.join(" ")
    }

    pub fn getNodes(&self) -> Vec<String> {
        self.validators.clone()
    }

    pub fn addNode(&mut self, node: &str) -> bool {
        if self.validators.iter().position(|n| n == node).is_none() {
            self.validators.push(node.to_string());
            true
        } else {
            false
        }
    }

    pub fn removeNode(&mut self, node: &str) -> bool {
        if let Some(pos) = self.validators.iter().position(|n| n == node) {
            self.validators.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn moveNode(&mut self, node: &str, pos: usize) -> bool {
        if self.removeNode(node) {
            self.validators.insert(pos, node.to_string());
            true
        } else {
            false
        }
    }

    pub fn moreWallets(&mut self, count: usize) {
        let schema = Schema::new();
        let seed = Seed::from_value(&self.seed);
        for key in seed.gen_keys(&schema).skip(self.wallets_seq.len())
                       .take(count) {
            let public = schema.get_public(&key).to_hex();
            self.wallets_seq.push(public.clone());
            self.wallets_map.insert(public, key);
        }
    }

    pub fn getWallets(&self) -> Vec<String> {
        self.wallets_seq.clone()
    }

    pub fn getWalletKey(&self, wallet: &str) -> Option<String> {
        self.wallets_map.get(wallet).map(|key| key.to_hex())
    }

    pub fn sign(&self, wallet: &str, addr: &str, coin: &str, 
                counter: u32) -> String {
        let key = &self.wallets_map[wallet];
        let hash = Transaction::calc_msg(&U256::from_hex(coin), 
                                         &U256::from_hex(addr), 
                                         counter as u64);
        let schema = Schema::new();
        let mut rng = rand::rng();
        let (sign_r, sign_s) = schema.build_signature(&mut rng, &hash, key);
        format!("{}{}", sign_r.to_hex(), sign_s.to_hex())
    }
}
