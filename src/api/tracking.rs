//! This module contains the api for order tracking.

#![allow(dead_code)]

use crate::data::orders::Order;
use crate::data::tracking::OrderTracking;
use crate::endpoint::Endpoint;
use derive_builder::Builder;
use std::borrow::Cow;

/// Update tracking for an order.
#[derive(Debug, Clone, Builder)]
pub struct AddOrderTracking {
    /// The id of the order.
    pub order_id: String,
    /// The endpoint body.
    pub body: OrderTracking,
}

impl AddOrderTracking {
    /// New constructor.
    pub fn new(order_id: &str, body: OrderTracking) -> Self {
        Self {
            order_id: order_id.to_string(),
            body,
        }
    }
}

impl Endpoint for AddOrderTracking {
    type Query = ();

    type Body = OrderTracking;

    type Response = Order;

    fn relative_path(&self) -> Cow<str> {
        Cow::Owned(format!("/v2/checkout/orders/{}/track", self.order_id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(self.body.clone())
    }
}
