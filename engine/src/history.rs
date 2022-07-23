use crate::Engine;



impl Engine {
    pub fn save_world(&self, path:&str) {

        
    }

    pub fn load_world(&mut self, path:&str) {

    }

    pub fn push_timeline(&mut self) {
        self.timeline.push(self.world.clone());
    }

    pub fn pop_timeline(&mut self) {
        if let Some(world) = self.timeline.pop() {
            self.world = world;
        }
    }

    pub fn update_history(&mut self) {
        if self.world.iterations % 10 == 0 {
            self.push_timeline();
        }
    }

}