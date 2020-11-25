mod checkers_pattern;
mod color;
mod colors;
mod flat_pattern;
mod gradient_pattern;
mod material;
mod pattern;
mod ring_pattern;
mod stripe_pattern;

pub use checkers_pattern::CheckersPattern;
pub use color::Color;
pub use colors::*;
pub use flat_pattern::FlatPattern;
pub use gradient_pattern::GradientPattern;
pub use material::Material;
pub use pattern::Pattern;
pub use ring_pattern::RingPattern;
pub use stripe_pattern::StripePattern;

#[cfg(test)]
mod checkers_pattern_test;

#[cfg(test)]
mod color_test;

#[cfg(test)]
mod gradient_pattern_test;

#[cfg(test)]
mod material_test;

#[cfg(test)]
mod ring_pattern_test;

#[cfg(test)]
mod stripe_pattern_test;
