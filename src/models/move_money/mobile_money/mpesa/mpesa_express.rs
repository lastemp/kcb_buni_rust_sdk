use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct MpesaExpressInputDetails {
    phone_number: String,
    _amount: u32,
    invoice_number: String,
    shared_short_code: bool,
    org_short_code: String,
    org_passkey: String,
    callback_url: String,
    transaction_description: String,
}

impl MpesaExpressInputDetails {
    pub fn new(
        phone_number: String,
        _amount: u32,
        invoice_number: String,
        shared_short_code: bool,
        org_short_code: String,
        org_passkey: String,
        callback_url: String,
        transaction_description: String,
    ) -> Result<Self, String> {
        if phone_number.is_empty() || phone_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("phone number is empty"));
        }
        // phone_number has a length of 12 characters
        else if phone_number.trim().len() == 12 {
            // phone_number is valid
        } else {
            return Err(String::from("phone number has invalid length"));
        }

        if _amount > 0 { // _amount is valid
        } else {
            return Err(String::from("amount has invalid value"));
        }

        if invoice_number.is_empty() || invoice_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("invoice number is empty"));
        }
        // invoice_number has a max length of 12 characters
        else if invoice_number.trim().len() > 0 && invoice_number.trim().len() <= 12 {
            // invoice_number is valid
        } else {
            return Err(String::from("invoice number has invalid length"));
        }

        if org_short_code.is_empty() || org_short_code.replace(" ", "").trim().len() == 0 {
            return Err(String::from("org short code is empty"));
        }
        // org_short_code has a max length of 12 characters
        else if org_short_code.trim().len() > 0 && org_short_code.trim().len() <= 12 {
            // org_short_code is valid
        } else {
            return Err(String::from("org short code has invalid length"));
        }

        if org_passkey.is_empty() || org_passkey.replace(" ", "").trim().len() == 0 {
            return Err(String::from("org passkey is empty"));
        }
        // org_passkey has a min length of 1 character
        else if org_passkey.trim().len() > 0 {
            // org_passkey is valid
        } else {
            return Err(String::from("org passkey has invalid length"));
        }

        if callback_url.is_empty() || callback_url.replace(" ", "").trim().len() == 0 {
            return Err(String::from("callback url is empty"));
        }
        // callback_url has a min length of 1 character
        else if callback_url.trim().len() > 0 {
            // callback_url is valid
        } else {
            return Err(String::from("callback url has invalid length"));
        }

        if transaction_description.is_empty()
            || transaction_description.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("transaction description is empty"));
        }
        // transaction_description has a max length of 13 characters
        else if transaction_description.trim().len() > 0
            && transaction_description.trim().len() <= 13
        {
            // transaction_description is valid
        } else {
            return Err(String::from("transaction description has invalid length"));
        }

        Ok(Self {
            phone_number,
            _amount,
            invoice_number,
            shared_short_code,
            org_short_code,
            org_passkey,
            callback_url,
            transaction_description,
        })
    }

    pub fn get_phone_number(&self) -> String {
        let phone_number = &self.phone_number;
        phone_number.to_string()
    }

    pub fn get_amount(&self) -> u32 {
        let _amount = &self._amount;
        *_amount
    }

    pub fn get_invoice_number(&self) -> String {
        let invoice_number = &self.invoice_number;
        invoice_number.to_string()
    }

    pub fn get_shared_short_code(&self) -> bool {
        let shared_short_code = &self.shared_short_code;
        *shared_short_code
    }

    pub fn get_org_short_code(&self) -> String {
        let org_short_code = &self.org_short_code;
        org_short_code.to_string()
    }

    pub fn get_org_passkey(&self) -> String {
        let org_passkey = &self.org_passkey;
        org_passkey.to_string()
    }

    pub fn get_callback_url(&self) -> String {
        let callback_url = &self.callback_url;
        callback_url.to_string()
    }

    pub fn get_transaction_description(&self) -> String {
        let transaction_description = &self.transaction_description;
        transaction_description.to_string()
    }
}

// request data

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct MpesaExpressData {
    pub phoneNumber: String,
    pub amount: String,
    pub invoiceNumber: String,
    pub sharedShortCode: bool,
    pub orgShortCode: String,
    pub orgPassKey: String,
    pub callbackUrl: String,
    pub transactionDescription: String,
}

// response data

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct HeaderResponseData {
    pub statusCode: Option<String>,
    pub statusDescription: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct MpesaExpressResponseData {
    pub header: HeaderResponseData,
    pub response: Option<String>,
}
