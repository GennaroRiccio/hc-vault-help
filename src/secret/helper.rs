use std::fs;
use std::process::exit;
use vaultrs::client::{Client, VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct Settings {
    vault_addr: String,
    vault_token: String
}
#[derive(Deserialize)]
pub struct Secrets {
    path : String
}
#[derive(Deserialize)]
pub struct Config{
    settings: Settings,
    secrets: Secrets
}

pub fn get_config()-> Config{
    let filename = "config.toml";
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };
    let config: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };
    return config;
}
pub async fn get_vault_connection(config: &Config) -> VaultClient{
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(config.settings.vault_addr.clone())
            .token(config.settings.vault_token.clone())
            .build()
            .unwrap()
    ).unwrap();

    return client;

}
pub  async fn get_vault_kv2_list(path: String, client: VaultClient) ->Vec<String>{
     let list = kv2::list(&client, &path, "").await.unwrap();
     return list;
}