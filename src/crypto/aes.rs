use aes::Aes256;
use bytes::{Bytes, BytesMut};
use cipher::{block_padding::Pkcs7, BlockEncryptMut};
use cipher::{BlockDecryptMut, KeyInit};

use crate::error::CryptoError;
use crate::random_32_bytes;

type PaddingMode = Pkcs7;

type AesEncryptor = ecb::Encryptor<Aes256>;
type AesDecryptor = ecb::Decryptor<Aes256>;

const BLOCK_SIZE: usize = 32;

pub fn generate_aes_encryption_token() -> Bytes {
    random_32_bytes()
}

pub fn encrypt_with_aes(
    encryption_token: &Bytes,
    target: &mut BytesMut,
) -> Result<BytesMut, CryptoError> {
    let original_len = target.len();
    let padding_len = (original_len / BLOCK_SIZE + 1) * BLOCK_SIZE;
    target.extend(vec![0u8; padding_len - original_len]);
    let enc = AesEncryptor::new(encryption_token[..].into());
    let result = enc
        .encrypt_padded_mut::<PaddingMode>(target.as_mut(), original_len)
        .map(BytesMut::from)
        .map_err(|e| CryptoError::Other(format!("{e:?}")))?;
    Ok(result)
}

pub fn decrypt_with_aes(
    encryption_token: &Bytes,
    target: &mut BytesMut,
) -> Result<BytesMut, CryptoError> {
    let dec = AesDecryptor::new(encryption_token[..].into());
    let result = dec
        .decrypt_padded_mut::<PaddingMode>(target.as_mut())
        .map(BytesMut::from)
        .map_err(|e| CryptoError::Other(format!("{e:?}")))?;
    Ok(result)
}

#[test]
fn test() -> Result<(), CryptoError> {
    let encryption_token = random_32_bytes();
    let mut target = BytesMut::from_iter(
        "hello world! this is my plaintext888888888888888."
            .as_bytes()
            .to_vec(),
    );
    encrypt_with_aes(&encryption_token, &mut target)?;
    println!("Encrypt result: [{:?}]", String::from_utf8_lossy(&target));
    let mut encrypted_target = BytesMut::from_iter(target.to_vec());
    let descrypted_result = decrypt_with_aes(&encryption_token, &mut encrypted_target)?;
    println!(
        "Decrypted result: [{:?}]",
        String::from_utf8_lossy(&descrypted_result)
    );
    Ok(())
}
