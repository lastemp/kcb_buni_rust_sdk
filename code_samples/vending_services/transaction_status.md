# transaction_status

This API is used for checking status of a transaction from KCB Systems. This API call is initiated from your system to KCB system.

## main.rs

This should contain below code:

```rust
pub mod vending_services {
    pub mod transaction_status;
}

// SANDBOX
const CONSUMER_KEY_SANDBOX: &str = "***";
const CONSUMER_SECRET_SANDBOX: &str = "***";

const ENVIRONMENT: &str = "sandbox";

#[tokio::main]
async fn main() {
    let consumer_key = CONSUMER_KEY_SANDBOX.to_string();
    let consumer_secret = CONSUMER_SECRET_SANDBOX.to_string();
    let _env = ENVIRONMENT.to_string();

    let x = vending_services::transaction_status::test_enquire_transaction_status(
        consumer_key,
        consumer_secret,
        _env,
    );
	
    x.await;
}
```

## funds_transfer.rs

This module contains the function test_enquire_transaction_status:

```rust
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
```
