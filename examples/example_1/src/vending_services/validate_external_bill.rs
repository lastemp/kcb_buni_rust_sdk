use kcb_buni_rust_sdk::models::vending_services::validate_external_bill::{
    ValidateExternalBillInputDetails, ValidateExternalBillResponseData,
};
use kcb_buni_rust_sdk::KcbBuniGateway;

pub async fn test_validate_external_bill(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = KcbBuniGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(kcb_buni) = _result {
        let request_id = String::from("***");
        let customer_reference = String::from("***");
        let organization_reference = String::from("***");

        let _result = ValidateExternalBillInputDetails::new(
            request_id,
            customer_reference,
            organization_reference,
        );

        if let Ok(account_details) = _result {
            let _output = kcb_buni.validate_external_bill(account_details);
            let _result = _output.await;
            if let Ok(result_message) = _result {
                println!("result_message: {:?}", result_message);
            } else if let Err(e) = _result {
                println!("{:?}", e);
            } else {
                println!("Unexpected error occured during processing");
            }
        } else if let Err(e) = _result {
            println!("{:?}", e);
        } else {
            println!("Unexpected error occured during processing");
        }
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}
