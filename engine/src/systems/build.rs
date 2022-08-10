use bevy::{prelude::*, utils::tracing::Instrument, render::render_resource::PrimitiveTopology};

use crate::components::MyRaycastSet;

pub fn startup(mut commands:Commands, mut meshes:ResMut<Assets<Mesh>>, mut materials:ResMut<Assets<StandardMaterial>>) {
    let size = 256;
   
    /*let plane_mesh = Mesh::from(shape::Plane {
        size:size
    });*/

    let mut normals = Vec::new();
    let mut vertices = Vec::new();

    for y in 0..size {
        for x in 0..size {
            let margin = 0.05;
            let x = x as f32;
            let y = y as f32;
            vertices.push([x + margin, 0.0, y + margin]);
            normals.push([0.0, 1.0, 0.0]);
        
            vertices.push([x + margin, 0.0, y + 1.0 - margin]);
            normals.push([0.0, 1.0, 0.0]);
        
            vertices.push([x + 1.0 - margin, 0.0, y + 1.0 - margin]);
            normals.push([0.0, 1.0, 0.0]);
        
            vertices.push([x + margin, 0.0, y + margin]);
            normals.push([0.0, 1.0, 0.0]);
        
            vertices.push([x + 1.0 - margin, 0.0, y + 1.0 - margin]);
            normals.push([0.0, 1.0, 0.0]);
        
            vertices.push([x + 1.0 - margin, 0.0, y + margin]);
            normals.push([0.0, 1.0, 0.0]);
        }
    }

   

    let mut plane_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    plane_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    plane_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    //plane_mesh.set_indices(indices)

    let plane_mesh = meshes.add(plane_mesh);
    let material:StandardMaterial = Color::rgb(1.0, 1.0, 1.0).into();
    let material = materials.add(material);
    commands.spawn_bundle(PbrBundle {
        mesh:plane_mesh,
        material:material,
        //transform:Transform::from_xyz(size / 2.0, 0.0, size / 2.0),
        ..Default::default()
    });

    let size = size as f32;
    
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, size, size).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}