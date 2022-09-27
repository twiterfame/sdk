// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

use aleo_account::{Address, PrivateKey as PrivateKeyNative, RecordCiphertext, ViewKey};

use rand::{rngs::StdRng, SeedableRng};
use std::{convert::TryFrom, str::FromStr};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PrivateKey {
    pub(crate) private_key: PrivateKeyNative,
}

#[wasm_bindgen]
impl PrivateKey {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();

        Self {
            private_key: PrivateKeyNative::new(&mut StdRng::from_entropy()).unwrap(),
        }
    }

    #[wasm_bindgen]
    pub fn from_string(private_key: &str) -> Self {
        let private_key = PrivateKeyNative::from_str(private_key).unwrap();

        Self { private_key }
    }

    #[wasm_bindgen]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.private_key.to_string()
    }

    #[wasm_bindgen]
    pub fn to_view_key(&self) -> String {
        let view_key = ViewKey::try_from(self.private_key).unwrap();
        view_key.to_string()
    }

    #[wasm_bindgen]
    pub fn to_address(&self) -> String {
        let address = Address::try_from(self.private_key).unwrap();
        address.to_string()
    }

    #[wasm_bindgen]
    pub fn decrypt(&self, ciphertext: &str) -> Result<String, String> {
        let view_key = ViewKey::try_from(self.private_key).map_err(|error| error.to_string())?;
        let ciphertext = RecordCiphertext::from_str(ciphertext).map_err(|error| error.to_string())?;
        match ciphertext.decrypt(&view_key) {
            Ok(plaintext) => Ok(plaintext.to_string()),
            Err(_) => Err("Incorrect view key".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    const ITERATIONS: u64 = 1_000;
    const CORRECT_PRIVATE_KEY: &str = "APrivateKey1zkp6ka4UZu9JfMFDBqzKBffX7HQpYneKtzCTwnHb5GXVDpL";
    const INCORRECT_PRIVATE_KEY: &str = "APrivateKey1zkp6zrcYuoiMR6ePdWVLLdFehR8VMZQ2amsb8nqjLfNFanp";

    #[wasm_bindgen_test]
    pub fn test_private_key_new() {
        for _ in 0..ITERATIONS {
            // Generate a new private_key.
            let expected = PrivateKey::new();

            // Check the private_key derived from string.
            assert_eq!(
                expected.to_string(),
                PrivateKey::from_string(&expected.to_string()).to_string()
            );
        }
    }

    #[wasm_bindgen_test]
    pub fn test_address_from_private_key() {
        for _ in 0..ITERATIONS {
            // Sample a new private key.
            let private_key = PrivateKey::new();
            let expected = Address::from_str(&private_key.to_address()).unwrap();

            // Check the private_key derived from the view key.
            let view_key = ViewKey::from_str(&private_key.to_view_key()).unwrap();
            assert_eq!(expected, Address::try_from(&view_key).unwrap());
        }
    }

    #[wasm_bindgen_test]
    pub fn test_decrypt_success() {
        let ciphertext = "record1qyqspplg2ud9gguy8ud9wjmee3cf2vztxcjxe2ernf8m7ru5wvsqkdqxqyqsq7y540qmemqx3675pufewwmywsudzrpstjx3fd38c6d8uz4r4mgpqqqt2q2jjczxp2y6986zdqz3mr5jmhggmge3exc72vgw2kgr4gea2zgzhrz8q";
        let private_key = PrivateKey::from_string(CORRECT_PRIVATE_KEY);
        let plaintext = private_key.decrypt(ciphertext);
        let expected_plaintext = "{owner: aleo1snwe5h89dv6hv2q2pl3v8l9cweeuwrgejmlnwza6ndacygznlu9sjt8pgv.private, gates: 1u64.private, data: {}, _nonce: 4447510634654730534613001085815220248957154008834207042015711498717088580021group.public}";
        assert!(plaintext.is_ok());
        assert_eq!(expected_plaintext, plaintext.unwrap())
    }

    #[wasm_bindgen_test]
    pub fn test_decrypt_fails() {
        let ciphertext = "record1qyqspplg2ud9gguy8ud9wjmee3cf2vztxcjxe2ernf8m7ru5wvsqkdqxqyqsq7y540qmemqx3675pufewwmywsudzrpstjx3fd38c6d8uz4r4mgpqqqt2q2jjczxp2y6986zdqz3mr5jmhggmge3exc72vgw2kgr4gea2zgzhrz8q";
        let incorrect_private_key = PrivateKey::from_string(INCORRECT_PRIVATE_KEY);
        let plaintext = incorrect_private_key.decrypt(ciphertext);
        assert!(plaintext.is_err());
    }
}
