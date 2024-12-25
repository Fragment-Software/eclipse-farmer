use base64::{prelude::BASE64_STANDARD, Engine};
use fake::{
    faker::{
        internet::en::DomainSuffix,
        lorem::{en::Sentence, zh_tw::Word},
    },
    Fake,
};
use rand::Rng;
use reqwest::Proxy;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::VersionedTransaction,
};

use crate::onchain::eclipse::common::tx::send_and_confirm_tx;

use super::{api::get_create_nft_tx, schema::CollectionsBody};

fn generate_random_url() -> String {
    let domain: String = Word().fake();
    let tld: String = DomainSuffix().fake();

    format!("https://{domain}.{tld}")
}

pub async fn create_collection(
    provider: &RpcClient,
    wallet: &Keypair,
    proxy: Option<&Proxy>,
) -> eyre::Result<()> {
    let account = wallet.pubkey().to_string();

    let name: String = Word().fake();
    let image = generate_random_url();
    let description: String = Sentence(10..20).fake();
    let external_url = generate_random_url();

    let soulbound = rand::thread_rng().gen_bool(0.5);
    let transferable = rand::thread_rng().gen_bool(0.5);
    let burnable = rand::thread_rng().gen_bool(0.5);

    let body = CollectionsBody::default()
        .account(&account)
        .name(&name)
        .image(&image)
        .description(&description)
        .external_url(&external_url)
        .soulbound(soulbound)
        .transferable(transferable)
        .burnable(burnable);

    let response = get_create_nft_tx(body, proxy).await?;

    let tx_base64 = response.transaction;

    let tx_bytes = BASE64_STANDARD.decode(&tx_base64)?;

    let mut tx = bincode::deserialize::<VersionedTransaction>(&tx_bytes)?;

    let new_signature: Signature = wallet.sign_message(&tx.message.serialize());
    tx.signatures[0] = new_signature;

    send_and_confirm_tx(provider, tx).await?;

    Ok(())
}
