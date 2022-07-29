use bevy::math::Vec3;

#[derive(Debug, Clone, Copy, Default)]
pub struct Config {
    pub pixel_snap_resolution:f32
}

impl Config {
    pub fn snap_vec3(&self, mut v:Vec3) -> Vec3 {
        if self.pixel_snap_resolution <= 0.0 {
            return v;
        }
        
        for i in 0..3 {
            v[i] = (v[i] / self.pixel_snap_resolution).floor() * self.pixel_snap_resolution;
        }

        return v;
    }
}
