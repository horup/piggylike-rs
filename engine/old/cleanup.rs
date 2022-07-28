use crate::Engine;

impl Engine {
    pub fn update_cleanup(&mut self) {
        let mut delete = Vec::with_capacity(self.world.things.len());
        for (index, thing) in self.world.things.iter() {
            if thing.delete {
                delete.push(index);
            }
        }

        for id in delete {
            self.world.things.remove(id);
        }

        for (_, thing) in self.world.things.iter_mut() {
            thing.touched_thing = None;
        }
    }
}