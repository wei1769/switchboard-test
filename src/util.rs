
   
use solana_sdk::{
    instruction::AccountMeta,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
};
use std::str::FromStr;
pub fn get_pub(pubkey: &str) -> Pubkey {
    Pubkey::from_str(pubkey).unwrap()
}
/* pub fn getkey(public_key: Pubkey, is_signer: bool, is_writable: bool) -> AccountMeta {
    if is_writable {
        AccountMeta::new(public_key, is_signer)
    } else {
        AccountMeta::new_readonly(public_key, is_signer)
    }
}*/