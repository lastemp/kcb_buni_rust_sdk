use reqwest::StatusCode;

use crate::{
    models::move_money::mobile_money::mpesa::mpesa_express::{
        MpesaExpressInputDetails, MpesaExpressResponseData,
    },
    util::util::{build_headers, build_mpesa_express_data},
};

pub async fn stk_push(
    account_details: MpesaExpressInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<MpesaExpressResponseData, String> {
    let phone_number = account_details.get_phone_number();
    let _amount = account_details.get_amount();
    let invoice_number = account_details.get_invoice_number();
    let shared_short_code = account_details.get_shared_short_code();
    let org_short_code = account_details.get_org_short_code();
    let org_passkey = account_details.get_org_passkey();
    let callback_url = account_details.get_callback_url();
    let transaction_description = account_details.get_transaction_description();

    // Lets build the request params as a struct
    let transfer_data = build_mpesa_express_data(
        phone_number,
        _amount,
        invoice_number,
        shared_short_code,
        org_short_code,
        org_passkey,
        callback_url,
        transaction_description,
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
            // 200-OK
            StatusCode::OK => {
                match response.json::<MpesaExpressResponseData>().await {
                    Ok(transfer_response_data) => {
                        // Handle success case

                        return Ok(transfer_response_data);
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
