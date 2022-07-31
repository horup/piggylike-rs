use std::{path::Path, fs::{create_dir_all, read_to_string}};

use bevy::prelude::*;

use crate::resources::{Snapshot, History};

pub fn snapshot_system(world:&mut World) {
    let snapshot = Snapshot::new(world);
    world.insert_resource(snapshot.clone());

    let quick_save_path = "saves/quicksave.sav";

    if world.get_resource::<Input<KeyCode>>().unwrap().just_pressed(KeyCode::F5) {
        
        if Path::new("saves").exists() == false {
            create_dir_all("saves");
        }

        let json = serde_json::to_string(&snapshot).unwrap();
        
        std::fs::write(&quick_save_path, json).unwrap();
    };

    if world.get_resource::<Input<KeyCode>>().unwrap().just_pressed(KeyCode::F6) {
        if let Ok(s) = read_to_string(&quick_save_path) {
            match serde_json::from_str::<Snapshot>(&s) {
                Ok(snapshot) => {
                    world.clear_entities();
                    snapshot.restore(world);
                },
                Err(err) => println!("{:?}", err),
            }
        }
    };

    let dt = world.get_resource::<Time>().unwrap().delta_seconds();
    if let Some(mut history) = world.get_resource_mut::<History>() {
        if history.timer_sec > history.interval_sec {
            history.snapshots.push_front(snapshot.clone());
            history.timer_sec = 0.0;
            
            if history.snapshots.len() > history.max_snapshots {
                history.snapshots.pop_back();
            }
        }

        history.timer_sec += dt;
    }

  
}