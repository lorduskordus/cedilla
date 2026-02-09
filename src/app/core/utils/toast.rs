// SPDX-License-Identifier: GPL-3.0

use cosmic::widget::Toast;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct CedillaToast {
    pub message: String,
}

impl CedillaToast {
    pub fn new<T>(message: T) -> Self
    where
        T: ToString,
    {
        Self {
            message: message.to_string(),
        }
    }
}

impl From<CedillaToast> for Toast<crate::app::Message> {
    fn from(toast: CedillaToast) -> Self {
        Toast::new(toast.message).duration(Duration::from_secs(5))
    }
}
