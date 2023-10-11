pub mod move_money {
    pub mod funds_transfer {
        pub mod funds_transfer;
    }
    pub mod business_transfer {
        pub mod b2b {
            pub mod b2b;
        }
        pub mod b2c {
            pub mod b2c;
        }
    }
    pub mod mobile_money {
        pub mod mpesa {
            pub mod mpesa_express;
        }
    }
}
pub mod vending_services {
    pub mod transaction_status;
    pub mod validate_external_bill;
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

    // move_money

    let x = move_money::funds_transfer::funds_transfer::test_funds_transfer(
        consumer_key,
        consumer_secret,
        _env,
    );

    /*
    let x = move_money::business_transfer::b2b::b2b::test_b2b_payments(
        consumer_key,
        consumer_secret,
        _env,
    );

    let x = move_money::business_transfer::b2c::b2c::test_b2c_payments(
        consumer_key,
        consumer_secret,
        _env,
    );

    let x = move_money::mobile_money::mpesa::mpesa_express::test_mpesa_express_stk_push(
        consumer_key,
        consumer_secret,
        _env,
    );
    */

    // vending_services
    /*
    let x = vending_services::transaction_status::test_enquire_transaction_status(
        consumer_key,
        consumer_secret,
        _env,
    );

    let x = vending_services::validate_external_bill::test_validate_external_bill(
        consumer_key,
        consumer_secret,
        _env,
    );
    */

    x.await;
}
