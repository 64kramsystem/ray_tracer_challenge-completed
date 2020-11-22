mod color;
mod colors;
mod flat_pattern;
mod material;
mod pattern;
mod stripe_pattern;

pub use color::Color;
pub use colors::{COLOR_BLACK, COLOR_BLUE, COLOR_GREEN, COLOR_RED, COLOR_WHITE};
pub use flat_pattern::FlatPattern;
pub use material::Material;
pub use pattern::Pattern;
pub use stripe_pattern::StripePattern;

#[cfg(test)]
mod color_test;

#[cfg(test)]
mod material_test;

#[cfg(test)]
mod stripe_pattern_test;
