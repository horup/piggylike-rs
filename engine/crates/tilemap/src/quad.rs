use bevy::prelude::{Vec3, Vec2, Vec4};

pub struct Quad {
    pub vertices:[[f32;3]; 6],
    pub normals:[[f32;3]; 6],
    pub colors:[[f32;4]; 6],
    pub uvs:[[f32;2]; 6]
}

impl Quad {
    #[inline(always)]
    pub const fn new_front() -> Self {
        let v = [[-0.5, 0.5, 0.0],     [-0.5, -0.5, 0.0],    [0.5, -0.5, 0.0],     [-0.5, 0.5, 0.0],    [0.5, -0.5, 0.0],      [0.5, 0.5, 0.0]];
        let n = [[0.0, 0.0, 1.0],      [0.0, 0.0, 1.0],      [0.0, 0.0, 1.0],      [0.0, 0.0, 1.0],      [0.0, 0.0, 1.0],      [0.0, 0.0, 1.0]];
        let c = [[1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0]];
        let u = [[0.0, 0.0],           [0.0, 1.0],           [1.0, 1.0],           [0.0, 0.0],           [1.0, 1.0],           [1.0, 0.0]];

        Self {
            vertices: v, 
            normals: n, 
            colors: c, 
            uvs: u
        }
    }

    #[inline(always)]
    pub const fn new_back() -> Self {
        let v = [[0.5, 0.5, 0.0],     [0.5, -0.5, 0.0],    [-0.5, -0.5, 0.0],     [0.5, 0.5, 0.0],    [-0.5, -0.5, 0.0],      [-0.5, 0.5, 0.0]];
        let n = [[0.0, 0.0, -1.0],      [0.0, 0.0, -1.0],      [0.0, 0.0, -1.0],      [0.0, 0.0, -1.0],      [0.0, 0.0, -1.0],      [0.0, 0.0, -1.0]];
        let c = [[1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0]];
        let u = [[0.0, 0.0],           [0.0, 1.0],           [1.0, 1.0],           [0.0, 0.0],           [1.0, 1.0],           [1.0, 0.0]];

        Self {
            vertices: v, 
            normals: n, 
            colors: c, 
            uvs: u
        }
    }

    #[inline(always)]
    pub const fn new_left() -> Self {
        let v = [[0.0, 0.5, -0.5],     [0.0, -0.5, -0.5],    [0.0, -0.5, 0.5],     [0.0, 0.5, -0.5],    [0.0, -0.5, 0.5],      [0.0, 0.5, 0.5]];
        let n = [[-1.0, 0.0, 0.0],      [-1.0, 0.0, 0.0],      [-1.0, 0.0, 0.0],      [-1.0, 0.0, 0.0],      [-1.0, 0.0, 0.0],      [-1.0, 0.0, 0.0]];
        let c = [[1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0]];
        let u = [[0.0, 0.0],           [0.0, 1.0],           [1.0, 1.0],           [0.0, 0.0],           [1.0, 1.0],           [1.0, 0.0]];

        Self {
            vertices: v, 
            normals: n, 
            colors: c, 
            uvs: u
        }
    }

    #[inline(always)]
    pub const fn new_right() -> Self {
        let v = [[0.0, 0.5, 0.5],     [0.0, -0.5, 0.5],    [0.0, -0.5, -0.5],     [0.0, 0.5, 0.5],    [0.0, -0.5, -0.5],      [0.0, 0.5, -0.5]];
        let n = [[1.0, 0.0, 0.0],      [1.0, 0.0, 0.0],      [1.0, 0.0, 0.0],      [1.0, 0.0, 0.0],      [1.0, 0.0, 0.0],      [1.0, 0.0, 0.0]];
        let c = [[1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0]];
        let u = [[0.0, 0.0],           [0.0, 1.0],           [1.0, 1.0],           [0.0, 0.0],           [1.0, 1.0],           [1.0, 0.0]];

        Self {
            vertices: v, 
            normals: n, 
            colors: c, 
            uvs: u
        }
    }

    #[inline(always)]
    pub fn recompute_uvs(&mut self) {
        let uvs = &mut self.uvs;
        let top = self.vertices[0][1];
        let bottom = self.vertices[1][1];

        for i in [0, 3, 5] {
            uvs[i][1] = -top.abs();
        }

        for i in [1, 2, 4] {
            uvs[i][1] = bottom.abs();
        }
    }

    #[inline(always)]
    pub fn copy_to(self, vertices:&mut Vec<[f32;3]>, normals:&mut Vec<[f32;3]>, colors:&mut Vec<[f32;4]>, uvs:&mut Vec<[f32;2]>) {
        self.vertices.into_iter().for_each(|v|vertices.push(v));
        self.normals.into_iter().for_each(|v|normals.push(v));
        self.colors.into_iter().for_each(|v|colors.push(v));
        self.uvs.into_iter().for_each(|v|uvs.push(v));
    }

    #[inline(always)]
    pub fn translate(&mut self, translate:[f32;3]) {
        for v in self.vertices.iter_mut() {
            v[0] += translate[0];
            v[1] += translate[1];
            v[2] += translate[2];
        }
    }

    #[inline(always)]
    pub fn set_top(&mut self, y:f32) {
        self.vertices[0][1] = y;
        self.vertices[3][1] = y;
        self.vertices[5][1] = y;
    }

    #[inline(always)]
    pub fn set_bottom(&mut self, y:f32) {
        self.vertices[1][1] = y;
        self.vertices[2][1] = y;
        self.vertices[4][1] = y;
    }
}