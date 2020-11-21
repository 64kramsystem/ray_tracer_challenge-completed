mod color;
mod material;
mod pattern;
mod stripe_pattern;

pub use color::{Color, COLOR_BLACK, COLOR_WHITE};
pub use material::Material;
pub use pattern::Pattern;
pub use stripe_pattern::StripePattern;

#[cfg(test)]
mod color_test;

#[cfg(test)]
mod material_test;

#[cfg(test)]
mod stripe_pattern_test;
