#[cfg(feature = "embed")]
pub use khiin_data::CONVERSIONS_CSV;

#[cfg(feature = "embed")]
pub use khiin_data::INPUTS_CSV;

#[cfg(not(feature = "embed"))]
pub static CONVERSIONS_CSV: &'static str = "feature = embed not enabled";

#[cfg(not(feature = "embed"))]
pub static INPUTS_CSV: &'static str = "feature = embed not enabled";
