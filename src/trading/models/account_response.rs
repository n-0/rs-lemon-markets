/*
 * lemon.markets | Trading API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: b996d5fa9acbe1796d08c2f8360172c63ac713ad
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AccountResponse {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "firstname", skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(rename = "lastname", skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "billing_address", skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<String>,
    #[serde(rename = "billing_email", skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<String>,
    #[serde(rename = "billing_name", skip_serializing_if = "Option::is_none")]
    pub billing_name: Option<String>,
    #[serde(rename = "billing_vat", skip_serializing_if = "Option::is_none")]
    pub billing_vat: Option<String>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "deposit_id", skip_serializing_if = "Option::is_none")]
    pub deposit_id: Option<String>,
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "account_number", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(rename = "iban_brokerage", skip_serializing_if = "Option::is_none")]
    pub iban_brokerage: Option<String>,
    #[serde(rename = "iban_origin", skip_serializing_if = "Option::is_none")]
    pub iban_origin: Option<String>,
    #[serde(rename = "bank_name_origin", skip_serializing_if = "Option::is_none")]
    pub bank_name_origin: Option<String>,
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<i32>,
    #[serde(rename = "cash_to_invest", skip_serializing_if = "Option::is_none")]
    pub cash_to_invest: Option<i32>,
    #[serde(rename = "cash_to_withdraw", skip_serializing_if = "Option::is_none")]
    pub cash_to_withdraw: Option<i32>,
    #[serde(rename = "amount_bought_intraday", skip_serializing_if = "Option::is_none")]
    pub amount_bought_intraday: Option<i32>,
    #[serde(rename = "amount_sold_intraday", skip_serializing_if = "Option::is_none")]
    pub amount_sold_intraday: Option<i32>,
    #[serde(rename = "amount_open_orders", skip_serializing_if = "Option::is_none")]
    pub amount_open_orders: Option<i32>,
    #[serde(rename = "amount_open_withdrawals", skip_serializing_if = "Option::is_none")]
    pub amount_open_withdrawals: Option<i32>,
    #[serde(rename = "amount_estimate_taxes", skip_serializing_if = "Option::is_none")]
    pub amount_estimate_taxes: Option<i32>,
    #[serde(rename = "approved_at", skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<String>,
    #[serde(rename = "trading_plan")]
    pub trading_plan: String,
    #[serde(rename = "data_plan")]
    pub data_plan: String,
    #[serde(rename = "plan", skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(rename = "tax_allowance", skip_serializing_if = "Option::is_none")]
    pub tax_allowance: Option<i32>,
    #[serde(rename = "tax_allowance_start", skip_serializing_if = "Option::is_none")]
    pub tax_allowance_start: Option<String>,
    #[serde(rename = "tax_allowance_end", skip_serializing_if = "Option::is_none")]
    pub tax_allowance_end: Option<String>,
}

impl AccountResponse {
    pub fn new(created_at: String, account_id: String, trading_plan: String, data_plan: String) -> AccountResponse {
        AccountResponse {
            created_at,
            account_id,
            firstname: None,
            lastname: None,
            email: None,
            phone: None,
            address: None,
            billing_address: None,
            billing_email: None,
            billing_name: None,
            billing_vat: None,
            mode: None,
            deposit_id: None,
            client_id: None,
            account_number: None,
            iban_brokerage: None,
            iban_origin: None,
            bank_name_origin: None,
            balance: None,
            cash_to_invest: None,
            cash_to_withdraw: None,
            amount_bought_intraday: None,
            amount_sold_intraday: None,
            amount_open_orders: None,
            amount_open_withdrawals: None,
            amount_estimate_taxes: None,
            approved_at: None,
            trading_plan,
            data_plan,
            plan: None,
            tax_allowance: None,
            tax_allowance_start: None,
            tax_allowance_end: None,
        }
    }
}


