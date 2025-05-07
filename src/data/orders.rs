//! Paypal object definitions used by the orders api.

use super::common::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The intent to either capture payment immediately or authorize a payment for an order after order creation.
#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Intent {
    /// The merchant intends to capture payment immediately after the customer makes a payment.
    #[default]
    Capture,
    /// The merchant intends to authorize a payment and place funds on hold after the customer makes a payment.
    /// Authorized payments are guaranteed for up to three days but are available to capture for up to 29 days.
    /// After the three-day honor period, the original authorized payment expires and you must re-authorize the payment.
    /// You must make a separate request to capture payments on demand.
    /// This intent is not supported when you have more than one `purchase_unit` within your order.
    Authorize,
}

/// Represents a payer name.
///
/// <https://developer.paypal.com/docs/api/orders/v2/#definition-payer.name>
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Builder)]
pub struct PayerName {
    /// When the party is a person, the party's given, or first, name.
    pub given_name: String,
    /// When the party is a person, the party's surname or family name. Also known as the last name.
    /// Required when the party is a person. Use also to store multiple surnames including the matronymic, or mother's, surname.
    pub surname: String,
}

/// The phone number, in its canonical international E.164 numbering plan format.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct PhoneNumber {
    /// The country calling code (CC), in its canonical international E.164 numbering plan format.
    /// The combined length of the CC and the national number must not be greater than 15 digits.
    /// The national number consists of a national destination code (NDC) and subscriber number (SN).
    pub country_code: Option<String>,
    /// The national number, in its canonical international E.164 numbering plan format.
    /// The combined length of the country calling code (CC) and the national number must not be greater than 15 digits.
    /// The national number consists of a national destination code (NDC) and subscriber number (SN).
    pub national_number: String,
}

/// The phone number of the customer. Available only when you enable the
/// Contact Telephone Number option in the Profile & Settings for the merchant's PayPal account.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Phone {
    /// The phone type.
    pub phone_type: Option<PhoneType>,
    /// The phone number
    pub phone_number: PhoneNumber,
}

/// The customer's tax ID type. Supported for the PayPal payment method only.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)]
pub enum TaxIdType {
    /// The individual tax ID type.
    BR_CPF,
    /// The business tax ID type.
    BR_CNPJ,
}

/// The tax information of the payer.
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct TaxInfo {
    /// The customer's tax ID. Supported for the PayPal payment method only.
    /// Typically, the tax ID is 11 characters long for individuals and 14 characters long for businesses.
    pub tax_id: String,
    /// The customer's tax ID type. Supported for the PayPal payment method only.
    pub tax_id_type: TaxIdType,
}

/// The customer who approves and pays for the order. The customer is also known as the payer.
///
/// <https://developer.paypal.com/docs/api/orders/v2/#definition-payer>
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option), default)]
pub struct Payer {
    /// The name of the payer.
    pub name: Option<PayerName>,
    /// The email address of the payer.
    pub email_address: Option<String>,
    /// The PayPal-assigned ID for the payer.
    pub payer_id: Option<String>,
    /// The phone number of the customer. Available only when you enable the Contact
    /// Telephone Number option in the Profile & Settings for the merchant's PayPal account.
    pub phone: Option<Phone>,
    /// The birth date of the payer in YYYY-MM-DD format.
    pub birth_date: Option<String>,
    /// The tax information of the payer. Required only for Brazilian payer's.
    pub tax_info: Option<TaxInfo>,
    /// The address of the payer.
    pub address: Option<Address>,
}

/// Breakdown provides details such as total item amount, total tax amount, shipping, handling, insurance, and discounts, if any.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct Breakdown {
    /// The subtotal for all items. Required if the request includes purchase_units[].items[].unit_amount.
    /// Must equal the sum of (items[].unit_amount * items[].quantity) for all items.
    pub item_total: Option<Money>,
    /// The shipping fee for all items within a given purchase_unit.
    pub shipping: Option<Money>,
    /// The handling fee for all items within a given purchase_unit.
    pub handling: Option<Money>,
    /// The total tax for all items. Required if the request includes purchase_units.items.tax. Must equal the sum of (items[].tax * items[].quantity) for all items.
    pub tax_total: Option<Money>,
    /// The insurance fee for all items within a given purchase_unit.
    pub insurance: Option<Money>,
    /// The shipping discount for all items within a given purchase_unit.
    pub shipping_discount: Option<Money>,
    /// The discount for all items within a given purchase_unit.
    pub discount: Option<Money>,
}

/// Represents an amount of money.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Amount {
    /// The [three-character ISO-4217 currency code](https://developer.paypal.com/docs/integration/direct/rest/currency-codes/) that identifies the currency.
    pub currency_code: Currency,
    /// The value, which might be:
    /// - An integer for currencies like JPY that are not typically fractional.
    /// - A decimal fraction for currencies like TND that are subdivided into thousandths.
    ///
    /// For the required number of decimal places for a currency code, see [Currency Codes](https://developer.paypal.com/docs/api/reference/currency-codes/).
    pub value: String,
    /// The breakdown of the amount.
    pub breakdown: Option<Breakdown>,
}

impl Amount {
    /// Creates a new amount with the required values.
    pub fn new(currency: Currency, value: &str) -> Self {
        Amount {
            currency_code: currency,
            value: value.to_owned(),
            breakdown: None,
        }
    }

    /// Creates a new amount with the EUR currency.
    pub fn eur(value: &str) -> Self {
        Amount {
            currency_code: Currency::EUR,
            value: value.to_owned(),
            breakdown: None,
        }
    }

    /// Creates a new amount with the USD currency.
    pub fn usd(value: &str) -> Self {
        Amount {
            currency_code: Currency::USD,
            value: value.to_owned(),
            breakdown: None,
        }
    }
}

/// The merchant who receives payment for this transaction.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct Payee {
    /// The email address of merchant.
    pub email_address: Option<String>,
    /// The encrypted PayPal account ID of the merchant.
    pub merchant_id: Option<String>,
}

/// Fees, commissions, tips, or donations
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct PlatformFee {
    /// The fee for this transaction.
    pub amount: Money,

    /// The merchant who receives payment for this transaction.
    pub payee: Option<Payee>,
}

/// The funds that are held on behalf of the merchant
#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum DisbursementMode {
    /// The funds are released to the merchant immediately.
    #[default]
    Instant,
    /// The funds are held for a finite number of days. The actual duration depends on the region and type of integration.
    /// You can release the funds through a referenced payout.
    /// Otherwise, the funds disbursed automatically after the specified duration.
    Delayed,
}
/// Any additional payment instructions for PayPal Commerce Platform customers.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct PaymentInstruction {
    /// An array of various fees, commissions, tips, or donations.
    pub platform_fees: Option<Vec<PlatformFee>>,
    /// This field is only enabled for selected merchants/partners to use and provides the ability to trigger a specific pricing rate/plan for a payment transaction
    /// The list of eligible 'payee_pricing_tier_id' would be provided to you by your Account Manager. Specifying values other than the one provided to you by your account manager would result in an error.
    pub payee_pricing_tier_id: Option<String>,
    /// FX identifier generated returned by PayPal to be used for payment processing in order to honor FX rate (for eligible integrations) to be used when amount is settled/received into the payee account.
    pub payee_receivable_fx_rate_id: Option<String>,
    /// The funds that are held on behalf of the merchant.
    pub disbursement_mode: Option<DisbursementMode>,
}

/// The item category type.
#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemCategoryType {
    /// Goods that are stored, delivered, and used in their electronic format.
    /// This value is not currently supported for API callers that leverage
    /// the [PayPal for Commerce Platform](https://www.paypal.com/us/webapps/mpp/commerce-platform) product.
    #[default]
    DigitalGoods,
    /// A tangible item that can be shipped with proof of delivery.
    PhysicalGoods,

    /// A contribution or gift for which no good or service is exchanged, usually to a not for profit organization.
    Donation,
}
/// The name of the person to whom to ship the items.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ShippingDetailName {
    /// The name of the person to whom to ship the items. Supports only the full_name property.
    pub full_name: String,
}

/// Method of purchase fulfillment
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShippingType {
    /// The payer intends to receive the items at a specified address.
    Shipping,
    /// DEPRECATED. Please use "PICKUP_FROM_PERSON" instead.
    PickupInPerson,
    /// The payer intends to pick up the item(s) from the payee's physical store.
    /// Also termed as BOPIS, "Buy Online, Pick-up in Store".
    /// Seller protection is provided with this option.
    PickupInStore,
    /// The payer intends to pick up the item(s) from the payee in person.
    /// Also termed as BOPIP, "Buy Online, Pick-up in Person".
    /// Seller protection is not available, since the payer is receiving the item from the payee in person, and can validate the item prior to payment.
    PickupFromPerson,
}

/// The status of the item shipment.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrackerStatus {
    /// The shipment was cancelled and the tracking number no longer applies.
    Cancelled,
    /// The merchant has assigned a tracking number to the items being shipped from the Order.
    /// This does not correspond to the carrier's actual status for the shipment.
    /// The latest status of the parcel must be retrieved from the carrier.
    Shipped,
}
/// item in the shipment.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
pub struct ShipmentItem {
    /// The item name or title.
    pub name: Option<String>,
    /// The item quantity. Must be a whole number.
    pub quantity: Option<String>,
    /// The stock keeping unit (SKU) for the item. This can contain unicode characters.
    pub sku: Option<String>,
    /// The URL to the item being purchased. Visible to buyer and used in buyer experiences.
    pub url: Option<String>,
    /// The URL of the item's image.
    /// File type and size restrictions apply.
    /// An image that violates these restrictions will not be honored.
    pub image_url: Option<String>,
    /// The Universal Product Code of the item.
    pub upc: Option<ItemUpc>,
}

/// trackers for a transaction.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
pub struct TransactionTracker {
    /// The tracker id.
    pub id: Option<String>,
    /// The status of the item shipment.
    pub status: Option<TrackerStatus>,
    /// An array of details of items in the shipment.
    pub items: Option<Vec<ShipmentItem>>,
    /// An array of request-related HATEOAS links.
    pub links: Option<Vec<LinkDescription>>,
    /// The date and time when the transaction occurred, in Internet date and time format.
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the transaction was last updated, in Internet date and time format.
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// Shipping options that the payee or merchant offers to the payer to ship or pick up their items.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
pub struct ShippingOption {
    /// A unique ID that identifies a payer-selected shipping option.
    pub id: String,
    /// A description that the payer sees, which helps them choose an appropriate shipping option.
    /// For example, Free Shipping, USPS Priority Shipping, Expédition prioritaire USPS, or USPS yōuxiān fā huò.
    /// Localize this description to the payer's locale.
    pub label: String,
    /// If the API request sets selected = true, it represents the shipping option that the payee
    /// or merchant expects to be pre-selected for the payer when they
    /// first view the shipping.options in the PayPal Checkout experience.
    /// As part of the response if a shipping.option contains selected=true,
    /// it represents the shipping option that the payer selected during the course of checkout with PayPal.
    /// Only one shipping.option can be set to selected=true.
    pub selected: bool,
    /// A classification for the method of purchase fulfillment.
    pub shipping_type: Option<ShippingType>,
    /// The shipping cost for the selected option.
    pub amount: Option<Money>,
}

/// The name and address of the person to whom to ship the items.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct ShippingDetail {
    /// A classification for the method of purchase fulfillment (e.g shipping, in-store pickup, etc).
    /// Either type or options may be present, but not both.
    #[serde(rename = "type")]
    pub shipping_type: Option<ShippingType>,
    /// An array of shipping options that the payee or merchant offers to the payer to ship or pick up their items.
    pub options: Option<Vec<ShippingOption>>,
    /// The name of the person to whom to ship the items. Supports only the full_name property.
    pub name: Option<ShippingDetailName>,
    /// The phone number of the recipient of the shipped items,
    /// which may belong to either the payer, or an alternate contact, for delivery. [Format - canonical international E.164 numbering plan]
    pub phone_number: Option<PhoneNumber>,
    /// The address of the person to whom to ship the items.
    pub address: Option<Address>,
    /// An array of trackers for a transaction.
    pub trackers: Option<Vec<TransactionTracker>>,
}

/// Represents an item.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct Item {
    /// The item name or title.
    pub name: String,
    /// The item quantity. Must be a whole number.
    pub quantity: String,
    /// The detailed item description.
    pub description: Option<String>,
    /// The stock keeping unit (SKU) for the item.
    pub sku: Option<String>,
    /// The URL to the item being purchased. Visible to buyer and used in buyer experiences.
    pub url: Option<String>,
    /// The item category type
    pub category: Option<ItemCategoryType>,
    /// The URL of the item's image. File type and size restrictions apply. An image that violates these restrictions will not be honored.
    pub image_url: Option<String>,
    /// The item price or rate per unit.
    /// If you specify unit_amount, purchase_units[].amount.breakdown.item_total is required. Must equal unit_amount * quantity for all items.
    pub unit_amount: Money,
    /// The item tax for each unit. If tax is specified, purchase_units[].amount.breakdown.tax_total is required. Must equal tax * quantity for all items.
    pub tax: Option<Money>,
    /// The Universal Product Code of the item.
    pub upc: Option<ItemUpc>,
}

/// The status of the payment authorization.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorizationStatus {
    /// The authorized payment is created. No captured payments have been made for this authorized payment.
    Created,
    /// The authorized payment has one or more captures against it. The sum of these captured payments is greater than the amount of the original authorized payment.
    Captured,
    /// PayPal cannot authorize funds for this authorized payment.
    Denied,
    /// The authorized payment has expired.
    Expired,
    /// A captured payment was made for the authorized payment for an amount that is less than the amount of the original authorized payment.
    PartiallyExpired,
    /// The payment which was authorized for an amount that is less than the originally requested amount.
    PartiallyCaptured,
    /// The authorized payment was voided. No more captured payments can be made against this authorized payment.
    Voided,
    /// The created authorization is in pending state. For more information, see status.details.
    Pending,
}

/// Seller Protection Status
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SellerProtectionStatus {
    /// Your PayPal balance remains intact if the customer claims that
    /// they did not receive an item or the account holder claims that they did not authorize the payment.
    Eligible,
    /// Your PayPal balance remains intact if the customer claims that they did not receive an item.
    PartiallyEligible,
    /// This transaction is not eligible for seller protection.
    NotEligible,
}

/// Seller Protection Data
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct SellerProtection {
    /// Indicates whether the transaction is eligible for seller protection.
    /// For information, see PayPal Seller Protection for Merchants.
    pub status: Option<SellerProtectionStatus>,
    /// An array of conditions that are covered for the transaction.
    pub dispute_categories: Option<Vec<String>>,
}

/// Reference values used by the card network to identify a transaction.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct NetworkTransactionReference {
    /// Transaction reference id returned by the scheme.
    /// For Visa and Amex, this is the "Tran id" field in response.
    /// For MasterCard, this is the "BankNet reference id" field in response.
    /// For Discover, this is the "NRID" field in response.
    /// The pattern we expect for this field from Visa/Amex/CB/Discover is numeric, Mastercard/BNPP is alphanumeric and Paysecure is alphanumeric with special character -.
    pub id: String,
    /// The date that the transaction was authorized by the scheme.
    /// This field may not be returned for all networks. MasterCard refers to this field as "BankNet reference date.
    pub date: Option<String>,
    /// Reference ID issued for the card transaction.
    /// This ID can be used to track the transaction across processors, card brands and issuing banks.
    pub acquirer_reference_number: Option<String>,
    /// Name of the card network through which the transaction was routed.
    pub network: Option<String>,
}

/// A payment authorization.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct AuthorizationWithData {
    /// The status for the authorized payment.
    pub status: AuthorizationStatus,
    /// The details of the authorized order pending status.
    pub status_details: AuthorizationStatusDetails,
    /// The PayPal-generated ID for the authorized payment.
    pub id: Option<String>,
    /// The API caller-provided external invoice number for this order.
    /// Appears in both the payer's transaction history and the emails that the payer receives.
    pub invoice_id: Option<String>,
    /// The API caller-provided external ID.
    /// Used to reconcile API caller-initiated transactions with PayPal transactions.
    /// Appears in transaction and settlement reports.
    pub custom_id: Option<String>,
    /// An array of related HATEOAS links.
    pub links: Option<Vec<LinkDescription>>,
    /// The amount for this authorized payment.
    pub amount: Option<Money>,
    /// The level of protection offered as defined by PayPal Seller Protection for Merchants.
    pub seller_protection: Option<SellerProtection>,
    /// The date and time when the authorized payment expires, in Internet date and time format.
    pub expiration_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the transaction occurred, in Internet date and time format.
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the transaction was last updated, in Internet date and time format.
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    ///The processor response information for payment requests, such as direct credit card transactions.
    pub processor_response: Option<serde_json::Value>,
}

/// The capture status.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CaptureStatus {
    /// The funds for this captured payment were credited to the payee's PayPal account.
    Completed,
    ///  The funds could not be captured.
    Declined,
    /// An amount less than this captured payment's amount was partially refunded to the payer.
    PartiallyRefunded,
    /// The funds for this captured payment was not yet credited to the payee's PayPal account. For more information, see status.details.
    Pending,
    /// An amount greater than or equal to this captured payment's amount was refunded to the payer.
    Refunded,
}

/// Capture status reason.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CaptureStatusDetailsReason {
    /// The payer initiated a dispute for this captured payment with PayPal.
    BuyerComplaint,
    /// The captured funds were reversed in response to the payer disputing this captured payment with
    /// the issuer of the financial instrument used to pay for this captured payment.
    Chargeback,
    /// The payer paid by an eCheck that has not yet cleared.
    Echeck,
    /// Visit your online account. In your **Account Overview**, accept and deny this payment.
    InternationalWithdrawal,
    /// No additional specific reason can be provided. For more information about this captured payment, visit your account online or contact PayPal.
    Other,
    /// The captured payment is pending manual review.
    PendingReview,
    /// The payee has not yet set up appropriate receiving preferences for their account.
    /// For more information about how to accept or deny this payment, visit your account online.
    /// This reason is typically offered in scenarios such as when the currency of the captured
    /// payment is different from the primary holding currency of the payee.
    ReceivingPreferenceMandatesManualAction,
    /// The captured funds were refunded.
    Refunded,
    /// The payer must send the funds for this captured payment. This code generally appears for manual EFTs.
    TransactionApprovedAwaitingFunding,
    /// The payee does not have a PayPal account.
    Unilateral,
    /// The payee's PayPal account is not verified.
    VerificationRequired,
}

/// Details about the captured payment status.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub struct CaptureStatusDetails {
    /// The reason why the captured payment status is PENDING or DENIED.
    pub reason: CaptureStatusDetailsReason,
}

/// Exchange Rate Detail
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExchangeRateDetail {
    /// The target currency amount. Equivalent to one unit of the source currency.
    /// Formatted as integer or decimal value with one to 15 digits to the right of the decimal point.
    pub value: String,
    /// The source currency from which to convert an amount.
    pub source_currency: Option<String>,
    /// The target currency to which to convert an amount.
    pub target_currency: Option<String>,
}

/// Seller Receivable Breakdown
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SellerReceivableBreakdown {
    /// An array of platform or partner fees, commissions, or brokerage fees that associated with the captured payment.
    pub platform_fees: Option<Vec<PlatformFee>>,
    /// The amount for this captured payment in the currency of the transaction.
    pub gross_amount: Money,
    /// The applicable fee for this captured payment in the currency of the transaction.
    pub paypal_fee: Money,
    /// The applicable fee for this captured payment in the receivable currency.
    /// Returned only in cases the fee is charged in the receivable currency. Example 'CNY'.
    pub paypal_fee_in_receivable_currency: Option<Money>,
    /// The net amount that the payee receives for this captured payment in their PayPal account.
    /// The net amount is computed as gross_amount minus the paypal_fee minus the platform_fees.
    pub net_amount: Option<Money>,
    /// The net amount that is credited to the payee's PayPal account.
    /// Returned only when the currency of the captured payment is different from the currency of the PayPal account where the payee wants to credit the funds.
    /// The amount is computed as net_amount times exchange_rate.
    pub receivable_amount: Option<Money>,
    /// The exchange rate that determines the amount that is credited to the payee's PayPal account.
    /// Returned when the currency of the captured payment is different from the currency of the PayPal account where the payee wants to credit the funds.
    pub exchange_rate: Option<ExchangeRateDetail>,
}

/// A captured payment.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Capture {
    /// The date and time when the transaction occurred, in Internet date and time format.
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the transaction was last updated, in Internet date and time format.
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The PayPal-generated ID for the captured payment.
    pub id: Option<String>,
    /// The API caller-provided external invoice number for this order.
    /// Appears in both the payer's transaction history and the emails that the payer receives.
    pub invoice_id: Option<String>,
    /// The API caller-provided external ID.
    /// Used to reconcile API caller-initiated transactions with PayPal transactions.
    /// Appears in transaction and settlement reports.
    pub custom_id: Option<String>,
    /// Indicates whether you can make additional captures against the authorized payment.
    /// Set to true if you do not intend to capture additional payments against the authorization.
    /// Set to false if you intend to capture additional payments against the authorization.
    pub final_capture: Option<bool>,
    /// An array of related HATEOAS links.
    pub links: Option<Vec<LinkDescription>>,
    /// The amount for this captured payment.
    pub amount: Money,
    /// Reference values used by the card network to identify a transaction.
    pub network_transaction_reference: Option<NetworkTransactionReference>,
    /// The level of protection offered as defined by PayPal Seller Protection for Merchants.
    pub seller_protection: Option<SellerProtection>,
    /// The status of the captured payment.
    pub status: CaptureStatus,
    /// The details of the captured payment status.
    pub status_details: Option<CaptureStatusDetails>,
    /// The detailed breakdown of the capture activity. This is not available for transactions that are in pending state.
    pub seller_receivable_breakdown: Option<SellerReceivableBreakdown>,
    /// The funds that are held on behalf of the merchant.
    pub disbursement_mode: Option<DisbursementMode>,
    ///An object that provides additional processor information for a direct credit card transaction.
    pub processor_response: Option<serde_json::Value>,
}

/// The status of the refund
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundStatus {
    /// The refund was cancelled.
    Cancelled,
    /// The refund is pending. For more information, see status_details.reason.
    Pending,
    /// The funds for this transaction were debited to the customer's account.
    Completed,
    /// The refund could not be processed.
    Failed,
}

/// Refund status reason.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundStatusDetailsReason {
    /// The customer's account is funded through an eCheck, which has not yet cleared.
    Echeck,
}

/// Details about the status of the refund.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct RefundStatusDetails {
    /// The reason why the refund has the PENDING or FAILED status.
    pub reason: RefundStatusDetailsReason,
}

/// Exchange rate.
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct ExchangeRate {
    /// The source currency from which to convert an amount.
    pub source_currency: Currency,
    /// The target currency to which to convert an amount.
    pub target_currency: Currency,
    /// The target currency amount. Equivalent to one unit of the source currency. Formatted as integer or decimal value with one to 15 digits to the right of the decimal point.
    pub value: String,
}

/// The net breakdown of the refund.
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct NetAmountBreakdown {
    /// The converted payable amount.
    pub converted_amount: Money,
    /// The exchange rate that determines the amount that was debited from the merchant's PayPal account.
    pub exchange_rate: ExchangeRate,
    /// The net amount debited from the merchant's PayPal account.
    pub payable_amount: Money,
}

/// The breakdown of the refund.
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct SellerPayableBreakdown {
    /// An array of platform or partner fees, commissions, or brokerage fees for the refund.
    pub platform_fees: Option<Vec<PlatformFee>>,
    /// An array of breakdown values for the net amount. Returned when the currency of the refund is different from the currency of the PayPal account where the payee holds their funds.
    pub net_amount_breakdown: Option<Vec<NetAmountBreakdown>>,
    /// The amount that the payee refunded to the payer.
    pub gross_amount: Option<Money>,
    /// The PayPal fee that was refunded to the payer in the currency of the transaction. This fee might not match the PayPal fee that the payee paid when the payment was captured.
    pub paypal_fee: Option<Money>,
    /// The PayPal fee that was refunded to the payer in the receivable currency. Returned only in cases when the receivable currency is different from transaction currency. Example 'CNY'.
    pub paypal_fee_in_receivable_currency: Option<Money>,
    /// The net amount that the payee's account is debited in the transaction currency. The net amount is calculated as gross_amount minus paypal_fee minus platform_fees.
    pub net_amount: Option<Money>,
    /// The net amount that the payee's account is debited in the receivable currency. Returned only in cases when the receivable currency is different from transaction currency. Example 'CNY'.
    pub net_amount_in_receivable_currency: Option<Money>,
    /// The total amount refunded from the original capture to date. For example, if a payer makes a $100 purchase and was refunded $20 a week ago and was refunded $30 in this refund, the gross_amount is $30 for this refund and the total_refunded_amount is $50.
    pub total_refunded_amount: Money,
}

/// A refund
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Refund {
    /// The status of the refund.
    pub status: RefundStatus,
    /// The details of the refund status.
    pub status_details: Option<RefundStatusDetails>,
    /// The PayPal-generated ID for the refund.
    pub id: String,
    /// The API caller-provided external invoice number for this order. Appears in both the payer's transaction history and the emails that the payer receives.
    pub invoice_id: Option<String>,
    /// The API caller-provided external ID. Used to reconcile API caller-initiated transactions with PayPal transactions. Appears in transaction and settlement reports.
    pub customer_id: Option<String>,
    /// Reference ID issued for the card transaction.
    /// This ID can be used to track the transaction across processors, card brands and issuing banks.
    pub acquirer_reference_number: Option<String>,
    /// The reason for the refund. Appears in both the payer's transaction history and the emails that the payer receives.
    pub note_to_payer: Option<String>,
    /// The breakdown of the refund.
    pub seller_payable_breakdown: SellerPayableBreakdown,
    /// An array of related HATEOAS links.
    pub links: Vec<LinkDescription>,
    /// The amount that the payee refunded to the payer.
    pub amount: Money,
    /// The details associated with the merchant for this transaction.
    pub payer: Option<Payer>,
    /// The date and time when the transaction occurred, in Internet date and time format.
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the transaction was last updated, in Internet date and time format.
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// The comprehensive history of payments for the purchase unit.
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct PaymentCollection {
    /// An array of authorized payments for a purchase unit. A purchase unit can have zero or more authorized payments.
    #[serde(default)]
    pub authorizations: Vec<AuthorizationWithData>,
    /// An array of captured payments for a purchase unit. A purchase unit can have zero or more captured payments.
    #[serde(default)]
    pub captures: Vec<Capture>,
    /// An array of refunds for a purchase unit. A purchase unit can have zero or more refunds.
    #[serde(default)]
    pub refunds: Vec<Refund>,
}
/// Supplementary customer struct.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
pub struct SupplementaryCustomer {
    /// The consumer's IP address, which can be represented in either IPv4 or IPv6 format.
    pub ip_address: Option<String>,
}

/// Supplementary customer parameters
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
pub struct SupplementaryRisk {
    /// Profile information of the sender or receiver.
    pub customer: Option<SupplementaryCustomer>,
}

/// Supplementary data about this payment.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
pub struct SupplementaryData {
    // todo: add support for card 2 and card 3
    /// The level 2 card processing data collections.
    /// If your merchant account has been configured for
    /// Level 2 processing this field will be passed to the processor on your behalf.
    /// Please contact your PayPal Technical Account Manager to define level 2 data for your business.
    pub level_2: Option<serde_json::Value>,
    /// The level 3 card processing data collections,
    /// If your merchant account has been configured for
    /// Level 3 processing this field will be passed to the processor on your behalf.
    /// Please contact your PayPal Technical Account Manager to define level 3 data for your business.
    pub level_3: Option<serde_json::Value>,
    /// Merchants and partners can add additional customer parameters that can help with better fraud protection and reduced risk for unbranded card payments.
    pub risk: Option<SupplementaryRisk>,
}

/// Represents either a full or partial order that the payer intends to purchase from the payee.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct PurchaseUnit {
    /// The API caller-provided external ID for the purchase unit. Required for multiple purchase units when you must update the order through PATCH.
    /// If you omit this value and the order contains only one purchase unit, PayPal sets this value to default.
    pub reference_id: Option<String>,
    /// The purchase description.
    pub description: Option<String>,
    /// The API caller-provided external ID. Used to reconcile client transactions with PayPal transactions.
    /// Appears in transaction and settlement reports but is not visible to the payer.
    pub custom_id: Option<String>,
    /// The API caller-provided external invoice number for this order.
    /// Appears in both the payer's transaction history and the emails that the payer receives.
    pub invoice_id: Option<String>,
    /// The PayPal-generated ID for the purchase unit.
    /// This ID appears in both the payer's transaction history and the emails that the payer receives.
    /// In addition, this ID is available in transaction and settlement reports that merchants and API callers can use to reconcile transactions.
    /// This ID is only available when an order is saved by calling v2/checkout/orders/id/save.
    pub id: Option<String>,
    /// The soft descriptor is the dynamic text used to construct the statement descriptor that appears on a payer's card statement.
    ///
    /// More info here: <https://developer.paypal.com/docs/api/orders/v2/#definition-purchase_unit_request>
    pub soft_descriptor: Option<String>,
    /// An array of items that the customer purchases from the merchant.
    pub items: Option<Vec<Item>>,
    /// The total order amount with an optional breakdown that provides details, such as the total item amount,
    /// total tax amount, shipping, handling, insurance, and discounts, if any.
    ///
    /// If you specify amount.breakdown, the amount equals item_total plus tax_total plus shipping plus handling plus insurance minus shipping_discount minus discount.
    ///
    /// The amount must be a positive number. For listed of supported currencies and decimal precision,
    /// see the PayPal REST APIs [Currency Codes](https://developer.paypal.com/docs/integration/direct/rest/currency-codes/).
    pub amount: Amount,
    /// The merchant who receives payment for this transaction.
    pub payee: Option<Payee>,
    /// Any additional payment instructions for PayPal Commerce Platform customers.
    /// Enables features for the PayPal Commerce Platform, such as delayed disbursement and collection of a platform fee.
    /// Applies during order creation for captured payments or during capture of authorized payments.
    pub payment_instruction: Option<PaymentInstruction>,
    /// The name and address of the person to whom to ship the items.
    pub shipping: Option<ShippingDetail>,
    /// The comprehensive history of payments for the purchase unit.
    pub payments: Option<PaymentCollection>,
}

impl PurchaseUnit {
    /// Creates a new PurchaseUnit with the required properties.
    pub fn new(amount: Amount) -> Self {
        Self {
            amount,
            ..Default::default()
        }
    }
}

/// The type of landing page to show on the PayPal site for customer checkout.
#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LandingPage {
    /// When the customer clicks PayPal Checkout, the customer is redirected to a page to log in to PayPal and approve the payment.
    Login,
    /// When the customer clicks PayPal Checkout, the customer is redirected to a page
    /// to enter credit or debit card and other relevant billing information required to complete the purchase.
    Billing,
    /// When the customer clicks PayPal Checkout, the customer is redirected to either a page to log in to PayPal and approve
    /// the payment or to a page to enter credit or debit card and other relevant billing information required to complete the purchase,
    /// depending on their previous interaction with PayPal.
    #[default]
    NoPreference,
}

/// The shipping preference
#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShippingPreference {
    /// Use the customer-provided shipping address on the PayPal site.
    #[default]
    GetFromFile,
    /// Redact the shipping address from the PayPal site. Recommended for digital goods.
    NoShipping,
    ///  Use the merchant-provided address. The customer cannot change this address on the PayPal site.
    SetProvidedAddress,
}

/// Configures a Continue or Pay Now checkout flow.
#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserAction {
    /// After you redirect the customer to the PayPal payment page, a Continue button appears. Use this option when
    /// the final amount is not known when the checkout flow is initiated and you want to redirect the customer
    /// to the merchant page without processing the payment.
    #[default]
    Continue,
    /// After you redirect the customer to the PayPal payment page, a Pay Now button appears.
    /// Use this option when the final amount is known when the checkout is initiated and you want to
    /// process the payment immediately when the customer clicks Pay Now.
    PayNow,
}
/// The merchant-preferred payment sources.
#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayeePreferred {
    /// Accepts any type of payment from the customer.
    #[default]
    Unrestricted,
    /// Accepts only immediate payment from the customer.
    /// For example, credit card, PayPal balance, or instant ACH.
    /// Ensures that at the time of capture, the payment does not have the `pending` status.
    ImmediatePaymentRequired,
}

/// A payment method.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PaymentMethod {
    /// The customer-selected payment method on the merchant site.
    pub payer_selected: Option<String>,
    /// The merchant-preferred payment sources.
    pub payee_preferred: Option<PayeePreferred>,
}

/// Customize the payer experience during the approval process for the payment with PayPal.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct ApplicationContext {
    /// The label that overrides the business name in the PayPal account on the PayPal site.
    pub brand_name: Option<String>,
    /// The BCP 47-formatted locale of pages that the PayPal payment experience shows. PayPal supports a five-character code.
    ///
    /// For example, da-DK, he-IL, id-ID, ja-JP, no-NO, pt-BR, ru-RU, sv-SE, th-TH, zh-CN, zh-HK, or zh-TW.
    pub locale: Option<String>,
    /// The type of landing page to show on the PayPal site for customer checkout
    pub landing_page: Option<LandingPage>,
    /// The shipping preference
    pub shipping_preference: Option<ShippingPreference>,
    /// Configures a Continue or Pay Now checkout flow.
    pub user_action: Option<UserAction>,
    /// The customer and merchant payment preferences.
    pub payment_method: Option<PaymentMethod>,
    /// The URL where the customer is redirected after the customer approves the payment.
    pub return_url: Option<String>,
    /// The URL where the customer is redirected after the customer cancels the payment.
    pub cancel_url: Option<String>,
}

/// A card used in payment sources.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct PaymentCard {
    /// The card number.
    pub number: String,
    /// The expiry date.
    pub expiry: String,
    /// The card owner name.
    pub name: String,
    /// The billing address.
    pub billing_address: Address,
}

/// A transaction reference.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct TransactionReference {
    /// The transaction id.
    pub id: String,
    /// The transaction network, e.g "VISA"
    pub network: String,
}

/// A stored credential.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct StoredCredential {
    /// The payment initiator, e.g "MERCHANT"
    pub payment_initiator: String,
    /// The payment type, e.g "RECURRING"
    pub payment_type: String,
    /// The stored credential usage, e.g: SUBSEQUENT
    pub usage: String,
    /// The billing address.
    pub previous_network_transaction_reference: TransactionReference,
}

/// A order payload to be used when creating an order.
// TODO: this only appears in the example body, not documented.
// https://developer.paypal.com/docs/api/orders/v2/#orders_create
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct OrderPaymentSource {
    /// The card used in the payment.
    pub card: PaymentCard,
    /// A stored credential.
    // TODO: figure out what is this.
    #[builder(default)]
    pub stored_credential: Option<StoredCredential>,
}

/// A order payload to be used when creating an order.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct OrderPayload {
    /// The intent to either capture payment immediately or authorize a payment for an order after order creation.
    pub intent: Intent,
    /// DEPRECATED. The customer who approves and pays for the order. The customer is also known as the payer.
    #[builder(default)]
    pub payer: Option<Payer>,
    /// An array of purchase units. Each purchase unit establishes a contract between a payer and the payee.
    /// Each purchase unit represents either a full or partial order that the payer intends to purchase from the payee.
    pub purchase_units: Vec<PurchaseUnit>,
    /// Customize the payer experience during the approval process for the payment with PayPal.
    #[builder(default)]
    pub application_context: Option<ApplicationContext>,
    /// The payment source.
    #[builder(default)]
    pub payment_source: Option<OrderPaymentSource>,
}

/// The card brand or network.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardBrand {
    /// Visa card.
    Visa,
    /// Mastecard card.
    Mastercard,
    /// Discover card.
    Discover,
    /// American Express card.
    Amex,
    /// Solo debit card.
    Solo,
    /// Japan Credit Bureau card.
    JCB,
    /// Military Star card.
    Star,
    /// Delta Airlines card.
    Delta,
    /// Switch credit card.
    Switch,
    /// Maestro credit card.
    Maestro,
    /// Carte Bancaire (CB) credit card.
    CbNationale,
    /// Configoga credit card.
    Configoga,
    /// Confidis credit card.
    Confidis,
    /// Visa Electron credit card.
    Electron,
    /// Cetelem credit card.
    Cetelem,
    /// China union pay credit card.
    ChinaUnionPay,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum CardType {
    Credit,
    Debit,
    Prepaid,
    Unknown,
}

/// The payment card to use to fund a payment.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CardResponse {
    /// The last digits of the payment card.
    pub last_digits: String,
    /// The card brand or network.
    pub brand: CardBrand,
    /// The payment card type.
    #[serde(rename = "type")]
    pub card_type: CardType,
}

/// The customer's wallet used to fund the transaction.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletResponse {
    /// Apple Pay Wallet response information.
    pub apple_pay: CardResponse,
}

/// The paypal account used to fund the transaction.
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct PaypalPaymentSourceResponse {
    /// The name of the payer.
    pub name: PayerName,
    /// The email address of the payer.
    pub email_address: String,
    /// The account id of the payer.
    pub account_id: String,
}

/// The payment source used to fund the payment.
#[derive(Debug, Serialize, Deserialize, Builder, Default, Clone)]
#[builder(setter(strip_option), default)]
pub struct PaymentSourceResponse {
    // /// The payment card to use to fund a payment. Card can be a credit or debit card
    // pub card: Option<CardResponse>,
    // /// The customer's wallet used to fund the transaction.
    // pub wallet: Option<WalletResponse>,
    // /// The paypal account used to fund the transaction.
    // pub paypal: Option<PaypalPaymentSourceResponse>,
    /// The payment card to use to fund a payment. Card can be a credit or debit card.
    pub card: Option<serde_json::Value>,

    /// Information used to pay Bancontact.
    pub bancontact: Option<serde_json::Value>,

    /// Information used to pay using BLIK.
    pub blik: Option<serde_json::Value>,

    /// Information used to pay using eps.
    pub eps: Option<serde_json::Value>,

    /// Information needed to pay using giropay.
    pub giropay: Option<serde_json::Value>,

    /// Information used to pay using iDEAL.
    pub ideal: Option<serde_json::Value>,

    /// Information used to pay using MyBank.
    pub mybank: Option<serde_json::Value>,

    /// Information used to pay using P24(Przelewy24).
    pub p24: Option<serde_json::Value>,

    /// Information used to pay using Sofort.
    pub sofort: Option<serde_json::Value>,

    /// Information needed to pay using Trustly.
    pub trustly: Option<serde_json::Value>,

    /// Venmo wallet response.
    pub venmo: Option<serde_json::Value>,

    /// The PayPal Wallet response.
    pub paypal: Option<serde_json::Value>,

    /// Information needed to pay using ApplePay.
    pub apple_pay: Option<serde_json::Value>,

    /// Google Pay Wallet payment data.
    pub google_pay: Option<serde_json::Value>,
}

/// The status of an order.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    /// The order was created with the specified context.
    Created,
    /// The order was saved and persisted. The order status continues to be in progress until a capture
    /// is made with final_capture = true for all purchase units within the order.
    Saved,
    /// The customer approved the payment through the PayPal wallet or another form of guest or unbranded payment. For example, a card, bank account, or so on.
    Approved,
    /// All purchase units in the order are voided.
    Voided,
    /// The payment was authorized or the authorized payment was captured for the order.
    Completed,
}

/// An order represents a payment between two or more parties.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Order {
    /// The date and time when the transaction occurred.
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the transaction was last updated.
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The ID of the order.
    pub id: String,
    /// An array of purchase units. Each purchase unit establishes a contract between a customer and merchant.
    /// Each purchase unit represents either a full or partial order that the customer intends to purchase from the merchant.
    pub purchase_units: Option<Vec<PurchaseUnit>>,
    /// An array of request-related HATEOAS links. To complete payer approval, use the approve link to redirect the payer.
    pub links: Vec<LinkDescription>,
    /// The payment source used to fund the payment.
    pub payment_source: Option<PaymentSourceResponse>,
    /// The intent to either capture payment immediately or authorize a payment for an order after order creation.
    pub intent: Option<Intent>,
    /// The customer who approves and pays for the order. The customer is also known as the payer.
    pub payer: Option<Payer>,
    /// The order status.
    pub status: OrderStatus,
}

/// An invoice number.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InvoiceNumber {
    /// The invoice number.
    pub invoice_number: String,
}
