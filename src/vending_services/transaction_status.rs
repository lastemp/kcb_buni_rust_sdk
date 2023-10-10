use reqwest::StatusCode;

use crate::{
    models::vending_services::transaction_status::{
        TransactionStatusInputDetails, TransactionStatusResponseData,
    },
    util::util::{build_headers, build_transaction_status_data},
};

pub async fn enquire(
    account_details: TransactionStatusInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<TransactionStatusResponseData, String> {
    let _payload = account_details.get_payload();
    let originator_request_id = _payload.get_originator_request_id();

    // Lets build the request params as a struct
    let transfer_data = build_transaction_status_data(originator_request_id);

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
                match response.json::<TransactionStatusResponseData>().await {
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
