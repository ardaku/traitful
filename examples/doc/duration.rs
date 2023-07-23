use std::time::Duration;

use traitful::extend;

/// Extension trait for [`Duration`]
#[extend(Duration)]
pub trait DurationExt {
    /// Returns the total number of whole centiseconds contained by this
    /// [`Duration`].
    fn as_centis(&self) -> u128 {
        self.as_millis() / 10
    }
}
