# funds_transfer

Funds Transfer API will enable you to transfer funds from your own KCB Bank account to another Bank account(s).
The API supports Internal Funds Transfer, Transfer to other banks via Pesalink, RTGS, EFT.

## main.rs

This should contain below code:

```rust
pub mod move_money {
    pub mod funds_transfer {
        pub mod funds_transfer;
    }
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

    let x = move_money::funds_transfer::funds_transfer::test_funds_transfer(
        consumer_key,
        consumer_secret,
        _env,
    );
	
    x.await;
}
```

## funds_transfer.rs

This module contains the function test_funds_transfer:

```rust
use kcb_buni_rust_sdk::models::move_money::funds_transfer::funds_transfer::{
    AccountFundsTransferResponseData, FundsTransferInputDetails, UnauthorizedErrorResponseData,
};
use kcb_buni_rust_sdk::KcbBuniGateway;

pub async fn test_funds_transfer(consumer_key: String, consumer_secret: String, _env: String) {
    let _result = KcbBuniGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(kcb_buni) = _result {
        let company_code = String::from("***");
        let transaction_type = String::from("***");
        let debit_account_number = String::from("***");
        let credit_account_number = String::from("***");
        let debit_amount: u32 = 100;
        let payment_details = String::from("***");
        let transaction_reference = String::from("***");
        let _currency = String::from("***");
        let beneficiary_details = String::from("***");
        let beneficiary_bank_code = String::from("***");

        let _result = FundsTransferInputDetails::new(
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

        if let Ok(account_details) = _result {
            let _output = kcb_buni.funds_transfer(account_details);
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
```
