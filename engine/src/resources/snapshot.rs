use bevy::{prelude::{Entity, World, Component}, utils::HashMap};
use serde::{Serialize, Deserialize};

use crate::components::{Player, Thing, Controller};

use super::Tilemap;

#[derive(Clone, Debug, Default, rune::Any, Serialize, Deserialize)]
pub struct Snapshot {
    pub tilemap:Tilemap,
    pub player:Vec<(Entity, Player)>,
    pub things:Vec<(Entity, Thing)>,
    pub controllers:Vec<(Entity, Controller)>
}

impl Snapshot {
    pub fn new(world:&mut World) -> Self {
        Snapshot {
            tilemap:collect_resource::<Tilemap>(world),
            player:collect::<Player>(world),
            things:collect::<Thing>(world),
            controllers:collect::<Controller>(world)
        }
    }
    pub fn restore(&self, world:&mut World) {
        let mut r = Snapshotter::default();
        r.resource(world, &self.tilemap);
        r.components(world, &self.player);
        r.components(world, &self.things);
        r.components(world, &self.controllers);
    }
}


fn collect_resource<T:Default + Clone + Send + Sync + 'static>(world:&mut World) -> T {
    world.get_resource::<T>().unwrap_or(&T::default()).clone()
}

fn collect<T:Component + Clone>(world:&mut World) -> Vec<(Entity, T)> {
    world.query::<(Entity, &T)>().iter(world).map(|(entity, componant)| (entity, componant.clone())).collect()
}

#[derive(Default)]
struct Snapshotter {
    pub map:HashMap<Entity, Entity>
}

impl Snapshotter {
    pub fn resource<T: Send + Sync + Clone + 'static>(&mut self, world:&mut World, res:&T) {
        world.insert_resource(res.clone());
    }
    pub fn components<T:Component + Clone>(&mut self, world:&mut World, components:&Vec<(Entity, T)>) {       
        for (e, c) in components.iter() {
            let e = *self.map.get(e).unwrap_or(&world.spawn().id());
            world.entity_mut(e).insert::<T>(c.clone());
        }
    }
}