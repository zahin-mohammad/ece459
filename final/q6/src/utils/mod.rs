use chrono::Utc;
use std::thread::sleep;
use std::time::Duration;
use rand::{thread_rng, Rng};
use crate::utils::types::FraudResult;

pub mod types;
pub mod invoice;

pub fn check_vat_number(vat_number: String) -> bool {
    // Pretend we are calling some external service
    sleep(Duration::from_millis(80));
    return if vat_number.is_empty() {
        false
    } else {
        thread_rng().gen_bool(0.9)
    }
}

pub fn check_for_fraud(company_id: u64) -> FraudResult {
    // Yep, some really hard computation going on
    sleep(Duration::from_millis(250));
    return FraudResult {
        company_id: company_id,
        datetime: Utc::now(),
        fraud_suspected: thread_rng().gen_bool(0.05)
    }
}

pub fn upload_fraud_result(fraud_result: FraudResult) -> bool {
    if !fraud_result.fraud_suspected {
        panic!("Trying to upload fraud result where it's not needed!")
    }
    // HTTP POST or something
    sleep(Duration::from_millis(50));
    true // This went fine
}

pub fn get_vat_number_for_company(_company_id: u64) -> Result<String, String> {
    Ok(String::from("DE1234567"))
}