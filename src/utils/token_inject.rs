use std::env;
use dotenv::dotenv;

pub fn get_vercel_api_key() -> String {
    dotenv().ok();
    env::var("VERCEL_API_KEY").expect("VERCEL_API_KEY not set in .env file")
}

pub fn get_namecheap_ddns_token() -> String {
    dotenv().ok();
    env::var("NAMECHEAP_DDNS_TOKEN").expect("NAMECHEAP_DDNS_TOKEN not set in .env file")
}
