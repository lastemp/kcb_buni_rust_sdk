use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct FundsTransferInputDetails {
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
}

impl FundsTransferInputDetails {
    pub fn new(
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
    ) -> Result<Self, String> {
        if company_code.is_empty() || company_code.replace(" ", "").trim().len() == 0 {
            return Err(String::from("company code is empty"));
        }
        // company_code has a max length of 32 characters
        else if company_code.trim().len() > 0 && company_code.trim().len() <= 32 {
            // company_code is valid
        } else {
            return Err(String::from("company code has invalid length"));
        }

        if transaction_type.is_empty() || transaction_type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("transaction type is empty"));
        }
        // transaction_type has a max length of 32 characters
        else if transaction_type.trim().len() > 0 && transaction_type.trim().len() <= 32 {
            // transaction_type is valid
        } else {
            return Err(String::from("transaction type has invalid length"));
        }

        if debit_account_number.is_empty()
            || debit_account_number.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("debit account number is empty"));
        }
        // debit_account_number has a max length of 32 characters
        else if debit_account_number.trim().len() > 0 && debit_account_number.trim().len() <= 32 {
            // debit_account_number is valid
        } else {
            return Err(String::from("debit account number has invalid length"));
        }

        if credit_account_number.is_empty()
            || credit_account_number.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("credit account number is empty"));
        }
        // credit_account_number has a max length of 32 characters
        else if credit_account_number.trim().len() > 0 && credit_account_number.trim().len() <= 32
        {
            // credit_account_number is valid
        } else {
            return Err(String::from("credit account number has invalid length"));
        }

        if debit_amount > 0 { // debit_amount is valid
        } else {
            return Err(String::from("debit amount has invalid value"));
        }

        if payment_details.is_empty() || payment_details.replace(" ", "").trim().len() == 0 {
            return Err(String::from("payment details is empty"));
        }
        // payment_details has a max length of 16 characters
        else if payment_details.trim().len() > 0 && payment_details.trim().len() <= 16 {
            // payment_details is valid
        } else {
            return Err(String::from("payment details has invalid length"));
        }

        if transaction_reference.is_empty()
            || transaction_reference.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("transaction reference is empty"));
        }
        // transaction_reference has a max length of 15 characters
        else if transaction_reference.trim().len() > 0 && transaction_reference.trim().len() <= 15
        {
            // transaction_reference is valid
        } else {
            return Err(String::from("transaction reference has invalid length"));
        }

        if _currency.is_empty() || _currency.replace(" ", "").trim().len() == 0 {
            return Err(String::from("currency is empty"));
        }
        // currency has a length of 3 characters
        else if _currency.trim().len() == 3 {
            // currency is valid
        } else {
            return Err(String::from("currency has invalid length"));
        }

        if beneficiary_details.is_empty() || beneficiary_details.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("beneficiary details is empty"));
        }
        // beneficiary_details has a max length of 32 characters
        else if beneficiary_details.trim().len() > 0 && beneficiary_details.trim().len() <= 32 {
            // beneficiary_details is valid
        } else {
            return Err(String::from("beneficiary details has invalid length"));
        }

        if beneficiary_bank_code.is_empty()
            || beneficiary_bank_code.replace(" ", "").trim().len() == 0
        {
            return Err(String::from("beneficiary bank code is empty"));
        }
        // beneficiary_bank_code has a max length of 32 characters
        else if beneficiary_bank_code.trim().len() > 0 && beneficiary_bank_code.trim().len() <= 32
        {
            // beneficiary_bank_code is valid
        } else {
            return Err(String::from("beneficiary bank code has invalid length"));
        }

        Ok(Self {
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
        })
    }

    pub fn get_company_code(&self) -> String {
        let company_code = &self.company_code;
        company_code.to_string()
    }

    pub fn get_transaction_type(&self) -> String {
        let transaction_type = &self.transaction_type;
        transaction_type.to_string()
    }

    pub fn get_debit_account_number(&self) -> String {
        let debit_account_number = &self.debit_account_number;
        debit_account_number.to_string()
    }

    pub fn get_credit_account_number(&self) -> String {
        let credit_account_number = &self.credit_account_number;
        credit_account_number.to_string()
    }

    pub fn get_debit_amount(&self) -> u32 {
        let debit_amount = &self.debit_amount;
        *debit_amount
    }

    pub fn get_payment_details(&self) -> String {
        let payment_details = &self.payment_details;
        payment_details.to_string()
    }

    pub fn get_transaction_reference(&self) -> String {
        let transaction_reference = &self.transaction_reference;
        transaction_reference.to_string()
    }

    pub fn get_currency(&self) -> String {
        let _currency = &self._currency;
        _currency.to_string()
    }

    pub fn get_beneficiary_details(&self) -> String {
        let beneficiary_details = &self.beneficiary_details;
        beneficiary_details.to_string()
    }

    pub fn get_beneficiary_bank_code(&self) -> String {
        let beneficiary_bank_code = &self.beneficiary_bank_code;
        beneficiary_bank_code.to_string()
    }
}

// request data

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct FundsTransferData {
    pub companyCode: String,
    pub transactionType: String,
    pub debitAccountNumber: String,
    pub creditAccountNumber: String,
    pub debitAmount: u32,
    pub paymentDetails: String,
    pub transactionReference: String,
    pub currency: String,
    pub beneficiaryDetails: String,
    pub beneficiaryBankCode: String,
}

// response data

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct HeaderResponseData {
    pub messageID: Option<String>,
    pub statusCode: Option<String>,
    pub statusMessage: Option<String>,
    pub statusDescription: Option<String>,
    pub retrievalRefNumber: Option<String>,
    pub merchantID: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AccountFundsTransferResponseData {
    pub header: HeaderResponseData,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UnauthorizedErrorData {
    pub code: Option<String>,
    pub message: Option<String>,
    pub description: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct UnauthorizedErrorResponseData {
    pub fault: UnauthorizedErrorData,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct BadRequestErrorResponseData {
    pub MessageReference: Option<String>,
    pub MessageDateTime: Option<String>,
    pub MessageCode: Option<String>,
    pub MessageDescription: Option<String>,
}
