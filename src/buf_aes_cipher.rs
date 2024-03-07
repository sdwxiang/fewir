use aes::{
    cipher::{
        BlockDecrypt, BlockEncrypt, KeyInit
    }, 
    Aes256, 
    Block,
    Block8
};
use sha3::{ Digest, Sha3_256};

pub struct BuffAesCipher {
    buf: Vec<u8>,
    cipher: Aes256,
    cache_size: usize
}

macro_rules! aes_enc_dec{
    (
        $name:ident,
        $enc_dec_block_fn:ident,
        $enc_dec_blocks_fn:ident
    ) => {
        pub fn $name(&mut self, src: &[u8]) -> &[u8] {
            self.buf.clear();
            self.buf.extend_from_slice(src);
    
            let mut block8_chunks = self.buf.chunks_exact_mut(BuffAesCipher::BLOCK8_SIZE);
    
            (&mut block8_chunks).for_each(|buf| {
                let blocks = unsafe {
                    &mut *(buf.as_mut_ptr() as * mut Block8)
                };
                self.cipher.$enc_dec_blocks_fn(blocks)
            });
    
            block8_chunks.into_remainder()
                .chunks_exact_mut(BuffAesCipher::BLOCK_SIZE).for_each(|buf| {
                self.cipher.$enc_dec_block_fn(Block::from_mut_slice(buf));
            });
    
            self.buf.as_slice()
        }
    };
}

impl BuffAesCipher {
    const BLOCK_SIZE: usize = 16;
    const BLOCK8_SIZE: usize = 16 * 8;

    pub fn new(key: &str, cache_size: usize) -> Self {
        let cache_size = if cache_size == 0 { 1024 * 80 } else { cache_size };
        let buf = Vec::<u8>::with_capacity(cache_size);

        let mut hasher = Sha3_256::default();
        hasher.update(key);
        let cipher = Aes256::new(&hasher.finalize());

        Self {
            buf,
            cipher,
            cache_size
        }
    }

    pub fn cache_size(&self) -> usize {
        self.cache_size
    }

    aes_enc_dec!(encrypt, encrypt_block, encrypt_blocks);
    aes_enc_dec!(decrypt, decrypt_block, decrypt_blocks);
}

#[cfg(test)]
mod tests {
    #[test]
    fn buff_aes_cipher_new() {
        super::BuffAesCipher::new("test", 0);
        super::BuffAesCipher::new("test", 1024 * 10);
    }
}