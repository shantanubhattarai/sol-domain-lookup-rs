use sns_sdk::non_blocking::resolve::resolve_owner;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value="bonfida", help="The domain name to lookup.")]
    domain_name: String,
}

//const SOLANA_DEV_RPC: &str = "https://api.devnet.solana.com";
const SOLANA_MAIN_BETA_RPC: &str = "https://api.mainnet-beta.solana.com";

async fn resolve_domain_owner(domain_name: &str) -> Option<Pubkey> {
    if domain_name == "" { return None; }

    let client = RpcClient::new(SOLANA_MAIN_BETA_RPC.to_string());
    let res = resolve_owner(&client, domain_name).await.unwrap();
    res
}

#[tokio::main]
async fn main() {
    let args=Args::parse();
    let domain_name = args.domain_name.as_str();

    match resolve_domain_owner(domain_name).await {
        Some(owner_pubkey) => {println!("Domain already registered by {}.", owner_pubkey)},
        None => {println!("Domain not registered.")}
    }
}

#[cfg(test)]
mod tests {
    use solana_sdk::pubkey;
    use super::*;

    #[tokio::test]
    async fn resolves_domain_owner_correctly() {
        let domain_name = "bonfida";

        assert_eq!(resolve_domain_owner(domain_name).await.unwrap(), pubkey!("HKKp49qGWXd639QsuH7JiLijfVW5UtCVY4s1n2HANwEA"))
    }

    #[tokio::test]
    async fn resolves_empty_domain_correctly(){
        let domain_name = "";
        assert_eq!(resolve_domain_owner(domain_name).await, None);
    }
}