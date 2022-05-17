use ethers_providers::{Provider, Http, Middleware};
use ethers_signers::{LocalWallet, Signer};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::prelude::{SignerMiddleware, TransactionRequest, Address};
use std::convert::TryFrom;
use std::str::FromStr;
use color_eyre::{Result, eyre::bail};

const URL: &str = "https://eth.bd.evmos.org:8545";
const HEXKEY: &str = ""; // no 0x prefix

#[tokio::main]
async fn main() -> Result<()> {
    // Instantiate provider
    let provider = Provider::<Http>::try_from(URL)?;
    let chain_id = provider.get_chainid().await?;

    // Instantiate signer
    let wallet: LocalWallet = HEXKEY.parse().unwrap();
    let signer = ethers::signers::Signer::with_chain_id(wallet, chain_id.as_u64());

    // Tx signer address
    let address = signer.address();
    println!("Signer address: {:#x}", address);

    // Signing provider
    let signing_provider = SignerMiddleware::new(provider, signer);

    // Build tx (sends 1 unit to null addr)
    let tx: TypedTransaction = TransactionRequest::new()
        .to(Address::from_str("0x000000000000000000000000000000000000dEaD").unwrap())
        .from(address)
        .value(1).into();

    // Dispatch tx
    println!("Dispatching tx...");
    let dispatched = signing_provider.send_transaction(tx, None).await.unwrap();
    let tx_hash: ethers::core::types::H256 = *dispatched;

    // Await tx dispatched to mempool
    println!("Awaiting dispatched tx {:#x}...", tx_hash);
    let receipt = dispatched.await?;

    // ERROR: This is the error our agents keep encountering. Tx receipt will 
    // not be in mempool, suggesting dropped tx. Eventually, a tx will go 
    // through from the signer address but it will be with a different tx hash.
    if receipt.is_none() {
        bail!("Could not find tx {:#x} in mempool", tx_hash)
    }
        
    println!(
        "confirmed transaction with tx_hash {:#x}",
        receipt.unwrap().transaction_hash
    );

    Ok(())
}