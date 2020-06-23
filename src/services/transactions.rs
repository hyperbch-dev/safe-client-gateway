extern crate reqwest;

use super::base_transaction_service_url;
use crate::models::backend::about::About;
use reqwest::Url;

pub fn get_about() -> String {
    let url_string = format!("{}{}", base_transaction_service_url(), "/about");
    let url = Url::parse(&url_string).unwrap();
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let about: About = serde_json::from_str(&body).unwrap();
    format!(
        "This is an API wrapper for {}, version {}\nNo guarantees in terms of availability.",
        about.name, about.api_version
    )
}

pub fn get_transactions_details(tx_hash: String) -> String {
    // let url_string = format!(
    //     "{}/transactions/{}",
    //     base_transaction_service_url(),
    //     tx_hash
    // );
    // let url = Url::parse(&url_string).unwrap();
    // println!("{}", &url);
    // let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    // println!("{:#}", body);
    // let transaction_dto: TransactionDto = serde_json::from_str(&body).unwrap();
    // let transaction: Transaction = transactionDto.to_transaction();
    // serde_json::to_string(&transaction).unwrap()
    String::new()
}