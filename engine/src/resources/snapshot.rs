use bevy::{prelude::{Entity, World, Component}, utils::HashMap};
use serde::{Serialize, Deserialize};

use crate::components::{Player, Thing, Controller, Cam, Body};

use super::Tilemap;

type Components<T:Default + Component + Clone> = Vec<(Entity, T)>;
#[derive(Clone, Debug, Default, rune::Any, Serialize, Deserialize)]
pub struct Snapshot {
    pub tilemap:Tilemap,
    pub players:Vec<(Entity, Player)>,
    pub things:Vec<(Entity, Thing)>,
    pub controllers:Vec<(Entity, Controller)>,
    pub cams:Components<Cam>,
    pub bodies:Components<Body>
}

impl Snapshot {
    pub fn new(world:&mut World) -> Self {
        Snapshot {
            tilemap:collect_resource::<Tilemap>(world),
            players:collect::<Player>(world),
            things:collect::<Thing>(world),
            controllers:collect::<Controller>(world),
            cams:collect::<Cam>(world),
            bodies:collect::<Body>(world)
        }
    }
    pub fn restore(&self, world:&mut World) {
        let mut r = Snapshotter::default();
        r.resource(world, &self.tilemap);
        r.components(world, &self.players);
        r.components(world, &self.things);
        r.components(world, &self.controllers);
        r.components(world, &self.cams);

        println!("{:?}", self.bodies);
        r.components(world, &self.bodies);
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
        for (org_e, c) in components.iter() {
            let e = *self.map.get(org_e).unwrap_or(&world.spawn().id());
            world.entity_mut(e).insert::<T>(c.clone());

            self.map.insert(*org_e, e);
        }
    }
}