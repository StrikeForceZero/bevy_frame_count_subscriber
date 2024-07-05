use std::fmt::Formatter;
use std::sync::RwLock;

use bevy::ecs::system::RunSystemOnce;
use bevy::log::LogPlugin;
use bevy::prelude::*;

use bevy_frame_count_subscriber::config::FrameCountSubscriberConfig;
use bevy_frame_count_subscriber::formatter::{FormatFrameCount, FrameCounterPrefixFormatter};
use bevy_frame_count_subscriber::plugin::FrameCountSubscriberPlugin;

#[test]
fn main() {
    static WAS_WRITE_CALLED: RwLock<bool> = RwLock::new(false);

    struct TestFormatter;

    impl FormatFrameCount for TestFormatter {
        fn debug_name(&self) -> &'static str {
            "TestFormatter"
        }
        fn write(&self, f: &mut Formatter<'_>, frame_count: u32) -> std::fmt::Result {
            *WAS_WRITE_CALLED.write().unwrap() = true;
            write!(f, "{frame_count} ")
        }
    }

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .insert_resource(FrameCountSubscriberConfig {
            formatter: FrameCounterPrefixFormatter::new(TestFormatter),
        })
        .add_plugins(FrameCountSubscriberPlugin);
    // using error! to be captured by env filter
    app.world_mut().run_system_once(|| error!("test"));
    assert!(*WAS_WRITE_CALLED.read().unwrap(), "write was never called");
}
