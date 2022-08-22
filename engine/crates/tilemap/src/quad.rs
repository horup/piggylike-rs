use bevy::prelude::{Vec3, Vec2, Vec4};

pub struct Quad {
    pub vertices:[Vec3; 6],
    pub normals:[Vec3; 6],
    pub colors:[Vec4; 6],
    pub uvs:[Vec2; 6]
}

impl Quad {
    #[inline(always)]
    pub fn new_front() -> Self {
        let v = [[-0.5, 0.5, 0.0],     [-0.5, -0.5, 0.0],    [0.5, -0.5, 0.0],     [-0.5, 0.5, 0.0],    [0.5, -0.5, 0.0],      [0.5, 0.5, 0.0]];
        let n = [[0.0, 0.0, 1.0],      [0.0, 0.0, 1.0],      [0.0, 0.0, 1.0],      [0.0, 0.0, 1.0],      [0.0, 0.0, 1.0],      [0.0, 0.0, 1.0]];
        let c = [[1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0]];
        let u = [[0.0, 0.0],           [0.0, 1.0],           [1.0, 1.0],           [0.0, 0.0],           [1.0, 1.0],           [1.0, 0.0]];

        Self {
            vertices: v.map(|v|Vec3::from(v)), 
            normals: n.map(|v|Vec3::from(v)), 
            colors: c.map(|v|Vec4::from(v)), 
            uvs: u.map(|v|Vec2::from(v))
        }
    }

    #[inline(always)]
    pub fn new_back() -> Self {
        let v = [[0.5, 0.5, 0.0],     [0.5, -0.5, 0.0],    [-0.5, -0.5, 0.0],     [0.5, 0.5, 0.0],    [-0.5, -0.5, 0.0],      [-0.5, 0.5, 0.0]];
        let n = [[0.0, 0.0, -1.0],      [0.0, 0.0, -1.0],      [0.0, 0.0, -1.0],      [0.0, 0.0, -1.0],      [0.0, 0.0, -1.0],      [0.0, 0.0, -1.0]];
        let c = [[1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0]];
        let u = [[0.0, 0.0],           [0.0, 1.0],           [1.0, 1.0],           [0.0, 0.0],           [1.0, 1.0],           [1.0, 0.0]];

        Self {
            vertices: v.map(|v|Vec3::from(v)), 
            normals: n.map(|v|Vec3::from(v)), 
            colors: c.map(|v|Vec4::from(v)), 
            uvs: u.map(|v|Vec2::from(v))
        }
    }

    #[inline(always)]
    pub fn new_left() -> Self {
        let v = [[0.0, 0.5, -0.5],     [0.0, -0.5, -0.5],    [0.0, -0.5, 0.5],     [0.0, 0.5, -0.5],    [0.0, -0.5, 0.5],      [0.0, 0.5, 0.5]];
        let n = [[-1.0, 0.0, 0.0],      [-1.0, 0.0, 0.0],      [-1.0, 0.0, 0.0],      [-1.0, 0.0, 0.0],      [-1.0, 0.0, 0.0],      [-1.0, 0.0, 0.0]];
        let c = [[1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0]];
        let u = [[0.0, 0.0],           [0.0, 1.0],           [1.0, 1.0],           [0.0, 0.0],           [1.0, 1.0],           [1.0, 0.0]];

        Self {
            vertices: v.map(|v|Vec3::from(v)), 
            normals: n.map(|v|Vec3::from(v)), 
            colors: c.map(|v|Vec4::from(v)), 
            uvs: u.map(|v|Vec2::from(v))
        }
    }

    #[inline(always)]
    pub fn new_right() -> Self {
        let v = [[0.0, 0.5, 0.5],     [0.0, -0.5, 0.5],    [0.0, -0.5, -0.5],     [0.0, 0.5, 0.5],    [0.0, -0.5, -0.5],      [0.0, 0.5, -0.5]];
        let n = [[1.0, 0.0, 0.0],      [1.0, 0.0, 0.0],      [1.0, 0.0, 0.0],      [1.0, 0.0, 0.0],      [1.0, 0.0, 0.0],      [1.0, 0.0, 0.0]];
        let c = [[1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0]];
        let u = [[0.0, 0.0],           [0.0, 1.0],           [1.0, 1.0],           [0.0, 0.0],           [1.0, 1.0],           [1.0, 0.0]];

        Self {
            vertices: v.map(|v|Vec3::from(v)), 
            normals: n.map(|v|Vec3::from(v)), 
            colors: c.map(|v|Vec4::from(v)), 
            uvs: u.map(|v|Vec2::from(v))
        }
    }

    #[inline(always)]
    pub fn recompute_uvs(&mut self) {
        let uvs = &mut self.uvs;
        let top = self.vertices[0].y;
        let bottom = self.vertices[1].y;

        for i in [0, 3, 5] {
            uvs[i].y = -top.abs();
        }

        for i in [1, 2, 4] {
            uvs[i].y = bottom.abs();
        }
    }

    #[inline(always)]
    pub fn copy_to(self, vertices:&mut Vec<[f32;3]>, normals:&mut Vec<[f32;3]>, colors:&mut Vec<[f32;4]>, uvs:&mut Vec<[f32;2]>) {
        self.vertices.into_iter().for_each(|v|vertices.push(v.into()));
        self.normals.into_iter().for_each(|v|normals.push(v.into()));
        self.colors.into_iter().for_each(|v|colors.push(v.into()));
        self.uvs.into_iter().for_each(|v|uvs.push(v.into()));
    }

    #[inline(always)]
    pub fn translate(&mut self, translate:Vec3) {
        for v in self.vertices.iter_mut() {
            *v += translate;
        }
    }

    #[inline(always)]
    pub fn set_top(&mut self, y:f32) {
        self.vertices[0].y = y;
        self.vertices[3].y = y;
        self.vertices[5].y = y;
    }

    #[inline(always)]
    pub fn set_bottom(&mut self, y:f32) {
        self.vertices[1].y = y;
        self.vertices[2].y = y;
        self.vertices[4].y = y;
    }
}