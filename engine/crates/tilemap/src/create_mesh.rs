use bevy::{prelude::{Mesh}, render::render_resource::PrimitiveTopology};
use metadata::Id;
use ndarray::{ArrayView2};

use crate::{Tile, Quad};

#[inline(always)]
pub fn create_mesh(tiles: &ArrayView2<Tile>, material: Id, min_bottom:f32, max_top:f32) -> Mesh {
    let size = tiles.dim().0 *  tiles.dim().1 * 6 * 6 * 4;
    let mut normals = Vec::with_capacity(size);
    let mut vertices = Vec::with_capacity(size);
    let mut colors = Vec::with_capacity(size);
    let mut uvs = Vec::with_capacity(size);

    // floor
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

    // front wall
    for ((x, y), tile) in tiles.indexed_iter() {
        let x = x as f32;
        let z = y as f32;
        let _y = tile.top;

        if tile.walls == material {
            let mut wall = Quad::new_front();
            wall.set_bottom(min_bottom);
            wall.set_top(tile.bottom);
            wall.translate([0.5, 0.0, 1.0]);
            wall.translate([x, 0.0, z]);
            wall.recompute_uvs();
            wall.copy_to(&mut vertices, &mut normals, &mut colors, &mut uvs);

            let mut wall = Quad::new_front();
            wall.set_bottom(tile.top);
            wall.set_top(max_top);
            wall.translate([0.5, 0.0, 1.0]);
            wall.translate([x, 0.0, z]);
            wall.recompute_uvs();
            wall.copy_to(&mut vertices, &mut normals, &mut colors, &mut uvs);
        }
    }

    // back wall
    for ((x, y), tile) in tiles.indexed_iter() {
        let x = x as f32;
        let z = y as f32;
        let _y = tile.top;

        if tile.walls == material {
            let mut wall = Quad::new_back();
            wall.set_bottom(min_bottom);
            wall.set_top(tile.bottom);
            wall.translate([0.5, 0.0, 0.0]);
            wall.translate([x, 0.0, z]);
            wall.recompute_uvs();
            wall.copy_to(&mut vertices, &mut normals, &mut colors, &mut uvs);

            let mut wall = Quad::new_back();
            wall.set_bottom(tile.top);
            wall.set_top(max_top);
            wall.translate([0.5, 0.0, 0.0]);
            wall.translate([x, 0.0, z]);
            wall.recompute_uvs();
            wall.copy_to(&mut vertices, &mut normals, &mut colors, &mut uvs);
        }
    }

    // left wall
    for ((x, y), tile) in tiles.indexed_iter() {
        let x = x as f32;
        let z = y as f32;
        let _y = tile.top;

        if tile.walls == material {
            let mut wall = Quad::new_left();
            wall.set_bottom(min_bottom);
            wall.set_top(tile.bottom);
            wall.translate([0.0, 0.0, 0.5]);
            wall.translate([x, 0.0, z]);
            wall.recompute_uvs();
            wall.copy_to(&mut vertices, &mut normals, &mut colors, &mut uvs);

            let mut wall = Quad::new_left();
            wall.set_bottom(tile.top);
            wall.set_top(max_top);
            wall.translate([0.0, 0.0, 0.5]);
            wall.translate([x, 0.0, z]);
            wall.recompute_uvs();
            wall.copy_to(&mut vertices, &mut normals, &mut colors, &mut uvs);
        }
    }

    // right wall
    for ((x, y), tile) in tiles.indexed_iter() {
        let x = x as f32;
        let z = y as f32;
        let _y = tile.top;

        if tile.walls == material {
            let mut wall = Quad::new_right();
            wall.set_bottom(min_bottom);
            wall.set_top(tile.bottom);
            wall.translate([1.0, 0.0, 0.5]);
            wall.translate([x, 0.0, z]);
            wall.recompute_uvs();
            wall.copy_to(&mut vertices, &mut normals, &mut colors, &mut uvs);

            let mut wall = Quad::new_right();
            wall.set_bottom(tile.top);
            wall.set_top(max_top);
            wall.translate([1.0, 0.0, 0.5]);
            wall.translate([x, 0.0, z]);
            wall.recompute_uvs();
            wall.copy_to(&mut vertices, &mut normals, &mut colors, &mut uvs);
        }
    }

   

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    return mesh;
}
