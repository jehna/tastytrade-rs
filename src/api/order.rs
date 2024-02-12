use derive_builder::Builder;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::accounts::AccountNumber;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PriceEffect {
    Debit,
    Credit,
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Action {
    #[serde(rename = "Buy to Open")]
    BuyToOpen,
    #[serde(rename = "Sell to Open")]
    SellToOpen,
    #[serde(rename = "Buy to Close")]
    BuyToClose,
    #[serde(rename = "Sell to Close")]
    SellToClose,
    Sell,
    Buy,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InstrumentType {
    Equity,
    #[serde(rename = "Equity Option")]
    EquityOption,
    #[serde(rename = "Equity Offering")]
    EquityOffering,
    Future,
    #[serde(rename = "Future Option")]
    FutureOption,
    Cryptocurrency,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderType {
    Limit,
    Market,
    #[serde(rename = "Marketable Limit")]
    MarketableLimit,
    Stop,
    #[serde(rename = "Stop Limit")]
    StopLimit,
    #[serde(rename = "Notional Market")]
    NotionalMarket,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TimeInForce {
    Day,
    GTC,
    GTD,
    Ext,
    #[serde(rename = "GTC Ext")]
    GTCExt,
    IOC,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderStatus {
    Received,
    Routed,
    #[serde(rename = "In Flight")]
    InFlight,
    Live,
    #[serde(rename = "Cancel Requested")]
    CancelRequested,
    #[serde(rename = "Replace Requested")]
    ReplaceRequested,
    Contingent,
    Filled,
    Cancelled,
    Expired,
    Rejected,
    Removed,
    #[serde(rename = "Partially Removed")]
    PartiallyRemoved,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct Symbol(pub String);

impl<T: AsRef<str>> From<T> for Symbol {
    fn from(value: T) -> Self {
        Self(value.as_ref().to_owned())
    }
}

pub trait AsSymbol {
    fn as_symbol(&self) -> Symbol;
}

impl<T: AsRef<str>> AsSymbol for T {
    fn as_symbol(&self) -> Symbol {
        Symbol(self.as_ref().to_owned())
    }
}

impl AsSymbol for Symbol {
    fn as_symbol(&self) -> Symbol {
        self.clone()
    }
}

impl AsSymbol for &Symbol {
    fn as_symbol(&self) -> Symbol {
        (*self).clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct OrderId(pub String);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LiveOrderRecord {
    pub id: Decimal,
    pub account_number: AccountNumber,
    pub time_in_force: TimeInForce,
    pub order_type: OrderType,
    pub size: u64,
    pub underlying_symbol: Symbol,
    pub underlying_instrument_type: InstrumentType,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub price: Decimal,
    pub price_effect: PriceEffect,
    pub status: OrderStatus,
    pub cancellable: bool,
    pub editable: bool,
    pub edited: bool,
    pub received_at: String,
    pub updated_at: u64,
    pub global_request_id: String,
    pub legs: Vec<LiveOrderLeg>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LiveOrderLeg {
    pub instrument_type: InstrumentType,
    pub symbol: Symbol,
    pub quantity: Decimal,
    pub remaining_quantity: Decimal,
    pub action: Action,
    pub fills: Vec<String>,
}

#[derive(Builder, Serialize)]
#[serde(rename_all = "kebab-case")]
#[builder(setter(into))]
pub struct Order {
    time_in_force: TimeInForce,
    order_type: OrderType,

    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    price: Decimal,
    price_effect: PriceEffect,
    legs: Vec<OrderLeg>,
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
#[builder(setter(into))]
pub struct OrderLeg {
    pub instrument_type: InstrumentType,
    pub symbol: Symbol,
    #[serde(with = "rust_decimal::serde::float")]
    pub quantity: Decimal,
    pub action: Action,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OrderPlacedResult {
    pub order: LiveOrderRecord,
    pub warnings: Vec<Warning>,
    pub buying_power_effect: BuyingPowerEffect,
    pub fee_calculation: FeeCalculation,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DryRunResult {
    pub order: DryRunRecord,
    pub warnings: Vec<Warning>,
    pub buying_power_effect: BuyingPowerEffect,
    pub fee_calculation: FeeCalculation,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DryRunRecord {
    pub account_number: AccountNumber,
    pub time_in_force: TimeInForce,
    pub order_type: OrderType,
    pub size: u64,
    pub underlying_symbol: Symbol,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub price: Decimal,
    pub price_effect: PriceEffect,
    pub status: OrderStatus,
    pub cancellable: bool,
    pub editable: bool,
    pub edited: bool,
    pub legs: Vec<OrderLeg>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]

pub struct FullOrder {
    pub id: OrderId,
    pub account_number: AccountNumber,
    pub time_in_force: TimeInForce,
    pub order_type: OrderType,
    pub size: Decimal,
    pub underlying_symbol: Symbol,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub price: Decimal,
    pub price_effect: PriceEffect,
    pub status: OrderStatus,
    pub cancellable: bool,
    pub editable: bool,
    pub edited: bool,
    pub legs: Vec<OrderLeg>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BuyingPowerEffect {
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub change_in_margin_requirement: Decimal,
    pub change_in_margin_requirement_effect: PriceEffect,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub change_in_buying_power: Decimal,
    pub change_in_buying_power_effect: PriceEffect,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub current_buying_power: Decimal,
    pub current_buying_power_effect: PriceEffect,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub new_buying_power: Decimal,
    pub new_buying_power_effect: PriceEffect,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub isolated_order_margin_requirement: Decimal,
    pub isolated_order_margin_requirement_effect: PriceEffect,
    pub is_spread: bool,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub impact: Decimal,
    pub effect: PriceEffect,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FeeCalculation {
    pub regulatory_fees: Decimal,
    pub regulatory_fees_effect: PriceEffect,
    pub clearing_fees: Decimal,
    pub clearing_fees_effect: PriceEffect,
    pub commission: Decimal,
    pub commission_effect: PriceEffect,
    pub proprietary_index_option_fees: Decimal,
    pub proprietary_index_option_fees_effect: PriceEffect,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub total_fees: Decimal,
    pub total_fees_effect: PriceEffect,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Warning {
    pub code: String,
    pub message: String,
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_derp() {
        let json = json!({
            "order":{"id":129359,"account-number":"5WU44237","time-in-force":"Day","order-type":"Limit","size":100,"underlying-symbol":"AAPL","underlying-instrument-type":"Equity","price":"181.01","price-effect":"Debit","status":"Received","cancellable":true,"editable":true,"edited":false,"received-at":"2024-02-11T21:59:57.143+00:00","updated-at":1234,"global-request-id":"153cc8811e19d5aba6c9bfa083251e56","legs":[{"instrument-type":"Equity","symbol":"AAPL","quantity":100,"remaining-quantity":100,"action":"Buy to Open","fills":[]}]},"warnings":[{"code":"tif_next_valid_sesssion","message":"Your order will begin working during next valid session."}],"buying-power-effect":{"change-in-margin-requirement":"9050.5","change-in-margin-requirement-effect":"Debit","change-in-buying-power":"9050.58","change-in-buying-power-effect":"Debit","current-buying-power":"10056.31","current-buying-power-effect":"Credit","new-buying-power":"1005.73","new-buying-power-effect":"Credit","isolated-order-margin-requirement":"9050.5","isolated-order-margin-requirement-effect":"Debit","is-spread":false,"impact":"9050.58","effect":"Debit"},"fee-calculation":{"regulatory-fees":"0.0","regulatory-fees-effect":"None","clearing-fees":"0.08","clearing-fees-effect":"Debit","commission":"0.0","commission-effect":"None","proprietary-index-option-fees":"0.0","proprietary-index-option-fees-effect":"None","total-fees":"0.08","total-fees-effect":"Debit"}
        }).to_string();
        let res: OrderPlacedResult = serde_json::from_str(&json).unwrap();
    }
}
