use bevy_ecs::system::ResMut;
use bevy_utils::{Duration, Instant};
use bevy_ecs::prelude::Component;
/// Tracks elapsed time since the last update and since the App has started
#[derive(Debug, Clone,Component)]
pub struct Time {
    timestamp: u64,
    delta_seconds_f64: f64,
}
pub trait TimeInterface {
  fn delta_seconds(&self) -> f32;
  fn update_with_timestamp(&mut self,timestamp:u64);
}
impl Default for Time {
    fn default() -> Time {
        Time {
            timestamp: 0,
            delta_seconds_f64: 0.0,
        }
    }
}

impl TimeInterface for Time {

    fn update_with_timestamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }

    /// The delta between the current and last tick as [`f32`] seconds
    #[inline]
    fn delta_seconds(&self) -> f32 {
        self.delta_seconds_f64 as f32
    }
}

