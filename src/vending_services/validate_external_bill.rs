use reqwest::StatusCode;

use crate::{
    models::vending_services::validate_external_bill::{
        ValidateExternalBillInputDetails, ValidateExternalBillResponseData,
    },
    util::util::{build_headers, build_validate_external_bill_data},
};

pub async fn validate(
    account_details: ValidateExternalBillInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<ValidateExternalBillResponseData, String> {
    let request_id = account_details.get_request_id();
    let customer_reference = account_details.get_customer_reference();
    let organization_reference = account_details.get_organization_reference();

    // Lets build the request params as a struct
    let transfer_data =
        build_validate_external_bill_data(request_id, customer_reference, organization_reference);

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
                match response.json::<ValidateExternalBillResponseData>().await {
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
