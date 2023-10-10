use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct ValidateExternalBillInputDetails {
    request_id: String,
    customer_reference: String,
    organization_reference: String,
}

impl ValidateExternalBillInputDetails {
    pub fn new(
        request_id: String,
        customer_reference: String,
        organization_reference: String,
    ) -> Result<Self, String> {
        if request_id.is_empty() || request_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("request id is empty"));
        }
        // request_id has a max length of 30 characters
        else if request_id.trim().len() > 0 && request_id.trim().len() <= 30 {
            // request_id is valid
        } else {
            return Err(String::from("request id has invalid length"));
        }

        if customer_reference.is_empty() || customer_reference.replace(" ", "").trim().len() == 0 {
            return Err(String::from("customer reference is empty"));
        }
        // customer_reference has a max length of 30 characters
        else if customer_reference.trim().len() > 0 && customer_reference.trim().len() <= 30 {
            // customer_reference is valid
        } else {
            return Err(String::from("customer reference has invalid length"));
        }

        if organization_reference.is_empty()
            || organization_reference.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("organization reference is empty"));
        }
        // organization_reference has a max length of 30 characters
        else if organization_reference.trim().len() > 0
            && organization_reference.trim().len() <= 30
        {
            // organization_reference is valid
        } else {
            return Err(String::from("organization reference has invalid length"));
        }

        Ok(Self {
            request_id,
            customer_reference,
            organization_reference,
        })
    }
    pub fn get_request_id(&self) -> String {
        let request_id = &self.request_id;
        request_id.to_string()
    }

    pub fn get_customer_reference(&self) -> String {
        let customer_reference = &self.customer_reference;
        customer_reference.to_string()
    }

    pub fn get_organization_reference(&self) -> String {
        let organization_reference = &self.organization_reference;
        organization_reference.to_string()
    }
}

// request data

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct ValidateExternalBillData {
    pub requestId: String,
    pub customerReference: String,
    pub organizationReference: String,
}

// response data

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ValidateExternalBillResponseData {
    pub transactionID: Option<String>,
    pub statusCode: Option<u16>,
    pub statusMessage: Option<String>,
    pub CustomerName: Option<String>,
    pub billType: Option<String>,
    pub currency: Option<String>,
    pub billAmount: Option<String>,
    pub creditAccountIdentifier: Option<String>,
}
