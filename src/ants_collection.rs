use macroquad::prelude::*;
use crate::ants::{WorkerAnt, TreePoint, Colony, Food};
use rstar::RTree;
use std::collections::HashSet;

/// A collection of worker ants. Responsible for handling interactions between biots
pub struct WorkerAntCollection {
    worker_ants: Vec<WorkerAnt>
}

impl WorkerAntCollection {
    /// Create n worker ants
    pub fn new(n: usize, colony:ColonyImplimintation) -> Self {
        let mut s = Self { worker_ants: Vec::new() };
        for _ in 0..n {
            s.worker_ants.push(WorkerAnt::worker_ant(vec2(colony.colony.pos.x, colony.colony.pos.y)));
        }
        s
    }
    /// Compute one step of the simulation.
    pub fn step(&mut self) {
        let mut new : Vec<WorkerAnt> = Vec::new();
        // let mut dead: Vec<Food> = Vec::new();
        // R-star datastructure used for quickly locating neighbors
        let tree : RTree<TreePoint> = RTree::bulk_load(
            self.worker_ants
                .iter()
                .enumerate()
                .map(|(n,ant)|TreePoint {x:ant.pos.x as f64, y:ant.pos.y as f64, idx:n})
                .collect());
        // Move and reproduce worker_ants
        for n in 0..(self.worker_ants.len()) {
            // let mut feed_dir : Option<Vec2> = None;

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
            let off = self.worker_ants[n].step(&tree, None);
            if let Some(offspring) = off {
                new.push(offspring);
            }

// work in progress, intend to convert dead ants to food

            // self.worker_ants[n].dead();
            // if self.worker_ants[n].dead == true {
            //     dead.push(self.worker_ants[n].turn_dead());

            // }
        }
        // Compute biot interactions
        // let mut visited : HashSet<usize> = HashSet::new();
        // for f in tree.iter() {
        //     visited.insert(f.idx);

            // makes the ants check for eachother, change so they check for pheromones instead, and act instead of fight
        //     for s in tree.locate_within_distance([f.x, f.y], 50.) //FIXME 30 is hardcoded
        //     {
        //         if ! visited.contains(&s.idx) { // Don't do it twice
                // WorkerAnt::interact(&mut self.worker_ants, f.idx, s.idx);
        //         }
        //     }
        // }
        // Remove dead worker_ants and add the new ones to the collection

        self.worker_ants.retain(|b| !b.dead());
        self.worker_ants.append(&mut new);
    }
    /// Display the biot collection
    pub fn draw_ant(&self) {
        for worker_ant in self.worker_ants.iter() {
            draw_circle(worker_ant.pos.x,worker_ant.pos.y, 3., RED);
        }
    }
    /// The number of biots currently in our collection
    pub fn len(&self) -> usize {
        self.worker_ants.len()
    }
}


pub struct ColonyImplimintation {
    colony: Colony
}

impl ColonyImplimintation {
    pub fn new(posx:f32, posy:f32) -> ColonyImplimintation {
        let q = Self { colony: Colony::colony(posx, posy) };
        q
    }

    pub fn draw_colony(&self) {
        let posx = &self.colony.pos.x;
        let posy = &self.colony.pos.y;
        draw_circle(*posx, *posy, 10., BROWN);
    }

    pub fn step (&self) {

    }
    
}

pub struct FoodCollection {
   food: Vec<Food>
}

impl FoodCollection {
    pub fn new(n: usize) -> Self {
        let mut f = Self { food: Vec::new() };
        for _ in 0..n {
            f.food.push(Food::food(vec2(
                rand::gen_range(0., 1.)*screen_width(),
                rand::gen_range(0., 1.)*screen_height()
            )));
        }
        f
    }
    
    pub fn step(&mut self) {
        let mut new : Vec<Food> = Vec::new();
        // R-star datastructure used for quickly locating neighbors
        let tree : RTree<TreePoint> = RTree::bulk_load(
            self.food
            .iter()
            .enumerate()
            .map(|(n,piece)|TreePoint {x:piece.pos.x as f64, y:piece.pos.y as f64, idx:n})
            .collect());
        
        
        let mut visited : HashSet<usize> = HashSet::new();
        for f in tree.iter() {
            visited.insert(f.idx);
            
            for s in tree.locate_within_distance([f.x, f.y], 50.) //FIXME 30 is hardcoded
            {
                if ! visited.contains(&s.idx) { // Don't do it twice
                    Food::aggergate( &mut self.food, f.idx, s.idx);
                }
            }
        }

        FoodCollection::new(1);
    
        self.food.append(&mut new);
        self.food.retain(|b| b.exist);
        
        let mut new : Vec<Food> = Vec::new();
        // R-star datastructure used for quickly locating neighbors
        let tree : RTree<TreePoint> = RTree::bulk_load(
            self.food
            .iter()
            .enumerate()
            .map(|(n,piece)|TreePoint {x:piece.pos.x as f64, y:piece.pos.y as f64, idx:n})
            .collect());
        
        
        let mut visited : HashSet<usize> = HashSet::new();
        for f in tree.iter() {
            visited.insert(f.idx);
            
            for s in tree.locate_within_distance([f.x, f.y], 50.) //FIXME 30 is hardcoded
            {
                if ! visited.contains(&s.idx) { // Don't do it twice
                    Food::aggergate( &mut self.food, f.idx, s.idx);
                }
            }
        }

        FoodCollection::new(1);
    
        self.food.append(&mut new);
        self.food.retain(|b| b.exist);
    }

pub fn draw_piece(&self) {
    for food in self.food.iter() {
        draw_circle(food.pos.x,food.pos.y, food.size, ORANGE);
    }
    }
    pub fn len(&self) -> usize {
        self.food.len()
    }



}


