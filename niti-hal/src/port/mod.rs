//! GPIO & Pin control.
//!
//! This module contains a [`Pins`] struct which represents all pins of the board.  The [`Pins`]
//! struct is most easily constructed using the [`niti_hal::pins!()`][crate::pins] macro:
//!
//! ```no_run
//! let dp = niti_hal::Peripherals::take().unwrap();
//! let pins = niti_hal::pins!(dp);
//! ```
//!
//! Additionally, the [`mode`] submodule contains all valid types for the `MODE` generic parameter
//! of a pin.  The [`Pin`] type-alias represents a pin which can represent _any_ of the pins
//! dynamically (while usually each pin has its own type).
//!
//! Check the documentation for [`avr_hal_generic::port::Pin`] for a detailed explanation of GPIO
//! pins in `avr-hal`.





#[cfg(any(feature = "arduino-mega2560", feature = "arduino-mega1280"))]
mod mega;
#[cfg(any(feature = "arduino-mega2560", feature = "arduino-mega1280"))]
pub use mega::*;

#[cfg(any(feature = "niti-v1"))]
mod niti;
#[cfg(any(feature = "niti-v1"))]
pub use niti::*;

#[cfg(any(feature = "niti-v1", feature = "niti-v1"))]
pub use niti::*;
#[cfg(any(
    feature = "arduino-nano",
    feature = "arduino-uno",
    feature = "nano168",
 
))]
mod uno;
#[cfg(any(
    feature = "arduino-nano",
    feature = "arduino-uno",
    feature = "nano168",

))]
pub use uno::*;


