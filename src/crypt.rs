use cfb8::cipher::{KeyIvInit, BlockDecryptMut, BlockEncryptMut, InvalidLength, BlockSizeUser};
use cfb8::cipher::consts::U16;
use cfb8::cipher::generic_array::GenericArray;

pub type DecCipher = cfb8::Decryptor<aes::Aes128>;
pub type EncCipher = cfb8::Encryptor<aes::Aes128>;

pub fn encrypt_bytes(cipher: &mut EncCipher, bytes: &mut [u8]) {
	for chunk in bytes.chunks_mut(EncCipher::block_size()) {
		let arr = GenericArray::from_mut_slice(chunk);
		cipher.encrypt_block_mut(arr)
	}
}

pub fn decrypt_bytes(cipher: &mut DecCipher, bytes: &mut [u8]) {
	for chunk in bytes.chunks_mut(DecCipher::block_size()) {
		let arr = GenericArray::from_mut_slice(chunk);
		cipher.decrypt_block_mut(arr)
	}
}