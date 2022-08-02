use bevy::prelude::*;

use crate::events::TouchEvent;

pub fn touch_system(mut touch_events:EventReader<TouchEvent>) {
    for e in touch_events.iter() {
        info!("{:?}", e);
    }
}