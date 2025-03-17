// This module acts as a facade that re-exports the appropriate types
// based on which feature is enabled

#[cfg(feature = "command")]
mod command_pattern;

#[cfg(feature = "no_pattern")]
mod no_pattern;

#[cfg(feature = "state")]
mod state_pattern;

#[cfg(feature = "type_state")]
mod type_state_pattern;

#[cfg(feature = "command")]
mod solution {
    pub use crate::solutions::command_pattern::*;
}

#[cfg(feature = "state")]
mod solution {
    pub use crate::solutions::state_pattern::*;
}

#[cfg(feature = "type_state")]
mod solution {
    pub use crate::solutions::type_state_pattern::*;
}

#[cfg(feature = "no_pattern")]
mod solution {
    pub use crate::solutions::no_pattern::*;
}

// Re-export all the contents from the appropriate inner solution module
/*
#[cfg(feature = "command")]
pub use self::solution::*;
 */
#[cfg(feature = "state")]
pub use self::solution::*;

#[cfg(feature = "type_state")]
pub use self::solution::*;

#[cfg(feature = "no_pattern")]
pub use self::solution::*;
