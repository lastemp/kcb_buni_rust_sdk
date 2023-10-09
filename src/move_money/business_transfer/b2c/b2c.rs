use reqwest::StatusCode;

use crate::{
    models::move_money::business_transfer::b2c::b2c::{
        BusinessTransferInputDetails, BusinessTransferResponseData, ErrorResponseData,
    },
    util::util::{build_b2c_business_transfer_data, build_headers},
};

pub async fn transfer(
    account_details: BusinessTransferInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<
    (
        Option<BusinessTransferResponseData>,
        Option<ErrorResponseData>,
    ),
    String,
> {
    let header_details = account_details.get_header();
    let request_payload = account_details.get_request_payload();
    let transaction_info_details = request_payload.get_transaction_info();
    let additional_details = request_payload.get_additional_details();
    let optional_details = request_payload.get_optional_details();

    // Lets build the request params as a struct
    let transfer_data = build_b2c_business_transfer_data(
        header_details,
        transaction_info_details,
        additional_details,
        optional_details,
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
                match response.json::<BusinessTransferResponseData>().await {
                    Ok(transfer_response_data) => {
                        // Handle success case

                        let my_output = (Some(transfer_response_data), None);

                        return Ok(my_output);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            // 400-BAD_REQUEST, 401-UNAUTHORIZED
            StatusCode::BAD_REQUEST | StatusCode::UNAUTHORIZED => {
                match response.json::<ErrorResponseData>().await {
                    Ok(transfer_error_response_data) => {
                        // Handle success case

                        let my_output = (None, Some(transfer_error_response_data));
                        return Ok(my_output);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            s => {
                match response.json::<ErrorResponseData>().await {
                    Ok(transfer_error_response_data) => {
                        // Handle success case

                        let my_output = (None, Some(transfer_error_response_data));
                        return Ok(my_output);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
                /*
                let mut _x = String::from("Request failed processing, status code: ");
                _x.push_str(&s.to_string());
                return Err(_x.to_string());
                */
            }
        },
    };
}
