use bevy::{prelude::{*}, diagnostic::{DiagnosticsPlugin, LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}, render::{render_resource::PrimitiveTopology, RenderApp}};

pub fn setup(mut commands:Commands, mut meshes:ResMut<Assets<Mesh>>) {
    let a = 10.0;
    commands.spawn_bundle(Camera3dBundle {
        transform:Transform::from_xyz(0.0, a, -a).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh:meshes.add(generate_mesh(1024)),
        ..Default::default()
    });
}

fn generate_mesh(size:usize) -> Mesh {
    let vertices:Vec<[f32;3]> = vec![[0.0, 0.0, 0.0];size*size * 6];
    let normals:Vec<[f32;3]> = vec![[0.0, 0.0, 0.0];size*size  * 6];
    let colors:Vec<[f32;4]> = vec![[0.0, 0.0, 0.0, 0.0];size*size  * 6];
    let uv:Vec<[f32;2]> = vec![[0.0, 0.0];size*size  * 6];
    
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    return mesh;
}

pub fn tick(mut meshes:ResMut<Assets<Mesh>>) {
   /* */ for (_, mesh) in meshes.iter_mut() {
        let pos = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap();
        if let bevy::render::mesh::VertexAttributeValues::Float32x3(_pos) = pos {
            //pos[0][0] += 0.01;
        }
       
    }
}

pub fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(DiagnosticsPlugin::default());
    app.add_plugin(LogDiagnosticsPlugin::default());
    app.add_plugin(FrameTimeDiagnosticsPlugin::default());
    let _render_app = app.get_sub_app_mut(RenderApp).unwrap();

    app.run();
}