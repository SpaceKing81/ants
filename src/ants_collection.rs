use macroquad::prelude::*;
use crate::ants::{WorkerAnt, TreePoint, Colony};
use rstar::RTree;
use std::collections::HashSet;

/// A collection of worker ants. Responsible for handling interactions between biots
pub struct WorkerAntCollection {
    worker_ants: Vec<WorkerAnt>
}

impl WorkerAntCollection {
    /// Create n worker ants
    pub fn new(n: usize) -> Self {
        let mut s = Self { worker_ants: Vec::new() };
        for _ in 0..n {
            s.worker_ants.push(WorkerAnt::worker_ant(/*i dont work pls fix me and put the pos of the colony so the ants spawn in me */));
        }
        s
    }
    /// Compute one step of the simulation.
    pub fn step(&mut self) {
        let mut new : Vec<WorkerAnt> = Vec::new();
        // R-star datastructure used for quickly locating neighbors
        let tree : RTree<TreePoint> = RTree::bulk_load(
            self.worker_ants
                .iter()
                .enumerate()
                .map(|(n,biot)|TreePoint {x:biot.pos.x as f64, y:biot.pos.y as f64, idx:n})
                .collect());
        // Move and reproduce worker_ants
        for n in 0..(self.worker_ants.len()) {
            let mut feed_dir : Option<Vec2> = None;

            // if self.worker_ants[n].intelligence > 0. {
            //     for (other, d2) in tree.nearest_neighbor_iter_with_distance_2(&[self.worker_ants[n].pos.x as f64, self.worker_ants[n].pos.y as f64]) {
            //         if other.idx == n {
            //             // Filter the biot itself out of the query.
            //             continue;
            //         }
            //         if d2 as f32 > (self.worker_ants[n].intelligence*self.worker_ants[n].intelligence)*1600. {
            //             break;
            //         }
            //         if self.worker_ants[n].stronger(&self.worker_ants[other.idx]) {
            //             feed_dir = Some(vec2(other.x as f32 -self.worker_ants[n].pos.x, other.y as f32 -self.worker_ants[n].pos.y).normalize());
            //             break;
            //         }
            //     }
            // }
            let off = self.worker_ants[n].step(&tree, feed_dir);
            if let Some(offspring) = off {
                new.push(offspring);
            }
        }
        // Compute biot interactions
        // let mut visited : HashSet<usize> = HashSet::new();
        // for f in tree.iter() {
        //     visited.insert(f.idx);

            // makes the ants check for eachother, change so they check for pheromones instead, and act instead of fight
        //     for s in tree.locate_within_distance([f.x, f.y], 50.) //FIXME 30 is hardcoded
        //     {
        //         if ! visited.contains(&s.idx) { // Don't do it twice
        //         WorkerAnt::interact(&mut self.worker_ants, f.idx, s.idx);
        //         }
        //     }
        // }
        // Remove dead worker_ants and add the new ones to the collection
        self.worker_ants.retain(|b| !b.dead());
        // self.worker_ants.append(&mut new);
    }
    /// Display the biot collection
    pub fn draw(&self) {
        for worker_ant in self.worker_ants.iter() {
            
            draw_circle(worker_ant.pos.x,worker_ant.pos.y, 3., RED);

        }
    }
    /// The number of biots currently in our collection
    pub fn len(&self) -> usize {
        self.worker_ants.len()
    }
}
