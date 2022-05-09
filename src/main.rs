use clap::Parser;
use log::*;
use regex::Regex;
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
struct Cli {
    /// path to the output file.
    #[clap(short, long, default_value_t = String::from("/tmp/.secrets"))]
    output: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Cli::parse();
    let shared_config = aws_config::load_from_env().await;
    let client = aws_sdk_secretsmanager::Client::new(&shared_config);
    let mut file = File::create(&args.output).expect("Unable to create file");

    let rg = Regex::new(r"\{\{inject:secretsmanager:(?P<secret_id>.*):SecretString:(?P<json_key>.*)}}").unwrap();
    for (key, value) in env::vars() {
        if let Some(target) = rg.captures(value.as_str()) {
            let secret_id = target.name("secret_id").unwrap().as_str();
            let json_key = target.name("json_key").unwrap().as_str();
            debug!("{}: {} : {}", key, secret_id, json_key);

            match client.get_secret_value().secret_id(secret_id).send().await {
                Ok(resp) => {
                    let secret_string = resp.secret_string().unwrap_or_default();
                    let secret_value: Value = serde_json::from_str(secret_string).unwrap();
                    writeln!(&mut file, "{}={}", key.clone(), secret_value[json_key].as_str().unwrap()).unwrap()
                }
                Err(e) => error!("{}", e),
            }
        }
    }
}
