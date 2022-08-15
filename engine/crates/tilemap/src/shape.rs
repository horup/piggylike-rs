use bevy::{prelude::*, render::render_resource::PrimitiveTopology};

pub struct Grid {
    pub width:usize,
    pub height:usize
}

impl From<Grid> for Mesh {
    fn from(grid: Grid) -> Self {
        let mut normals = Vec::new();
        let mut vertices = Vec::new();
        let mut colors = Vec::new();

        let margin = 0.00;
        let width = grid.width as f32;
        let height = grid.height as f32;

        let x = 0.0;
        let z = 0.0;
        let c = [0.0, 0.0, 0.0, 1.0];
        let y = -0.01;
        vertices.push([x + margin, y, z + margin]);
        normals.push([0.0, 1.0, 0.0]);
        colors.push(c);
    
        vertices.push([x + margin, y, z + height - margin]);
        normals.push([0.0, 1.0, 0.0]);
        colors.push(c);
    
        vertices.push([x + width - margin, y, z + height - margin]);
        normals.push([0.0, 1.0, 0.0]);
        colors.push(c);
    
        vertices.push([x + margin, y, z + margin]);
        normals.push([0.0, 1.0, 0.0]);
        colors.push(c);
    
        vertices.push([x + width - margin, y, z + height - margin]);
        normals.push([0.0, 1.0, 0.0]);
        colors.push(c);
    
        vertices.push([x + width - margin, y, z + margin]);
        normals.push([0.0, 1.0, 0.0]);
        colors.push(c);
        
        for y in 0..grid.height {
            for x in 0..grid.width {
                let margin = 0.05;
                let x = x as f32;
                let z = y as f32;
                let y = 0.00;
                let c = [1.0, 1.0, 1.0, 1.0];
                vertices.push([x + margin, y, z + margin]);
                normals.push([0.0, 1.0, 0.0]);
                colors.push(c);
            
                vertices.push([x + margin, y, z + 1.0 - margin]);
                normals.push([0.0, 1.0, 0.0]);
                colors.push(c);
            
                vertices.push([x + 1.0 - margin, y, z + 1.0 - margin]);
                normals.push([0.0, 1.0, 0.0]);
                colors.push(c);
            
                vertices.push([x + margin, y, z + margin]);
                normals.push([0.0, 1.0, 0.0]);
                colors.push(c);
            
                vertices.push([x + 1.0 - margin, y, z + 1.0 - margin]);
                normals.push([0.0, 1.0, 0.0]);
                colors.push(c);
            
                vertices.push([x + 1.0 - margin, y, z + margin]);
                normals.push([0.0, 1.0, 0.0]);
                colors.push(c);
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        return mesh;
    }
}