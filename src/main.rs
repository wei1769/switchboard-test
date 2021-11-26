use switchboard_program;
use solana_sdk::{self, account_info::AccountInfo};
use solana_client::rpc_client;
mod util ;
use hex::{self} ;
fn main() {
    let connection = rpc_client::RpcClient::new("https://raydium.genesysgo.net/".to_string());
    let pubkey = util::get_pub("CppyF6264uKZkGua1brTUa2fSVdMFSCszwzDs76HCuzU");
    let mut  switch_price_account_info = connection.get_account(&(pubkey)).unwrap();
    let mut  lamports = switch_price_account_info.lamports;
    let account_info = AccountInfo::new(&pubkey, false, true,&mut lamports, &mut switch_price_account_info.data, &switch_price_account_info.owner, false, switch_price_account_info.rent_epoch);
    let state = switchboard_program::get_aggregator((&account_info)).unwrap();
    let round_result = switchboard_program::get_aggregator_result(
        &state
    ).unwrap();
    
    println!(" {:?}\n{:?}",hex::encode( switch_price_account_info.data),hex::encode(round_result.round_open_slot.unwrap().to_be_bytes()) );
}
