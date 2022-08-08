use bevy::prelude::*;
use bevy_flycam::FlyCam;
use bevy_mod_raycast::{RayCastSource, DefaultPluginState, RayCastMesh};

use crate::components::MyRaycastSet;

pub fn startup(mut commands:Commands, mut meshes:ResMut<Assets<Mesh>>, mut materials:ResMut<Assets<StandardMaterial>>) {
    let size = 32.0;
   
    let plane_mesh = Mesh::from(shape::Plane {
        size:size
    });
    let plane_mesh = meshes.add(plane_mesh);
    let material:StandardMaterial = Color::rgb(1.0, 1.0, 1.0).into();
    let material = materials.add(material);
    commands.spawn_bundle(PbrBundle {
        mesh:plane_mesh,
        material:material,
        transform:Transform::from_xyz(size / 2.0, 0.0, size / 2.0),
        ..Default::default()
    }).insert(RayCastMesh::<MyRaycastSet>::default());


    commands.insert_resource(DefaultPluginState::<MyRaycastSet>::default().with_debug_cursor());
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, size, size).looking_at(Vec3::new(10.0, 0.0, 10.0), Vec3::Y),
        ..Default::default()
    }).insert(RayCastSource::<MyRaycastSet>::new_transform_empty())
    .insert(FlyCam);
}