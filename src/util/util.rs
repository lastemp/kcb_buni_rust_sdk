use crate::models::move_money::funds_transfer::funds_transfer::FundsTransferData;
use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

pub fn build_headers(access_token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", access_token.parse().unwrap());

    headers
}

pub fn build_account_funds_transfer_data(
    beneficiary_details: String,
    company_code: String,
    credit_account_number: String,
    _currency: String,
    debit_account_number: String,
    debit_amount: u32,
    payment_details: String,
    transaction_reference: String,
    transaction_type: String,
    beneficiary_bank_code: String,
) -> FundsTransferData {
    FundsTransferData {
        beneficiaryDetails: beneficiary_details,
        companyCode: company_code,
        creditAccountNumber: credit_account_number,
        currency: _currency,
        debitAccountNumber: debit_account_number,
        debitAmount: debit_amount,
        paymentDetails: payment_details,
        transactionReference: transaction_reference,
        transactionType: transaction_type,
        beneficiaryBankCode: beneficiary_bank_code,
    }
}

pub fn build_headers_generate_auth_token(api_key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", api_key.parse().unwrap());

    headers
}
