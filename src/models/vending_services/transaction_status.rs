use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct PayloadDetails {
    originator_request_id: String,
}

impl PayloadDetails {
    pub fn new(originator_request_id: String) -> Result<Self, String> {
        if originator_request_id.is_empty()
            || originator_request_id.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("originator request id is empty"));
        }
        // originator_request_id has a max length of 30 characters
        else if originator_request_id.trim().len() > 0 && originator_request_id.trim().len() <= 30
        {
            // originator_request_id is valid
        } else {
            return Err(String::from("originator request id has invalid length"));
        }

        Ok(Self {
            originator_request_id,
        })
    }

    pub fn get_originator_request_id(&self) -> String {
        let originator_request_id = &self.originator_request_id;
        originator_request_id.to_string()
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct TransactionStatusInputDetails {
    _payload: PayloadDetails,
}

impl TransactionStatusInputDetails {
    pub fn new(_payload: PayloadDetails) -> Result<Self, String> {
        Ok(Self { _payload })
    }

    pub fn get_payload(&self) -> &PayloadDetails {
        let _payload = &self._payload;
        _payload
    }
}

// request data

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct PayloadData {
    pub originatorRequestId: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct TransactionStatusData {
    pub payload: PayloadData,
}

// response data

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TransactionStatusResponseData {
    pub status: Option<String>,
    pub statusCode: Option<String>,
}
