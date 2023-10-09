use reqwest::StatusCode;

use crate::{
    models::move_money::funds_transfer::funds_transfer::{
        AccountFundsTransferResponseData, FundsTransferInputDetails, UnauthorizedErrorResponseData,
    },
    util::util::{build_account_funds_transfer_data, build_headers},
};

pub async fn transfer(
    account_details: FundsTransferInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<
    (
        Option<AccountFundsTransferResponseData>,
        Option<UnauthorizedErrorResponseData>,
    ),
    String,
> {
    let company_code = account_details.get_company_code();
    let transaction_type = account_details.get_transaction_type();
    let debit_account_number = account_details.get_debit_account_number();
    let credit_account_number = account_details.get_credit_account_number();
    let debit_amount = account_details.get_debit_amount();
    let payment_details = account_details.get_payment_details();
    let transaction_reference = account_details.get_transaction_reference();
    let _currency = account_details.get_currency();
    let beneficiary_details = account_details.get_beneficiary_details();
    let beneficiary_bank_code = account_details.get_beneficiary_bank_code();

    // Lets build the request params as a struct
    let transfer_data = build_account_funds_transfer_data(
        company_code,
        transaction_type,
        debit_account_number,
        credit_account_number,
        debit_amount,
        payment_details,
        transaction_reference,
        _currency,
        beneficiary_details,
        beneficiary_bank_code,
    );

    let client = reqwest::Client::new();

    let res = client
        .post(api_url)
        .headers(build_headers(access_token))
        .json(&transfer_data)
        .send()
        .await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => match response.status() {
            // 200-OK, 400-BAD_REQUEST, 403-FORBIDDEN, 409-CONFLICT
            StatusCode::OK
            | StatusCode::BAD_REQUEST
            | StatusCode::FORBIDDEN
            | StatusCode::CONFLICT => {
                match response.json::<AccountFundsTransferResponseData>().await {
                    Ok(account_transfer_response_data) => {
                        // Handle success case

                        let my_output = (Some(account_transfer_response_data), None);

                        return Ok(my_output);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            StatusCode::UNAUTHORIZED => {
                match response.json::<UnauthorizedErrorResponseData>().await {
                    Ok(account_transfer_error_response_data) => {
                        // Handle success case

                        let my_output = (None, Some(account_transfer_error_response_data));
                        return Ok(my_output);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            s => {
                let mut _x = String::from("Request failed processing, status code: ");
                _x.push_str(&s.to_string());
                return Err(_x.to_string());
            }
        },
    };
}
