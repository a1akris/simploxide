use poly1305::{
    Block as MacBlock, Poly1305,
    universal_hash::{KeyInit, UniversalHash},
};
use salsa20::{
    XSalsa20,
    cipher::{Array, KeyIvInit, StreamCipher, typenum::U10},
    hsalsa,
};
use subtle::ConstantTimeEq as _;
use zeroize::{Zeroize as _, Zeroizing};

use super::{Poly1305Tag, SimplexSecretBox, XSalsa20Key, XSalsa20Nonce};

pub struct SecretBox {
    cipher: XSalsa20,
    mac: Poly1305,
    // Partial bytes of Poly1305 block
    mac_tail: Zeroizing<[u8; 16]>,
    mac_tail_len: usize,
}

impl SecretBox {
    /// `poly1305::update` requires fully-filled 16-byte blocks. This helper buffers incomplete
    /// blocks and feeds only complete 16-byte blocks to `update`
    fn update_mac(&mut self, data: &[u8]) {
        let mut pos = 0;

        // Try to fill the existing partial block first.
        if self.mac_tail_len > 0 {
            let rem = std::cmp::min(16 - self.mac_tail_len, data.len());
            self.mac_tail[self.mac_tail_len..self.mac_tail_len + rem].copy_from_slice(&data[..rem]);

            self.mac_tail_len += rem;
            pos = rem;

            if self.mac_tail_len == 16 {
                self.mac
                    .update(&[MacBlock::clone_from_slice(self.mac_tail.as_ref())]);

                self.mac_tail_len = 0;
            }
        }

        // Feed remaining complete 16-byte blocks directly.
        let mut chunks = data[pos..].chunks_exact(16);

        for chunk in &mut chunks {
            self.mac.update(&[MacBlock::clone_from_slice(chunk)]);
        }

        let tail = chunks.remainder();

        if !tail.is_empty() {
            self.mac_tail[..tail.len()].copy_from_slice(tail);
            self.mac_tail_len += tail.len();
        }
    }
}

impl SimplexSecretBox for SecretBox {
    fn init(key: &XSalsa20Key, nonce: &XSalsa20Nonce) -> Self {
        let mut intermediate = hsalsa::<U10>(&Array(*key), &Array([0u8; 16]));
        let mut cipher = XSalsa20::new(&intermediate, &Array(*nonce));
        let mut poly_key = [0u8; 32];

        cipher.apply_keystream(&mut poly_key);
        let mac = Poly1305::new_from_slice(&poly_key).unwrap();

        intermediate.zeroize();
        poly_key.zeroize();

        Self {
            cipher,
            mac,
            mac_tail: Zeroizing::new([0u8; 16]),
            mac_tail_len: 0,
        }
    }

    fn encrypt_chunk(&mut self, chunk: impl AsRef<[u8]>, mut output: impl AsMut<[u8]>) {
        let chunk = chunk.as_ref();
        let output = output.as_mut();

        output[..chunk.len()].copy_from_slice(chunk);
        self.cipher.apply_keystream(&mut output[..chunk.len()]);
        self.update_mac(&output[..chunk.len()]);
    }

    fn decrypt_chunk(&mut self, chunk: impl AsRef<[u8]>, mut output: impl AsMut<[u8]>) {
        let chunk = chunk.as_ref();
        let output = output.as_mut();

        output[..chunk.len()].copy_from_slice(chunk);
        self.cipher.apply_keystream(&mut output[..chunk.len()]);
        self.update_mac(chunk); // MAC over original ciphertext
    }

    fn auth_tag(&mut self) -> Poly1305Tag {
        self.mac
            .clone()
            .compute_unpadded(&self.mac_tail[..self.mac_tail_len])
            .into()
    }

    fn verify_tag(&mut self, in_tag: &Poly1305Tag) -> bool {
        let tag = self.auth_tag();
        tag.as_slice().ct_eq(in_tag.as_slice()).into()
    }
}
