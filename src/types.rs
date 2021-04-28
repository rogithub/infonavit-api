use serde::{Deserialize, Serialize};
use rocket::request::FromForm;

#[derive(FromForm)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Credit {
    pub id: u64,
    pub credit_number: u64,
    pub start_date: i64,
    pub credit_name: String,
    pub interest_rate_percent: f64,
    pub montly_payment_by_employer: f64,
    pub montly_payment_by_employee: f64,
    pub total_credit_amount: f64,
    pub years: i32,
}

#[derive(FromForm)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub id: u64,
    pub credit_id: u64,
    pub payment_date: i64,
    pub number: usize,
    pub is_payment_on_time: bool,
    pub is_via_payrol: bool,
    pub amount: f64,
    pub document_id: Option<u64>,
    pub comments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: u64,
    pub document: Vec<u8>,
    pub file_name: String,
    pub created_date: i64,
}

