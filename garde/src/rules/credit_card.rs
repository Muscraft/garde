//! Credit card validation using the [`card_validate`] crate.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(credit_card)]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`CreditCard`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(credit_card)]` rule.
//!
//! This trait has a blanket implementation for all `T: AsRef<str>`.

use std::fmt::Display;

use crate::error::Error;

pub fn apply<T: CreditCard>(v: &T, _: ()) -> Result<(), Error> {
    if let Err(e) = v.validate_credit_card() {
        return Err(Error::new(format!("not a valid credit card number: {e}")));
    }
    Ok(())
}

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support credit card validation",
        label = "This type does not support credit card validation",
    )
)]
pub trait CreditCard {
    type Error: Display;

    fn validate_credit_card(&self) -> Result<(), Self::Error>;
}

impl<T: AsRef<str>> CreditCard for T {
    type Error = InvalidCard;

    fn validate_credit_card(&self) -> Result<(), Self::Error> {
        let _ = card_validate::Validate::from(self.as_ref())?;
        Ok(())
    }
}

pub struct InvalidCard(card_validate::ValidateError);
impl Display for InvalidCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            card_validate::ValidateError::InvalidFormat => write!(f, "invalid format"),
            card_validate::ValidateError::InvalidLength => write!(f, "invalid length"),
            card_validate::ValidateError::InvalidLuhn => write!(f, "invalid luhn"),
            card_validate::ValidateError::UnknownType => write!(f, "unknown type"),
            _ => write!(f, "unknown error"),
        }
    }
}

impl From<card_validate::ValidateError> for InvalidCard {
    fn from(value: card_validate::ValidateError) -> Self {
        Self(value)
    }
}
