use kcb_buni_rust_sdk::models::move_money::mobile_money::mpesa::mpesa_express::{
    MpesaExpressInputDetails, MpesaExpressResponseData,
};
use kcb_buni_rust_sdk::KcbBuniGateway;

pub async fn test_mpesa_express_stk_push(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = KcbBuniGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(kcb_buni) = _result {
        let phone_number = String::from("***");
        let _amount: u32 = 100;
        let invoice_number = String::from("***");
        let shared_short_code = false;
        let org_short_code = String::from("***");
        let org_passkey = String::from("***");
        let callback_url = String::from("***");
        let transaction_description = String::from("***");

        let _result = MpesaExpressInputDetails::new(
            phone_number,
            _amount,
            invoice_number,
            shared_short_code,
            org_short_code,
            org_passkey,
            callback_url,
            transaction_description,
        );

        if let Ok(account_details) = _result {
            let _output = kcb_buni.mpesa_express_stk_push(account_details);
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
