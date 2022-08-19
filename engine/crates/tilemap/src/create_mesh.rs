use bevy::{prelude::Mesh, render::render_resource::PrimitiveTopology};
use metadata::Id;
use ndarray::Array2;

use crate::Tile;

pub fn create_mesh(tiles:&Array2<Tile>, material:Id) -> Mesh {
    let mut normals = Vec::new();
    let mut vertices = Vec::new();
    let mut colors = Vec::new();
    let mut uvs = Vec::new();
    let mut bottom = 0.0;
    let mut top = 0.0;
    tiles.for_each(|t|{
        if t.bottom < bottom {
            bottom = t.bottom;
        }
        if t.top > top {
            top = t.top;
        }
    });

    // floors
    for ((x, y), tile) in tiles.indexed_iter() {
        let x = x as f32;
        let z = y as f32;
        let y = tile.bottom;

        let w = 1.0;
        let h = 1.0;
        let c = [1.0, 1.0, 1.0, 1.0];
        if tile.floor == material {
            vertices.push([x, y, z]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push(c);
            uvs.push([0.0, 0.0]);
        
            vertices.push([x, y, z + w]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push(c);
            uvs.push([0.0, 1.0]);
        
            vertices.push([x + h, y, z + w]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push(c);
            uvs.push([1.0, 1.0]);
        
            vertices.push([x, y, z]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push(c);
            uvs.push([0.0, 0.0]);
        
            vertices.push([x + h, y, z + w]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push(c);
            uvs.push([1.0, 1.0]);
        
            vertices.push([x + h, y, z]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push(c);
            uvs.push([1.0, 0.0]);
        }
    }

    // cealing
    for ((x, y), tile) in tiles.indexed_iter() {
        let x = x as f32;
        let z = y as f32;
        let y = tile.top;

        let w = 1.0;
        let h = 1.0;
        let c = [1.0, 1.0, 1.0, 1.0];
        if tile.cealing == material {
           
        
           
        
          
        
           
        
           
        
            vertices.push([x + h, y, z]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push(c);
            uvs.push([1.0, 0.0]);



 vertices.push([x + h, y, z + w]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push(c);
            uvs.push([1.0, 1.0]);






 vertices.push([x, y, z]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push(c);
            uvs.push([0.0, 0.0]);





  vertices.push([x + h, y, z + w]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push(c);
            uvs.push([1.0, 1.0]);









 vertices.push([x, y, z + w]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push(c);
            uvs.push([0.0, 1.0]);

 vertices.push([x, y, z]);
            normals.push([0.0, 1.0, 0.0]);
            colors.push(c);
            uvs.push([0.0, 0.0]);


        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    return mesh;
}