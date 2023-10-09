/*
use crate::models::move_money::business_transfer::business_transfer::{
    AdditionalDetails, AdditionalDetailsData, BusinessTransferData, HeaderData, HeaderDetails,
    OptionalDetails, OptionalDetailsData, RequestPayloadData, TransactionInfoData,
    TransactionInfoDetails,
};
*/
use crate::models::move_money::funds_transfer::funds_transfer::FundsTransferData;
use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

pub fn build_headers(access_token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", access_token.parse().unwrap());

    headers
}

pub fn build_account_funds_transfer_data(
    company_code: String,
    transaction_type: String,
    debit_account_number: String,
    credit_account_number: String,
    debit_amount: u32,
    payment_details: String,
    transaction_reference: String,
    _currency: String,
    beneficiary_details: String,
    beneficiary_bank_code: String,
) -> FundsTransferData {
    FundsTransferData {
        companyCode: company_code,
        transactionType: transaction_type,
        debitAccountNumber: debit_account_number,
        creditAccountNumber: credit_account_number,
        debitAmount: debit_amount,
        paymentDetails: payment_details,
        transactionReference: transaction_reference,
        currency: _currency,
        beneficiaryDetails: beneficiary_details,
        beneficiaryBankCode: beneficiary_bank_code,
    }
}

pub fn build_b2b_business_transfer_data(
    header_details: &crate::models::move_money::business_transfer::b2b::b2b::HeaderDetails,
    transaction_info_details: &crate::models::move_money::business_transfer::b2b::b2b::TransactionInfoDetails,
    additional_details: &crate::models::move_money::business_transfer::b2b::b2b::AdditionalDetails,
    optional_details: &crate::models::move_money::business_transfer::b2b::b2b::OptionalDetails,
) -> crate::models::move_money::business_transfer::b2b::b2b::BusinessTransferData {
    let _header = crate::models::move_money::business_transfer::b2b::b2b::HeaderData {
        messageID: header_details.get_message_id(),
        featureCode: header_details.get_feature_code(),
        serviceCode: header_details.get_service_code(),
        serviceSubCategory: header_details.get_service_sub_category(),
        minorServiceVersion: header_details.get_minor_service_version(),
        routeCode: header_details.get_route_code(),
        timestamp: header_details.get_timestamp(),
    };

    let transaction_info =
        crate::models::move_money::business_transfer::b2b::b2b::TransactionInfoData {
            transactionID: transaction_info_details.get_transaction_id(),
            transactionType: transaction_info_details.get_transaction_type(),
        };
    let additional_details =
        crate::models::move_money::business_transfer::b2b::b2b::AdditionalDetailsData {
            keyOwner: additional_details.get_key_owner(),
            initiatorIdentifierType: additional_details.get_initiator_identifier_type(),
            initiatorIdentifier: additional_details.get_initiator_identifier(),
            initiatorSecurityCredentials: additional_details.get_initiator_security_credentials(),
            initiatorShortCode: additional_details.get_initiator_short_code(),
            receiverIdentifierType: additional_details.get_receiver_identifier_type(),
            receiverIdentifier: additional_details.get_receiver_identifier(),
            billerReferenceNo: additional_details.get_biller_reference_no(),
            amount: additional_details.get_amount().to_string(),
            currency: additional_details.get_currency(),
        };
    let optional_details =
        crate::models::move_money::business_transfer::b2b::b2b::OptionalDetailsData {
            customerName: optional_details.get_customer_name(),
            customerMSISDN: optional_details.get_customer_msisdn(),
            transactionNarration: optional_details.get_transaction_narration(),
        };
    let request_payload =
        crate::models::move_money::business_transfer::b2b::b2b::RequestPayloadData {
            transactionInfo: transaction_info,
            additionalDetails: additional_details,
            optionalDetails: optional_details,
        };

    crate::models::move_money::business_transfer::b2b::b2b::BusinessTransferData {
        header: _header,
        requestPayload: request_payload,
    }
}

pub fn build_b2c_business_transfer_data(
    header_details: &crate::models::move_money::business_transfer::b2c::b2c::HeaderDetails,
    transaction_info_details: &crate::models::move_money::business_transfer::b2c::b2c::TransactionInfoDetails,
    additional_details: &crate::models::move_money::business_transfer::b2c::b2c::AdditionalDetails,
    optional_details: &crate::models::move_money::business_transfer::b2c::b2c::OptionalDetails,
) -> crate::models::move_money::business_transfer::b2c::b2c::BusinessTransferData {
    let _header = crate::models::move_money::business_transfer::b2c::b2c::HeaderData {
        messageID: header_details.get_message_id(),
        featureCode: header_details.get_feature_code(),
        serviceCode: header_details.get_service_code(),
        serviceSubCategory: header_details.get_service_sub_category(),
        minorServiceVersion: header_details.get_minor_service_version(),
        routeCode: header_details.get_route_code(),
        timestamp: header_details.get_timestamp(),
    };

    let transaction_info =
        crate::models::move_money::business_transfer::b2c::b2c::TransactionInfoData {
            transactionID: transaction_info_details.get_transaction_id(),
            transactionType: transaction_info_details.get_transaction_type(),
        };
    let additional_details =
        crate::models::move_money::business_transfer::b2c::b2c::AdditionalDetailsData {
            keyOwner: additional_details.get_key_owner(),
            initiatorIdentifierType: additional_details.get_initiator_identifier_type(),
            initiatorIdentifier: additional_details.get_initiator_identifier(),
            initiatorSecurityCredentials: additional_details.get_initiator_security_credentials(),
            initiatorShortCode: additional_details.get_initiator_short_code(),
            receiverIdentifierType: additional_details.get_receiver_identifier_type(),
            receiverIdentifier: additional_details.get_receiver_identifier(),
            amount: additional_details.get_amount().to_string(),
            currency: additional_details.get_currency(),
        };
    let optional_details =
        crate::models::move_money::business_transfer::b2c::b2c::OptionalDetailsData {
            customerName: optional_details.get_customer_name(),
            customerMSISDN: optional_details.get_customer_msisdn(),
            transactionNarration: optional_details.get_transaction_narration(),
        };
    let request_payload =
        crate::models::move_money::business_transfer::b2c::b2c::RequestPayloadData {
            transactionInfo: transaction_info,
            additionalDetails: additional_details,
            optionalDetails: optional_details,
        };

    crate::models::move_money::business_transfer::b2c::b2c::BusinessTransferData {
        header: _header,
        requestPayload: request_payload,
    }
}

pub fn build_headers_generate_auth_token(api_key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", api_key.parse().unwrap());

    headers
}
