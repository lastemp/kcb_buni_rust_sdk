use kcb_buni_rust_sdk::models::vending_services::transaction_status::{
    PayloadDetails, TransactionStatusInputDetails, TransactionStatusResponseData,
};
use kcb_buni_rust_sdk::KcbBuniGateway;

pub async fn test_enquire_transaction_status(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = KcbBuniGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(kcb_buni) = _result {
        let originator_request_id = String::from("***");

        let _result = PayloadDetails::new(originator_request_id);

        if let Ok(payload_details) = _result {
            let _result = TransactionStatusInputDetails::new(payload_details);

            if let Ok(account_details) = _result {
                let _output = kcb_buni.enquire_transaction_status(account_details);
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
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}
