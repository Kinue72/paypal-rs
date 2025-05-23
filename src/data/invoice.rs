//! Paypal object definitions used in the invoice api.

use crate::{data::common::LinkDescription, data::common::*};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Paypal File reference
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileReference {
    /// The ID of the referenced file.
    pub id: String,
    /// The reference URL for the file.
    pub reference_url: String,
    /// Content type
    pub content_type: String,
    /// The date and time when the file was created
    pub create_time: chrono::DateTime<chrono::Utc>,
    /// The size of the file, in bytes.
    pub size: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// The payment term type.
pub enum PaymentTermType {
    /// The payment for the invoice is due upon receipt of the invoice.
    DueOnReceipt,
    /// The payment for the invoice is due on the date specified in the invoice.
    DueOnDateSpecified,
    /// The payment for the invoice is due in 10 days.
    Net10,
    /// The payment for the invoice is due in 15 days.
    Net15,
    /// The payment for the invoice is due in 30 days.
    Net30,
    /// The payment for the invoice is due in 45 days.
    Net45,
    /// The payment for the invoice is due in 60 days.
    Net60,
    /// The payment for the invoice is due in 90 days.
    Net90,
    /// The invoice has no payment due date.
    NoDueDate,
}

/// The payment due date for the invoice.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentTerm {
    /// The payment term. Payment can be due upon receipt, a specified date, or in a set number of days
    pub term_type: PaymentTermType,
    /// The date when the invoice payment is due,
    pub due_date: Option<chrono::NaiveDate>,
}

/// Flow type
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// The flow variation
pub enum FlowType {
    /// The invoice sent to multiple recipients.
    MultipleRecipientsGroup,
    /// The invoice sent as a batch.
    Batch,
    /// The regular invoice sent to single recipient.
    RegularSingle,
}

/// Metadata about a resource
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Metadata {
    /// The date and time when the resource was created
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The email address of the account that created the resource.
    pub created_by: Option<String>,
    /// The date and time when the resource was last edited
    pub last_update_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The email address of the account that last edited the resource.
    pub last_updated_by: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the resource was canceled
    pub cancel_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The actor who canceled the resource.
    pub cancelled_by: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the resource was first sent
    pub first_sent_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the resource was last sent
    pub last_sent_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The email address of the account that last sent the resource.
    pub last_sent_by: Option<String>,
    /// The flow variation that created this invoice
    pub created_by_flow: Option<FlowType>,
    /// The URL for the invoice payer view hosted on paypal.com.
    pub recipient_view_url: Option<String>,
    /// The URL for the invoice merchant view hosted on paypal.com
    pub invoicer_view_url: Option<String>,
}

/// The details of the invoice. Includes the invoice number, date, payment terms, and audit metadata.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder)]
#[builder(setter(strip_option), default)]
pub struct InvoiceDetail {
    /// The reference data. Includes a post office (PO) number.
    pub reference: Option<String>,
    /// The three-character ISO-4217 currency code that identifies the currency.
    pub currency_code: Currency,
    /// A note to the invoice recipient. Also appears on the invoice notification email.
    pub note: Option<String>,
    /// The general terms of the invoice. Can include return or cancellation policy and other terms and conditions.
    pub terms_and_conditions: Option<String>,
    /// A private bookkeeping memo for the user.
    pub memo: Option<String>,
    /// An array of PayPal IDs for the files that are attached to an invoice.
    pub attachments: Option<Vec<FileReference>>,
    /// The invoice number. Default is the number that is auto-incremented number from the last number.
    pub invoice_number: Option<String>,
    /// The invoice date as specificed by the sender
    pub invoice_date: Option<chrono::NaiveDate>,
    /// The payment due date for the invoice.
    pub payment_term: Option<PaymentTerm>,
    /// The audit metadata
    pub metadata: Option<Metadata>,
}

/// A name to be used as recipient, etc.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct Name {
    /// The prefix, or title, to the party's name.
    pub prefix: Option<String>,
    /// When the party is a person, the party's given, or first, name.
    pub given_name: Option<String>,
    /// When the party is a person, the party's surname or family name.
    /// Also known as the last name. Required when the party is a person.
    /// Use also to store multiple surnames including the matronymic, or mother's, surname.
    pub surname: Option<String>,
    /// When the party is a person, the party's middle name. Use also to store multiple middle names including the patronymic, or father's, middle name.
    pub middle_name: Option<String>,
    /// The suffix for the party's name.
    pub suffix: Option<String>,
    /// DEPRECATED. The party's alternate name. Can be a business name, nickname,
    /// or any other name that cannot be split into first, last name. Required when the party is a business.
    pub alternate_full_name: Option<String>,
    /// When the party is a person, the party's full name.
    pub full_name: Option<String>,
}

/// Phone information
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct PhoneDetail {
    /// The country calling code (CC), in its canonical international E.164 numbering plan format.
    pub country_code: String,
    /// The national number, in its canonical international E.164 numbering plan format.
    pub national_number: String,
    /// The extension number.
    pub extension_number: Option<String>,
    /// The phone type.
    pub phone_type: Option<PhoneType>,
}

/// The invoicer information.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder)]
#[builder(setter(strip_option), default)]
pub struct InvoicerInfo {
    /// Required. The business name of the party.
    pub business_name: Option<String>,
    /// The first and Last name of the recipient.
    pub name: Option<Name>,
    /// The invoicer email address, which must be listed in the user's PayPal profile.
    /// If you omit this value, notifications are sent from and to the primary email address but do not appear on the invoice.
    pub email_address: Option<String>,
    /// An array of invoicer's phone numbers. The invoicer can choose to hide the phone number on the invoice.
    pub phones: Option<Vec<PhoneDetail>>,
    /// The invoicer's website.
    pub website: Option<String>,
    /// The invoicer's tax ID.
    pub tax_id: Option<String>,
    /// Any additional information. Includes business hours.
    pub additional_notes: Option<String>,
    /// The full URL to an external logo image. The logo image must not be larger than 250 pixels wide by 90 pixels high.
    pub logo_url: Option<String>,
}

/// Billing information
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BillingInfo {
    /// Required. The business name of the party.
    pub business_name: String,
    /// The first and Last name of the recipient.
    pub name: Option<Name>,
    /// The address of the recipient.
    pub address: Option<Address>,
    /// The invoice recipient email address. If you omit this value, the invoice is payable and a notification email is not sent.
    pub email_address: Option<String>,
    /// The invoice recipient's phone numbers. Extension number is not supported.
    pub phones: Option<Vec<PhoneDetail>>,
    /// Any additional information about the recipient. Maximum length: 40.
    pub additional_info: Option<String>,
    /// The language in which to show the invoice recipient's email message. Used only when the recipient does not have a PayPal account
    pub language: Option<String>,
}

/// Contact information
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContactInformation {
    /// Required. The business name of the party.
    pub business_name: String,
    /// The first and Last name of the recipient.
    pub name: Option<Name>,
    /// The address of the recipient.
    pub address: Option<Address>,
}

/// Recipient information
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RecipientInfo {
    /// The billing information for the invoice recipient. Includes name, address, email, phone, and language.
    pub billing_info: Option<BillingInfo>,
    /// The recipient's shipping information. Includes the user's contact information, which includes name and address.
    pub shipping_info: Option<ContactInformation>,
}

/// Tax information
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tax {
    /// The name of the tax applied on the invoice items.
    pub name: String,
    /// The tax rate. Value is from 0 to 100. Supports up to five decimal places.
    pub percent: String,
    /// The calculated tax amount. The tax amount is added to the item total.
    pub amount: Option<Money>,
}

/// Discount information
#[derive(Debug, Serialize, Deserialize, Clone, Builder, Default)]
#[builder(setter(strip_option, into), default)]
pub struct Discount {
    /// The discount as a percentage value. Value is from 0 to 100. Supports up to five decimal places.
    pub percent: Option<String>,
    /// The invoice level discount amount. Value is from 0 to 1000000. Supports up to two decimal places.
    pub amount: Option<Box<Amount>>,
}

/// The unit of measure for the invoiced item.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UnitOfMeasure {
    /// The unit of measure is quantity. This invoice template is typically used for physical goods.
    Quantity,
    /// The unit of measure is hours. This invoice template is typically used for services.
    Hours,
    /// The unit of measure is amount. This invoice template is typically used when only amount is required.
    Amount,
}

/// Item information
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct Item {
    /// The ID of the invoice line item.
    /// Read only.
    #[builder(default)]
    pub id: Option<String>,
    /// The item name for the invoice line item.
    pub name: String,
    /// The item description for the invoice line item.
    #[builder(default)]
    pub description: Option<String>,
    /// The quantity of the item that the invoicer provides to the payer. Value is from -1000000 to 1000000. Supports up to five decimal places.
    pub quantity: String,
    /// The unit price of the item. This does not include tax and discount. Value is from -1000000 to 1000000. Supports up to two decimal places.
    pub unit_amount: Money,
    /// The tax associated with the item. The tax amount is added to the item total. Value is from 0 to 100. Supports up to five decimal places.
    #[builder(default)]
    pub tax: Option<Tax>,
    /// The date when the item or service was provided, in Internet date and time format.
    #[builder(default)]
    pub item_date: Option<chrono::DateTime<chrono::Utc>>,
    /// Discount as a percent or amount at invoice level. The invoice discount amount is subtracted from the item total.
    #[builder(default)]
    pub discount: Option<Discount>,
    /// The unit of measure for the invoiced item. For AMOUNT the unit_amount and quantity are not shown on the invoice.
    #[builder(default)]
    pub unit_of_measure: Option<UnitOfMeasure>,
}

/// The partial payment details.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct PartialPayment {
    /// Indicates whether the invoice allows a partial payment. If false, the invoice must be paid in full. If true, the invoice allows partial payments.
    pub allow_partial_payment: Option<bool>,
    /// The minimum amount allowed for a partial payment. Valid only when allow_partial_payment is true.
    pub minimum_amount_due: Option<Money>,
}

/// The invoice configuration details. Includes partial payment, tip, and tax calculated after discount.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Default, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct Configuration {
    /// Indicates whether the tax is calculated before or after a discount. If false, the tax is calculated before a discount. If true, the tax is calculated after a discount.
    pub tax_calculated_after_discount: Option<bool>,
    /// Indicates whether the unit price includes tax.
    pub tax_inclusive: Option<bool>,
    /// Indicates whether the invoice enables the customer to enter a tip amount during payment.
    /// If true, the invoice shows a tip amount field so that the customer can enter a tip amount. If false, the invoice does not show a tip amount field.
    pub allow_tip: Option<bool>,
    /// The partial payment details. Includes the minimum amount that the invoicer wants the payer to pay.
    pub partial_payment: Option<PartialPayment>,
    /// The template ID. The template determines the layout of the invoice. Includes which fields to show and hide. Default: PayPal system template.
    pub template_id: Option<String>,
}

/// The discount
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct AggregatedDiscount {
    /// The discount as a percent or amount at invoice level. The invoice discount amount is subtracted from the item total.
    pub invoice_discount: Option<Discount>,
    /// The discount as a percent or amount at item level. The item discount amount is subtracted from each item amount.
    pub item_discount: Option<Money>,
}

/// The shipping fee
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct ShippingCost {
    /// The shipping amount. Value is from 0 to 1000000. Supports up to two decimal places.
    pub amount: Option<Money>,
    /// The tax associated with the shipping.
    pub tax: Option<Tax>,
}

/// The custom amount to apply to an invoice
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct CustomAmount {
    /// The label to the custom amount of the invoice.
    pub label: String,
    /// The custom amount value. Value is from -1000000 to 1000000. Supports up to two decimal places.
    #[builder(default)]
    pub amount: Option<Money>,
}

/// The breakdown of the amount. Breakdown provides details such as total item amount, total tax amount, custom amount, shipping and discounts, if any.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct Breakdown {
    /// The subtotal for all items. Must equal the sum of (items[].unit_amount * items[].quantity) for all items.
    pub item_total: Option<Money>,
    /// The discount can be at the item or invoice level, or both. Can be applied as a percent or amount. If you provide both amount and percent, amount takes precedent.
    pub discount: Option<AggregatedDiscount>,
    /// The aggregated amount of the item and shipping taxes.
    pub tax_total: Option<Money>,
    /// The shipping fee for all items. Includes tax on shipping.
    pub shipping: Option<ShippingCost>,
    /// The custom amount to apply to an invoice. If you include a label, you must include the custom amount.
    pub custom: Option<CustomAmount>,
}

/// Represents an amount of money.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct Amount {
    /// The [three-character ISO-4217 currency code](https://developer.paypal.com/docs/integration/direct/rest/currency-codes/) that identifies the currency.
    pub currency_code: Currency,
    /// The value, which might be:
    /// - An integer for currencies like JPY that are not typically fractional.
    /// - A decimal fraction for currencies like TND that are subdivided into thousandths.
    ///
    /// For the required number of decimal places for a currency code, see [Currency Codes](https://developer.paypal.com/docs/api/reference/currency-codes/).
    pub value: String,
    /// The breakdown of the amount. Breakdown provides details such as total item amount, total tax amount, custom amount, shipping and discounts, if any.
    #[builder(default)]
    pub breakdown: Option<Breakdown>,
}

impl Amount {
    /// Creates a new amount with the required values.
    pub fn new(currency_code: Currency, value: &str) -> Self {
        Amount {
            currency_code,
            value: value.into(),
            breakdown: None,
        }
    }
}

/// The payment type in an invoicing flow
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentType {
    /// The payment type is PayPal.
    Paypal,
    /// The payment type is an external cash or a check payment.
    External,
}

/// The payment mode or method through which the invoicer can accept the payment.
#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentMethod {
    /// Payments can be received through bank transfers.
    BankTransfer,
    /// Payments can be received as cash.
    Cash,
    /// Payments can be received as check.
    Check,
    /// Payments can be received through credit card payments.
    CreditCard,
    /// Payments can be received through debit card payments.
    DebitCard,
    /// Payments can be received through paypal payments.
    #[default]
    Paypal,
    /// Payments can be received through wire transfer.
    WireTransfer,
    /// Payments can be received through other modes.
    Other,
}

/// Payment detail
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct PaymentDetail {
    /// The payment type in an invoicing flow which can be PayPal or an external cash or check payment.
    #[builder(default)]
    pub r#type: Option<PaymentType>,
    /// The ID for a PayPal payment transaction. Required for the PAYPAL payment type.
    #[builder(default)]
    pub payment_id: Option<String>,
    /// The date when the invoice was paid, in Internet date and time format.
    #[builder(default)]
    pub payment_date: Option<chrono::DateTime<chrono::Utc>>,
    /// The payment mode or method through which the invoicer can accept the payment.
    pub method: PaymentMethod,
    /// A note associated with an external cash or check payment.
    #[builder(default)]
    pub note: Option<String>,
    /// The payment amount to record against the invoice. If you omit this parameter, the total invoice amount is marked as paid. This amount cannot exceed the amount due.
    #[builder(default)]
    pub amount: Option<Money>,
    /// The recipient's shipping information. Includes the user's contact information, which includes name and address.
    #[builder(default)]
    pub shipping_info: Option<ContactInformation>,
}

/// Payments registered against the invoice
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct Payments {
    /// The aggregated payment amounts against this invoice.
    /// Read only.
    pub paid_amount: Option<Money>,
    /// An array of payment details for the invoice. The payment details of the invoice like payment type, method, date, discount and transaction type.
    /// Read only.
    pub transactions: Option<Vec<PaymentDetail>>,
}

/// Refund details
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct RefundDetail {
    /// The PayPal refund type. Indicates whether the refund was paid through PayPal or externally in the invoicing flow.
    #[builder(default)]
    pub r#type: Option<PaymentType>,
    /// The ID for a PayPal payment transaction. Required for the PAYPAL payment type.
    #[builder(default)]
    pub refund_id: Option<String>,
    /// The date when the invoice was refunded, in Internet date format.
    #[builder(default)]
    pub refund_date: Option<chrono::DateTime<chrono::Utc>>,
    /// The amount to record as refunded. If you omit the amount, the total invoice paid amount is recorded as refunded.
    #[builder(default)]
    pub amount: Option<Money>,
    /// The payment mode or method through which the invoicer can accept the payments.
    pub method: PaymentMethod,
}

/// List of refunds
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct Refunds {
    /// The aggregated refund amounts.
    /// Read only.
    pub refund_amount: Option<Money>,
    /// An array of refund details for the invoice. Includes the refund type, date, amount, and method.
    /// Read only.
    pub transactions: Option<Vec<RefundDetail>>,
}

/// The status of the invoice
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    ///  The invoice is in draft state. It is not yet sent to the payer.
    Draft,
    /// The invoice has been sent to the payer. The payment is awaited from the payer.
    Sent,
    /// The invoice is scheduled on a future date. It is not yet sent to the payer.
    Scheduled,
    /// The payer has paid for the invoice.
    Paid,
    /// The invoice is marked as paid by the invoicer.
    MarkedAsPaid,
    /// The invoice has been cancelled by the invoicer.
    Cancelled,
    /// The invoice has been refunded by the invoicer.
    Refunded,
    /// The payer has partially paid for the invoice.
    PartiallyPaid,
    /// The invoice has been partially refunded by the invoicer.
    PartiallyRefunded,
    /// The invoice is marked as refunded by the invoicer.
    MarkedAsRefunded,
    /// The invoicer is yet to receive the payment from the payer for the invoice.
    Unpaid,
    /// The invoicer is yet to receive the payment for the invoice. It is under pending review.
    PaymentPending,
}

/// An invoice payload
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder)]
#[builder(setter(strip_option), default)]
pub struct InvoicePayload {
    /// The details of the invoice. Includes the invoice number, date, payment terms, and audit metadata.
    pub detail: InvoiceDetail,
    /// The invoicer information. Includes the business name, email, address, phone, fax, tax ID, additional notes, and logo URL.
    pub invoicer: Option<InvoicerInfo>,
    /// The billing and shipping information. Includes name, email, address, phone and language.
    pub primary_recipient: Option<Vec<RecipientInfo>>,
    /// An array of one or more CC: emails to which notifications are sent.
    /// If you omit this parameter, a notification is sent to all CC: email addresses that are part of the invoice.
    pub additional_recipients: Option<Vec<String>>,
    /// An array of invoice line item information.
    pub items: Vec<Item>,
    /// The invoice configuration details. Includes partial payment, tip, and tax calculated after discount.
    pub configuration: Option<Configuration>,
    /// The invoice amount summary of item total, discount, tax total and shipping..
    pub amount: Option<Amount>,
    /// List of payments registered against the invoice.
    pub payments: Option<Payments>,
    /// List of refunds against this invoice. The invoicing refund details includes refund type, date, amount, and method.
    pub refunds: Option<Refunds>,
}

/// Definition: <https://developer.paypal.com/docs/api/invoicing/v2/#invoices_get>
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option, into))]
pub struct Invoice {
    /// The ID of the invoice.
    pub id: String,
    /// The parent ID to an invoice that defines the group invoice to which the invoice is related.
    #[builder(default)]
    pub parent_id: Option<String>,
    /// The status of the invoice.
    pub status: Status,
    /// The details of the invoice. Includes the invoice number, date, payment terms, and audit metadata.
    pub detail: InvoiceDetail,
    /// The invoicer information. Includes the business name, email, address, phone, fax, tax ID, additional notes, and logo URL.
    #[builder(default)]
    pub invoicer: Option<InvoicerInfo>,
    /// The billing and shipping information. Includes name, email, address, phone and language.
    #[builder(default)]
    pub primary_recipients: Option<Vec<RecipientInfo>>,
    /// An array of one or more CC: emails to which notifications are sent.
    /// If you omit this parameter, a notification is sent to all CC: email addresses that are part of the invoice.
    #[builder(default)]
    pub additional_recipients: Option<Vec<String>>,
    /// An array of invoice line item information.
    #[builder(default)]
    pub items: Option<Vec<Item>>,
    /// The invoice configuration details. Includes partial payment, tip, and tax calculated after discount.
    #[builder(default)]
    pub configuration: Option<Configuration>,
    /// The invoice amount summary of item total, discount, tax total and shipping..
    pub amount: Amount,
    /// The due amount, which is the balance amount outstanding after payments.
    #[builder(default)]
    pub due_amount: Option<Money>,
    /// The amount paid by the payer as gratuity to the invoicer.
    #[builder(default)]
    pub gratuity: Option<Money>,
    /// List of payments registered against the invoice..
    #[builder(default)]
    pub payments: Option<Payments>,
    /// List of refunds against this invoice. The invoicing refund details includes refund type, date, amount, and method.
    #[builder(default)]
    pub refunds: Option<Refunds>,
    /// An array of request-related HATEOAS links.
    #[builder(default)]
    pub links: Option<Vec<LinkDescription>>,
}

/// A invoice list
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct InvoiceList {
    /// Total items
    pub total_items: i32,
    /// Total pages
    pub total_pages: i32,
    /// The invoices
    pub items: Vec<Invoice>,
    /// HATEOAS links
    pub links: Vec<LinkDescription>,
}

/// Cancel invoice reason
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct CancelReason {
    /// The subject of the email that is sent as a notification to the recipient.
    pub subject: Option<String>,
    /// A note to the payer.
    pub note: Option<String>,
    /// Indicates whether to send a copy of the email to the merchant.
    pub send_to_invoicer: Option<bool>,
    /// Indicates whether to send a copy of the email to the recipient.
    pub send_to_recipient: Option<bool>,
    /// An array of one or more CC: emails to which notifications are sent.
    /// If you omit this parameter, a notification is sent to all CC: email addresses that are part of the invoice.
    pub additional_recipients: Option<Vec<String>>,
}

/// QR pay action
pub const QR_ACTION_PAY: &str = "pay";
/// QR details action
pub const QR_ACTION_DETAILS: &str = "details";

/// QR creation parameters
#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder)]
pub struct QRCodeParams {
    /// The width, in pixels, of the QR code image. Value is from 150 to 500.
    pub width: i32,
    /// The height, in pixels, of the QR code image. Value is from 150 to 500.
    pub height: i32,
    /// The type of URL for which to generate a QR code. Valid values are pay and details.
    ///
    /// Check QR_ACTION_PAY and QR_ACTION_DETAILS constants
    pub action: Option<String>,
}

/// Used to record a payment.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone, Builder)]
pub struct RecordPaymentPayload {
    /// The payment id.
    pub payment_id: Option<String>,
    /// The payment date
    pub payment_date: Option<chrono::DateTime<chrono::Utc>>,
    /// The payment method.
    pub method: PaymentMethod,
    /// A note.
    pub note: Option<String>,
    /// The amount.
    pub amount: Amount,
    /// The shipping info.
    pub shipping_info: Option<ContactInformation>,
}

/// Send Invoice Payload
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Builder, Clone)]
pub struct SendInvoicePayload {
    /// An array of one or more CC: emails to which notifications are sent.
    /// If you omit this parameter, a notification is sent to all CC: email addresses that are part of the invoice.
    pub additional_recipients: Option<Vec<String>>,
    /// A note to the payer.
    pub note: Option<String>,
    /// Indicates whether to send a copy of the email to the merchant.
    pub send_to_invoicer: Option<bool>,
    /// Indicates whether to send a copy of the email to the recipient.
    pub send_to_recipient: Option<bool>,
    /// The subject of the email that is sent as a notification to the recipient.
    pub subject: Option<String>,
}
