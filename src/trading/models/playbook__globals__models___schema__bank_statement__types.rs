/*
 * lemon.markets | Trading API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: b996d5fa9acbe1796d08c2f8360172c63ac713ad
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PlaybookGlobalsModelsSchemaBankStatementTypes : An enumeration.

/// An enumeration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlaybookGlobalsModelsSchemaBankStatementTypes {
    #[serde(rename = "pay_in")]
    PayIn,
    #[serde(rename = "pay_out")]
    PayOut,
    #[serde(rename = "order_buy")]
    OrderBuy,
    #[serde(rename = "order_sell")]
    OrderSell,
    #[serde(rename = "dividend")]
    Dividend,
    #[serde(rename = "tax_refund")]
    TaxRefund,
    #[serde(rename = "interest_paid")]
    InterestPaid,
    #[serde(rename = "interest_earned")]
    InterestEarned,
    #[serde(rename = "eod_balance")]
    EodBalance,
    #[serde(rename = "goodwill")]
    Goodwill,

}

impl ToString for PlaybookGlobalsModelsSchemaBankStatementTypes {
    fn to_string(&self) -> String {
        match self {
            Self::PayIn => String::from("pay_in"),
            Self::PayOut => String::from("pay_out"),
            Self::OrderBuy => String::from("order_buy"),
            Self::OrderSell => String::from("order_sell"),
            Self::Dividend => String::from("dividend"),
            Self::TaxRefund => String::from("tax_refund"),
            Self::InterestPaid => String::from("interest_paid"),
            Self::InterestEarned => String::from("interest_earned"),
            Self::EodBalance => String::from("eod_balance"),
            Self::Goodwill => String::from("goodwill"),
        }
    }
}

impl Default for PlaybookGlobalsModelsSchemaBankStatementTypes {
    fn default() -> PlaybookGlobalsModelsSchemaBankStatementTypes {
        Self::PayIn
    }
}




