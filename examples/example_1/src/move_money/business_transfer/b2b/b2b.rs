use kcb_buni_rust_sdk::models::move_money::business_transfer::b2b::b2b::{
    AdditionalDetails, BusinessTransferInputDetails, BusinessTransferResponseData,
    ErrorResponseData, HeaderDetails, OptionalDetails, RequestPayloadDetails,
    TransactionInfoDetails,
};
use kcb_buni_rust_sdk::KcbBuniGateway;

pub async fn test_b2b_payments(consumer_key: String, consumer_secret: String, _env: String) {
    let _result = KcbBuniGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(kcb_buni) = _result {
        let message_id = String::from("***");
        let feature_code = String::from("***");
        let service_code = String::from("***");
        let service_sub_category = String::from("***");
        let minor_service_version = String::from("***");
        let route_code = String::from("***");
        let _timestamp = String::from("***");

        let _result = HeaderDetails::new(
            message_id,
            feature_code,
            service_code,
            service_sub_category,
            minor_service_version,
            route_code,
            _timestamp,
        );

        if let Ok(header_details) = _result {
            let transaction_id = String::from("***");
            let transaction_type = String::from("***");

            let _result = TransactionInfoDetails::new(transaction_id, transaction_type);

            if let Ok(transaction_info_details) = _result {
                let key_owner = String::from("***");
                let initiator_identifier_type = String::from("***");
                let initiator_identifier = String::from("***");
                let initiator_security_credentials = String::from("***");
                let initiator_short_code = String::from("***");
                let receiver_identifier_type = String::from("***");
                let receiver_identifier = String::from("***");
                let biller_reference_no = String::from("***");
                let _amount: u32 = 100;
                let _currency = String::from("***");

                let _result = AdditionalDetails::new(
                    key_owner,
                    initiator_identifier_type,
                    initiator_identifier,
                    initiator_security_credentials,
                    initiator_short_code,
                    receiver_identifier_type,
                    receiver_identifier,
                    biller_reference_no,
                    _amount,
                    _currency,
                );

                if let Ok(additional_details) = _result {
                    let customer_name = String::from("***");
                    let customer_msisdn = String::from("***");
                    let transaction_narration = String::from("***");

                    let _result =
                        OptionalDetails::new(customer_name, customer_msisdn, transaction_narration);

                    if let Ok(optional_details) = _result {
                        let _result = RequestPayloadDetails::new(
                            transaction_info_details,
                            additional_details,
                            optional_details,
                        );

                        if let Ok(request_payload_details) = _result {
                            let _result = BusinessTransferInputDetails::new(
                                header_details,
                                request_payload_details,
                            );

                            if let Ok(account_details) = _result {
                                let _output = kcb_buni.b2b_payments(account_details);
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
