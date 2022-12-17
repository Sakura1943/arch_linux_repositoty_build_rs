pub mod aur;
pub mod official;
pub mod custom;
use super::Lazy;
use reqwest::Client;
pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder().user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36").build().unwrap()
});
