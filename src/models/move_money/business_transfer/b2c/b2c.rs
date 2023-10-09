use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct HeaderDetails {
    message_id: String,
    feature_code: String,
    service_code: String,
    service_sub_category: String,
    minor_service_version: String,
    route_code: String,
    _timestamp: String,
}

impl HeaderDetails {
    pub fn new(
        message_id: String,
        feature_code: String,
        service_code: String,
        service_sub_category: String,
        minor_service_version: String,
        route_code: String,
        _timestamp: String,
    ) -> Result<Self, String> {
        if message_id.is_empty() || message_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("message id is empty"));
        }
        // message_id has a max length of 16 characters
        else if message_id.trim().len() >= 2 && message_id.trim().len() <= 16 {
            // message_id is valid
        } else {
            return Err(String::from("message id has invalid length"));
        }

        if feature_code.is_empty() || feature_code.replace(" ", "").trim().len() == 0 {
            return Err(String::from("feature code is empty"));
        }
        // feature_code has a max length of 3 characters
        else if feature_code.trim().len() >= 2 && feature_code.trim().len() <= 3 {
            // feature_code is valid
        } else {
            return Err(String::from("feature code has invalid length"));
        }

        if service_code.is_empty() || service_code.replace(" ", "").trim().len() == 0 {
            return Err(String::from("service code is empty"));
        }
        // service_code has a max length of 4 characters
        else if service_code.trim().len() >= 2 && service_code.trim().len() <= 4 {
            // service_code is valid
        } else {
            return Err(String::from("service code has invalid length"));
        }

        if service_sub_category.is_empty()
            || service_sub_category.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("service sub category is empty"));
        }
        // service_sub_category has a max length of 15 characters
        else if service_sub_category.trim().len() >= 2 && service_sub_category.trim().len() <= 15
        {
            // service_sub_category is valid
        } else {
            return Err(String::from("service sub category has invalid length"));
        }

        if minor_service_version.is_empty()
            || minor_service_version.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("minor service version is empty"));
        }
        // minor_service_version has a max length of 3 characters
        else if minor_service_version.trim().len() >= 2 && minor_service_version.trim().len() <= 3
        {
            // minor_service_version is valid
        } else {
            return Err(String::from("minor service version has invalid length"));
        }

        if route_code.is_empty() || route_code.replace(" ", "").trim().len() == 0 {
            return Err(String::from("route code is empty"));
        }
        // route_code has a max length of 3 characters
        else if route_code.trim().len() >= 2 && route_code.trim().len() <= 3 {
            // route_code is valid
        } else {
            return Err(String::from("route code has invalid length"));
        }

        if _timestamp.is_empty() || _timestamp.replace(" ", "").trim().len() == 0 {
            return Err(String::from("timestamp is empty"));
        }
        // _timestamp has a max length of 15 characters
        else if _timestamp.trim().len() >= 2 && _timestamp.trim().len() <= 15 {
            // _timestamp is valid
        } else {
            return Err(String::from("timestamp has invalid length"));
        }

        Ok(Self {
            message_id,
            feature_code,
            service_code,
            service_sub_category,
            minor_service_version,
            route_code,
            _timestamp,
        })
    }

    pub fn get_message_id(&self) -> String {
        let message_id = &self.message_id;
        message_id.to_string()
    }

    pub fn get_feature_code(&self) -> String {
        let feature_code = &self.feature_code;
        feature_code.to_string()
    }

    pub fn get_service_code(&self) -> String {
        let service_code = &self.service_code;
        service_code.to_string()
    }

    pub fn get_service_sub_category(&self) -> String {
        let service_sub_category = &self.service_sub_category;
        service_sub_category.to_string()
    }

    pub fn get_minor_service_version(&self) -> String {
        let minor_service_version = &self.minor_service_version;
        minor_service_version.to_string()
    }

    pub fn get_route_code(&self) -> String {
        let route_code = &self.route_code;
        route_code.to_string()
    }

    pub fn get_timestamp(&self) -> String {
        let _timestamp = &self._timestamp;
        _timestamp.to_string()
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct TransactionInfoDetails {
    transaction_id: String,
    transaction_type: String,
}

impl TransactionInfoDetails {
    pub fn new(transaction_id: String, transaction_type: String) -> Result<Self, String> {
        if transaction_id.is_empty() || transaction_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("transaction id is empty"));
        }
        // transaction_id has a max length of 15 characters
        else if transaction_id.trim().len() >= 2 && transaction_id.trim().len() <= 15 {
            // transaction_id is valid
        } else {
            return Err(String::from("transaction id has invalid length"));
        }

        if transaction_type.is_empty() || transaction_type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("transaction type is empty"));
        }
        // transaction_type has a max length of 15 characters
        else if transaction_type.trim().len() >= 2 && transaction_type.trim().len() <= 15 {
            // transaction_type is valid
        } else {
            return Err(String::from("transaction type has invalid length"));
        }

        Ok(Self {
            transaction_id,
            transaction_type,
        })
    }

    pub fn get_transaction_id(&self) -> String {
        let transaction_id = &self.transaction_id;
        transaction_id.to_string()
    }

    pub fn get_transaction_type(&self) -> String {
        let transaction_type = &self.transaction_type;
        transaction_type.to_string()
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct AdditionalDetails {
    key_owner: String,
    initiator_identifier_type: String,
    initiator_identifier: String,
    initiator_security_credentials: String,
    initiator_short_code: String,
    receiver_identifier_type: String,
    receiver_identifier: String,
    _amount: u32,
    _currency: String,
}

impl AdditionalDetails {
    pub fn new(
        key_owner: String,
        initiator_identifier_type: String,
        initiator_identifier: String,
        initiator_security_credentials: String,
        initiator_short_code: String,
        receiver_identifier_type: String,
        receiver_identifier: String,
        _amount: u32,
        _currency: String,
    ) -> Result<Self, String> {
        if key_owner.is_empty() || key_owner.replace(" ", "").trim().len() == 0 {
            return Err(String::from("key owner is empty"));
        }
        // key_owner has a max length of 2 characters
        else if key_owner.trim().len() >= 1 && key_owner.trim().len() <= 2 {
            // key_owner is valid
        } else {
            return Err(String::from("key owner has invalid length"));
        }

        if initiator_identifier_type.is_empty()
            || initiator_identifier_type.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("initiator identifier type is empty"));
        }
        // initiator_identifier_type has a max length of 3 characters
        else if initiator_identifier_type.trim().len() >= 2
            && initiator_identifier_type.trim().len() <= 3
        {
            // initiator_identifier_type is valid
        } else {
            return Err(String::from("initiator identifier type has invalid length"));
        }

        if initiator_identifier.is_empty()
            || initiator_identifier.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("initiator identifier is empty"));
        }
        // initiator_identifier has a max length of 30 characters
        else if initiator_identifier.trim().len() >= 2 && initiator_identifier.trim().len() <= 30
        {
            // initiator_identifier is valid
        } else {
            return Err(String::from("initiator identifier has invalid length"));
        }

        if initiator_security_credentials.is_empty()
            || initiator_security_credentials.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("initiator security credentials is empty"));
        }
        // initiator_security_credentials has a max length of 150 characters
        else if initiator_security_credentials.trim().len() >= 2
            && initiator_security_credentials.trim().len() <= 150
        {
            // initiator_security_credentials is valid
        } else {
            return Err(String::from(
                "initiator security credentials has invalid length",
            ));
        }

        if initiator_short_code.is_empty()
            || initiator_short_code.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("initiator short code is empty"));
        }
        // initiator_short_code has a max length of 10 characters
        else if initiator_short_code.trim().len() >= 2 && initiator_short_code.trim().len() <= 10
        {
            // initiator_short_code is valid
        } else {
            return Err(String::from("initiator short code has invalid length"));
        }

        if receiver_identifier_type.is_empty()
            || receiver_identifier_type.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("receiver identifier type is empty"));
        }
        // receiver_identifier_type has a max length of 2 characters
        else if receiver_identifier_type.trim().len() >= 1
            && receiver_identifier_type.trim().len() <= 2
        {
            // receiver_identifier_type is valid
        } else {
            return Err(String::from("receiver identifier type has invalid length"));
        }

        if receiver_identifier.is_empty() || receiver_identifier.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("receiver identifier is empty"));
        }
        // receiver_identifier has a max length of 15 characters
        else if receiver_identifier.trim().len() >= 2 && receiver_identifier.trim().len() <= 15 {
            // receiver_identifier is valid
        } else {
            return Err(String::from("receiver identifier has invalid length"));
        }

        if _amount > 0 {
        } else {
            return Err(String::from("amount has invalid value"));
        }

        if _currency.is_empty() || _currency.replace(" ", "").trim().len() == 0 {
            return Err(String::from("currency is empty"));
        }
        // _currency has a max length of 10 characters
        else if _currency.trim().len() >= 2 && _currency.trim().len() <= 10 {
            // currency is valid
        } else {
            return Err(String::from("currency has invalid length"));
        }

        Ok(Self {
            key_owner,
            initiator_identifier_type,
            initiator_identifier,
            initiator_security_credentials,
            initiator_short_code,
            receiver_identifier_type,
            receiver_identifier,
            _amount,
            _currency,
        })
    }

    pub fn get_key_owner(&self) -> String {
        let key_owner = &self.key_owner;
        key_owner.to_string()
    }

    pub fn get_initiator_identifier_type(&self) -> String {
        let initiator_identifier_type = &self.initiator_identifier_type;
        initiator_identifier_type.to_string()
    }

    pub fn get_initiator_identifier(&self) -> String {
        let initiator_identifier = &self.initiator_identifier;
        initiator_identifier.to_string()
    }

    pub fn get_initiator_security_credentials(&self) -> String {
        let initiator_security_credentials = &self.initiator_security_credentials;
        initiator_security_credentials.to_string()
    }

    pub fn get_initiator_short_code(&self) -> String {
        let initiator_short_code = &self.initiator_short_code;
        initiator_short_code.to_string()
    }

    pub fn get_receiver_identifier_type(&self) -> String {
        let receiver_identifier_type = &self.receiver_identifier_type;
        receiver_identifier_type.to_string()
    }

    pub fn get_receiver_identifier(&self) -> String {
        let receiver_identifier = &self.receiver_identifier;
        receiver_identifier.to_string()
    }

    pub fn get_amount(&self) -> u32 {
        let _amount = &self._amount;
        *_amount
    }

    pub fn get_currency(&self) -> String {
        let _currency = &self._currency;
        _currency.to_string()
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct OptionalDetails {
    customer_name: String,
    customer_msisdn: String,
    transaction_narration: String,
}

impl OptionalDetails {
    pub fn new(
        customer_name: String,
        customer_msisdn: String,
        transaction_narration: String,
    ) -> Result<Self, String> {
        if customer_name.is_empty() || customer_name.replace(" ", "").trim().len() == 0 {
            return Err(String::from("customer name is empty"));
        }
        // customer_name has a max length of 50 characters
        else if customer_name.trim().len() >= 3 && customer_name.trim().len() <= 50 {
            // customer_name is valid
        } else {
            return Err(String::from("customer name has invalid length"));
        }

        if customer_msisdn.is_empty() || customer_msisdn.replace(" ", "").trim().len() == 0 {
            return Err(String::from("customer msisdn is empty"));
        }
        // customer_msisdn has a max length of 15 characters
        else if customer_msisdn.trim().len() >= 2 && customer_msisdn.trim().len() <= 15 {
            // customer_msisdn is valid
        } else {
            return Err(String::from("customer msisdn has invalid length"));
        }

        if transaction_narration.is_empty()
            || transaction_narration.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("transaction narration is empty"));
        }
        // transaction_narration has a max length of 150 characters
        else if transaction_narration.trim().len() >= 2
            && transaction_narration.trim().len() <= 150
        {
            // transaction_narration is valid
        } else {
            return Err(String::from("transaction narration has invalid length"));
        }

        Ok(Self {
            customer_name,
            customer_msisdn,
            transaction_narration,
        })
    }

    pub fn get_customer_name(&self) -> String {
        let customer_name = &self.customer_name;
        customer_name.to_string()
    }

    pub fn get_customer_msisdn(&self) -> String {
        let customer_msisdn = &self.customer_msisdn;
        customer_msisdn.to_string()
    }

    pub fn get_transaction_narration(&self) -> String {
        let transaction_narration = &self.transaction_narration;
        transaction_narration.to_string()
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct RequestPayloadDetails {
    transaction_info: TransactionInfoDetails,
    additional_details: AdditionalDetails,
    optional_details: OptionalDetails,
}

impl RequestPayloadDetails {
    pub fn new(
        transaction_info: TransactionInfoDetails,
        additional_details: AdditionalDetails,
        optional_details: OptionalDetails,
    ) -> Result<Self, String> {
        Ok(Self {
            transaction_info,
            additional_details,
            optional_details,
        })
    }

    pub fn get_transaction_info(&self) -> &TransactionInfoDetails {
        let transaction_info = &self.transaction_info;
        transaction_info
    }

    pub fn get_additional_details(&self) -> &AdditionalDetails {
        let additional_details = &self.additional_details;
        additional_details
    }

    pub fn get_optional_details(&self) -> &OptionalDetails {
        let optional_details = &self.optional_details;
        optional_details
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct BusinessTransferInputDetails {
    _header: HeaderDetails,
    request_payload: RequestPayloadDetails,
}

impl BusinessTransferInputDetails {
    pub fn new(
        _header: HeaderDetails,
        request_payload: RequestPayloadDetails,
    ) -> Result<Self, String> {
        Ok(Self {
            _header,
            request_payload,
        })
    }

    pub fn get_header(&self) -> &HeaderDetails {
        let _header = &self._header;
        _header
    }

    pub fn get_request_payload(&self) -> &RequestPayloadDetails {
        let request_payload = &self.request_payload;
        request_payload
    }
}

// request data

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct HeaderData {
    pub messageID: String,
    pub featureCode: String,
    pub serviceCode: String,
    pub serviceSubCategory: String,
    pub minorServiceVersion: String,
    pub routeCode: String,
    pub timestamp: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct TransactionInfoData {
    pub transactionID: String,
    pub transactionType: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct AdditionalDetailsData {
    pub keyOwner: String,
    pub initiatorIdentifierType: String,
    pub initiatorIdentifier: String,
    pub initiatorSecurityCredentials: String,
    pub initiatorShortCode: String,
    pub receiverIdentifierType: String,
    pub receiverIdentifier: String,
    pub amount: String,
    pub currency: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct OptionalDetailsData {
    pub customerName: String,
    pub customerMSISDN: String,
    pub transactionNarration: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct RequestPayloadData {
    pub transactionInfo: TransactionInfoData,
    pub additionalDetails: AdditionalDetailsData,
    pub optionalDetails: OptionalDetailsData,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct BusinessTransferData {
    pub header: HeaderData,
    pub requestPayload: RequestPayloadData,
}

// response data

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct HeaderResponseData {
    pub messageID: Option<String>,
    pub conversationID: Option<String>,
    pub targetSystemID: Option<String>,
    pub routeCode: Option<String>,
    pub statusCode: Option<String>,
    pub messageCode: Option<String>,
    pub statusDescription: Option<String>,
    pub statusMessage: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TransactionInfoResponseData {
    pub transactionId: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ResponsePayloadData {
    pub transactionInfo: TransactionInfoResponseData,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct BusinessTransferResponseData {
    pub header: HeaderResponseData,
    pub responsePayload: ResponsePayloadData,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ErrorResponseData {
    pub code: Option<String>,
    pub message: Option<String>,
}
