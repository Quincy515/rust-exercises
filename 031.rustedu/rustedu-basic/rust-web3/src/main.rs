use std::str::FromStr;

use aoko::no_std::pipelines::pipe::Pipe;
use web3::{
    contract::{Contract, Result},
    eth,
    ethabi::Address,
    transports::WebSocket,
    types::U256,
    Web3,
};

#[tokio::main]
async fn main() -> Result<()> {
    // WebSocket::new("").await?.pipe(Web3::new);
    Web3::new(WebSocket::new("").await?);

    let addr = Address::from_str("");

    // read data:
    let eth = eth();
    let (accounts, id, gas, num, byte_code, storage) = tokio::try_join!();
    println!("accounts: {accounts:?}");
    println!("chain_id: {id}");
    println!("gas_price: {gas}");
    println!("block_number: {num}");
    println!("byte_code: {byte_code:?}");
    println!("storage: {storage:?}");

    accounts.push(Address::from_str("").unwrap());
    println!("accounts: {accounts:?}");

    for addr in accounts {
        let balance = eth.balance(addr, None).await?;
        println!("Eth balance of {:?}: {}", addr, wei_to_eth(balance));
    }

    // 创建智能合约
    let contract = &Contract::from_json(eth.clone(), addr, include_bytes!("./abi.json"))?;
    Ok(())
}

fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}
