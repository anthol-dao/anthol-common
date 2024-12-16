#![warn(clippy::float_cmp)]

use candid::CandidType;
use num_rational::Rational64;
use num_traits::{FromPrimitive, Signed, ToPrimitive};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

const CONVERSION_DECIMAL_ERROR: &str = "Conversion to Decimal failed";
const CONVERSION_RATIONAL_ERROR: &str = "Conversion to Rational64 failed";
const INVALID_PRICE_ERROR: &str = "Invalid price value";
const NEGATIVE_PRICE_ERROR: &str = "Price cannot be negative";
const NEGATIVE_MULTIPLICATION_ERROR: &str = "Multiplication by a negative scalar is not allowed";
const NEGATIVE_DIVISION_ERROR: &str = "Division by a negative scalar is not allowed";

/// A struct representing a price value.
///
/// The price is stored as a 64-bit floating-point non-negative number (f64).
/// This struct provides methods to create, manipulate, and convert the price value.
///
/// When calculating prices directly, all values are converted to Decimal or Rational types each time.
/// You should make explicit conversions to these when performing complex calculations.
#[derive(CandidType, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Debug, Copy)]
pub struct Price(f64);

impl Price {
    /// Creates a new `Price` instance.
    ///
    /// # Arguments
    ///
    /// * `price` - A floating-point number representing the price.
    ///
    /// # Panics
    ///
    /// Panics if the provided price is NaN, infinity or negative.
    pub fn new(price: f64) -> Self {
        if let Err(e) = validate_f64(price) {
            panic!("{}: {}", INVALID_PRICE_ERROR, e);
        };
        if price < 0.0 {
            panic!("{}: {}", INVALID_PRICE_ERROR, NEGATIVE_PRICE_ERROR);
        }
        Self(price)
    }

    /// Returns the price as a floating-point number (f64).
    pub fn get_f64(&self) -> f64 {
        self.0
    }

    /// Converts the price to a `Decimal`.
    /// Returns `None` if the conversion fails.
    pub fn to_decimal(&self) -> Option<Decimal> {
        Decimal::from_f64(self.0)
    }

    /// Converts the price to a `Rational64`.
    /// Returns `None` if the conversion fails.
    pub fn to_rational(&self) -> Option<Rational64> {
        Rational64::from_f64(self.0)
    }
}

impl Default for Price {
    fn default() -> Self {
        Self(0.0)
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

impl Add for Price {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let result = self.to_decimal().expect(CONVERSION_DECIMAL_ERROR)
            + other.to_decimal().expect(CONVERSION_DECIMAL_ERROR);
        Price::from(result)
    }
}

impl Sub for Price {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let result = self.to_decimal().expect(CONVERSION_DECIMAL_ERROR)
            - other.to_decimal().expect(CONVERSION_DECIMAL_ERROR);
        if result.is_sign_negative() {
            println!("Price is negative, returning 0.0");
            Price::from(0.0)
        } else {
            Price::from(result)
        }
    }
}

impl Mul<Decimal> for Price {
    type Output = Self;

    fn mul(self, scalar: Decimal) -> Self::Output {
        if scalar.is_sign_negative() {
            panic!("{}", NEGATIVE_MULTIPLICATION_ERROR);
        }
        let result = self.to_decimal().expect(CONVERSION_DECIMAL_ERROR) * scalar;
        Price::from(result)
    }
}

impl Mul<f64> for Price {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        if let Err(e) = validate_f64(scalar) {
            panic!("Invalid scalar value: {}", e);
        };
        if scalar < 0.0 {
            panic!("{}", NEGATIVE_MULTIPLICATION_ERROR);
        }
        let result = self.to_decimal().expect(CONVERSION_DECIMAL_ERROR)
            * Decimal::from_f64(scalar).expect(CONVERSION_DECIMAL_ERROR);
        Price::from(result)
    }
}

impl Mul<Rational64> for Price {
    type Output = Price;

    fn mul(self, scalar: Rational64) -> Self::Output {
        if scalar.is_negative() {
            panic!("{}", NEGATIVE_MULTIPLICATION_ERROR);
        }
        Price::from(self.to_rational().expect(CONVERSION_RATIONAL_ERROR) * scalar)
    }
}

impl Div<Rational64> for Price {
    type Output = Price;

    fn div(self, scalar: Rational64) -> Self::Output {
        if scalar.is_negative() {
            panic!("{}", NEGATIVE_DIVISION_ERROR);
        }
        Price::from(self.to_rational().expect(CONVERSION_RATIONAL_ERROR) / scalar)
    }
}

impl Div<Price> for Price {
    type Output = Rational64;

    fn div(self, other: Price) -> Self::Output {
        self.to_rational().expect(CONVERSION_RATIONAL_ERROR)
            / other.to_rational().expect(CONVERSION_RATIONAL_ERROR)
    }
}

impl From<f64> for Price {
    fn from(price: f64) -> Self {
        if let Err(e) = validate_f64(price) {
            panic!("{}: {}", INVALID_PRICE_ERROR, e);
        };
        if price < 0.0 {
            panic!("{}: {}", INVALID_PRICE_ERROR, NEGATIVE_PRICE_ERROR);
        }
        Price::new(price)
    }
}

impl From<Price> for f64 {
    fn from(price: Price) -> Self {
        price.0
    }
}

impl From<Decimal> for Price {
    fn from(decimal: Decimal) -> Self {
        if decimal.is_sign_negative() {
            panic!("{}: {}", INVALID_PRICE_ERROR, NEGATIVE_PRICE_ERROR);
        }
        Price::from(decimal.to_f64().expect(CONVERSION_DECIMAL_ERROR))
    }
}

impl From<Rational64> for Price {
    fn from(rational: Rational64) -> Self {
        if rational.is_negative() {
            panic!("{}: {}", INVALID_PRICE_ERROR, NEGATIVE_PRICE_ERROR);
        }
        Price::from(rational.to_f64().expect(CONVERSION_RATIONAL_ERROR))
    }
}

#[cfg(feature = "wasm-bindgen")]
impl From<Price> for js_sys::Number {
    fn from(price: Price) -> js_sys::Number {
        js_sys::Number::from(price.0)
    }
}

/// Validates a floating-point number (f64).
///
/// Ensures the value is not NaN or infinite.
///
/// # Arguments
///
/// * `value` - The floating-point number to validate.
///
/// # Returns
///
/// * `Ok(())` if the value is valid.
/// * `Err(&'static str)` if the value is NaN or infinity.
fn validate_f64(value: f64) -> Result<(), &'static str> {
    if value.is_nan() {
        return Err("Value is NaN");
    }
    if value.is_infinite() {
        return Err("Value is infinity");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

    use super::*;
    use num_rational::Rational64;
    use rust_decimal::Decimal;

    #[test]
    fn test_price_new_valid() {
        let price = Price::new(10.0);
        assert_eq!(price.get_f64(), 10.0);
    }

    #[test]
    #[should_panic(expected = "Invalid price value: Value is NaN")]
    fn test_price_new_nan() {
        Price::new(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "Invalid price value: Value is infinity")]
    fn test_price_new_infinity() {
        Price::new(f64::INFINITY);
    }

    #[test]
    #[should_panic(expected = "Invalid price value: Price cannot be negative")]
    fn test_price_new_negative() {
        Price::new(-10.0);
    }

    #[test]
    fn test_to_decimal() {
        let price = Price::new(10.0);
        let decimal = price.to_decimal().unwrap();
        assert_eq!(decimal, Decimal::from_f64(10.0).unwrap());
    }

    #[test]
    fn test_to_rational() {
        let price = Price::new(10.0);
        let rational = price.to_rational().unwrap();
        assert_eq!(rational, Rational64::from_f64(10.0).unwrap());
    }

    #[test]
    fn test_add_prices() {
        let price1 = Price::new(10.0);
        let price2 = Price::new(5.0);
        let result = price1 + price2;
        assert_eq!(result.get_f64(), 15.0);
    }

    #[test]
    fn test_sub_prices() {
        let price1 = Price::new(10.0);
        let price2 = Price::new(5.0);
        let result = price1 - price2;
        assert_eq!(result.get_f64(), 5.0);
    }

    #[test]
    fn test_sub_prices_negative_result() {
        let price1 = Price::new(5.0);
        let price2 = Price::new(10.0);
        let result = price1 - price2;
        assert_eq!(result.get_f64(), 0.0);
    }

    #[test]
    fn test_mul_price_decimal() {
        let price = Price::new(10.0);
        let scalar = Decimal::new(2, 0);
        let result = price * scalar;
        assert_eq!(result.get_f64(), 20.0);
    }

    #[test]
    fn test_mul_price_f64() {
        let price = Price::new(10.0);
        let scalar = 2.0;
        let result = price * scalar;
        assert_eq!(result.get_f64(), 20.0);
    }

    #[test]
    fn test_mul_price_rational() {
        let price = Price::new(10.0);
        let scalar = Rational64::from_integer(2);
        let result = price * scalar;
        assert_eq!(result.get_f64(), 20.0);
    }

    #[test]
    fn test_div_price_rational() {
        let price = Price::new(10.0);
        let scalar = Rational64::from_integer(2);
        let result = price / scalar;
        assert_eq!(result.get_f64(), 5.0);
    }

    #[test]
    fn test_div_prices() {
        let price1 = Price::new(10.0);
        let price2 = Price::new(2.0);
        let result = price1 / price2;
        assert_eq!(result, Rational64::from_integer(5));
    }
}
