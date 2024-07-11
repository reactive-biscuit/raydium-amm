use raydium_amm::amm_info_hack::AmmInfoHack;
use raydium_amm::state::{AmmInfo, Loadable};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;

const RPC_URL: &str = "https://solana-mainnet.g.alchemy.com/v2/anGp1E5KehCx8KNw6DIs8BXgFAeygeDW";
const SOL_USDC_POOL: Pubkey = pubkey!("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2");

fn main() -> anyhow::Result<()> {
    let rpc_client = RpcClient::new(RPC_URL);
    let account = get_account::<AmmInfoHack>(&rpc_client, &SOL_USDC_POOL)?.unwrap();
    println!("Hack: {:#?}", account);
    let account = AmmInfo::from(account);
    println!("1: {}", account.state_data.swap_coin_in_amount);
    println!("2: {}", account.state_data.swap_pc_out_amount);
    println!("3: {}", account.state_data.swap_pc_in_amount);
    println!("4: {}", account.state_data.swap_coin_out_amount);
    Ok(())
}

pub fn get_account<T>(client: &RpcClient, addr: &Pubkey) -> anyhow::Result<Option<T>>
where
    T: Clone + Loadable,
{
    if let Some(account) = client
        .get_account_with_commitment(addr, CommitmentConfig::processed())?
        .value
    {
        let account_data = account.data.as_slice();
        let ret: &T = Loadable::load_from_bytes(account_data)?;
        Ok(Some(ret.clone()))
    } else {
        Ok(None)
    }
}
