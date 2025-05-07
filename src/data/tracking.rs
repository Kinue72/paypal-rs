//! This module contains the defined for tracking schema.

use crate::data::common::ItemUpc;
use crate::data::shipment_carrier::ShipmentCarrier;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The tracking information for the order.
#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct OrderTracking {
    /// The tracking number for the shipment. This property supports Unicode.
    pub tracking_number: String,
    /// The name of the carrier for the shipment. Provide this value only if the carrier parameter is OTHER. This property supports Unicode.
    pub carrier_name_other: Option<String>,
    /// The carrier for the shipment. Some carriers have a global version as well as local subsidiaries.
    /// The subsidiaries are repeated over many countries and might also have an entry in the global list.
    /// Choose the carrier for your country.
    /// If the carrier is not available for your country, choose the global version of the carrier.
    /// If your carrier name is not in the list, set carrier to OTHER and set carrier name in carrier_name_other.
    /// For allowed values, see Carriers.
    pub carrier: ShipmentCarrier,
    /// The PayPal capture ID.
    pub capture_id: String,
    /// If true, PayPal will send an email notification to the payer of the PayPal transaction.
    /// The email contains the tracking details provided through the Orders tracking API request.
    /// Independent of any value passed for notify_payer, the payer may receive tracking notifications within the PayPal app, based on the user's notification preferences.
    pub notify_payer: Option<bool>,
    /// An array of details of items in the shipment.
    pub items: Option<Vec<ShipmentItem>>,
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
