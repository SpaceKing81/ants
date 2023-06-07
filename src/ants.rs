use macroquad::{prelude::{Vec2, vec2, rand, screen_width, screen_height}, miniquad::gl::PFNGLCOMPRESSEDTEXIMAGE1DPROC};
use rstar::{AABB, PointDistance, RTree, RTreeObject};

// / Elements of the biots' genomes:
// / a for attack
// / d for defence
// / p for photosynthesis
// / m for motion
// / i for intelligence
// n does nothing
// const LETTERS : &[char] = &['a','d','p','m', 'n', 'n', 'n', 'i'];

// Modulus operator to get toroidal world topology
fn modulus<T>(a:T, b:T) -> T
where T: std::ops::Rem<Output=T>+
      std::ops::Add<Output = T>+
      Copy
{
    ((a % b) + b) % b
}

// A single biot
#[derive(Clone, Debug)]
pub struct WorkerAnt {
    // Status
    life: f32,
    pub pos: Vec2,
    speed: Vec2,
    age: u32,
    pub dead: bool

    // Genome
    // genome: [char; 13],
    // pub attack: f32,
    // pub defense: f32,
    // pub photosynthesis: f32,
    // pub motion: f32,
    // pub intelligence: f32,
}

impl WorkerAnt {
    /// Create a worker ant
    pub fn worker_ant(pos:Vec2<>) -> Self {
        // let mut genome = ['u';13];
        // for letter in genome.iter_mut() {
        //     *letter = LETTERS[rand::gen_range(0, LETTERS.len() as u32) as usize];
        // }
        let mut s = Self {
            life: 0.,
            pos,
            speed: vec2( 2., 2.),
            age: 0,
            dead: false,
            // genome,
            // attack: 0.,
            // defense: 0.,
            // photosynthesis: 0.,
            // motion: 0.,
            // intelligence: 0.,
        };
        // s.set_from_genome();
        s.life = s.base_life();
        s
    }
    // Compute the evolution of the biot for one simulation step
    pub fn step(&mut self, rtree: &RTree<TreePoint>, feed_dir: Option<Vec2>) -> Option<WorkerAnt> {
        let mut offspring = None;

            // put in colony purview, reproduction if life is big enough

            // if close_by.map_or(true, |(_,d2)|d2>200.) {
            //     let mut off = self.clone();
            //     off.age = 0;
            //     // while rand::gen_range(0., 1.) < 0.2 {
            //     //     off.mutate();
            //     // }
            //     off.life = off.base_life();
            //     off.random_move(1.5);
            //     offspring = Some(off);
            //     self.life = (adult_factor-1.)* self.base_life();
            // }


        self.pos += self.speed;
        self.pos.x = modulus(self.pos.x, screen_width());
        self.pos.y = modulus(self.pos.y, screen_height());
        self.speed *= 0.9;
        // self.life += (self.photosynthesis - self.metabolism())*0.4;
        // if rand::gen_range(0., 1.) < 0.2*self.motion {
            let speed = 5.;
            // if self.intelligence > 0. {
                // if let Some(feed_dir) = feed_dir {
                    // self.accelerate(feed_dir, speed);
                // } else {
                    // self.random_move(speed)
                // }
            // } else {
                self.random_move(speed);
            // }
        // }

        self.age += 1;
        offspring
    }
    // Fight between two ants
    // pub fn interact(worker_ants: &mut Vec<Self>, i:usize, j:usize) {
    //     let dist = (worker_ants[i].pos - worker_ants[j].pos).length();
    //     if dist < 10.* (worker_ants[i].weight() + worker_ants[j].weight()) {
    //         if worker_ants[i].stronger(&worker_ants[j]) {
    //             worker_ants[i].life += worker_ants[j].life * 0.8;
    //             worker_ants[j].life = 0.;
    //         }
    //         else if worker_ants[j].stronger(&worker_ants[i]) {
    //             worker_ants[j].life += worker_ants[i].life * 0.8;
    //             worker_ants[i].life = 0.;
    //         }
    //     }
    // }
    pub fn dead(&self) -> bool { //needs to be dead(&mut self) for it to be able to edit the dead property
        if self.life <= 0. || self.age >= 1000 {
            return true;
        }
        false

    }
    // Are we stronger than this other biot?
    // pub fn stronger(&self, other: &Self) -> bool {
    //     self.attack > other.attack + other.defense * 0.9
    // }
    // Compute chacteristics from biot genome
    // fn set_from_genome(&mut self) {
    //     self.attack= self.genome.iter().filter(|&&c|c=='a').count() as f32 * 0.1;
    //     self.defense= self.genome.iter().filter(|&&c|c=='d').count() as f32 * 0.1;
    //     self.photosynthesis= self.genome.iter().filter(|&&c|c=='p').count() as f32 * 0.1;
    //     self.motion= self.genome.iter().filter(|&&c|c=='m').count() as f32 * 0.1;
    //     self.intelligence= self.genome.iter().filter(|&&c|c=='i').count() as f32 * 10.;
    // }
    // Move in a random direction
    fn random_move(&mut self, speed: f32) {
        self.accelerate(vec2(rand::gen_range(0., 1.)-0.5, rand::gen_range(0., 1.)-0.5).normalize(), speed);
    }
    /// Apply acceleration in a certain direction
    fn accelerate(&mut self, dir:Vec2, speed: f32) {
        self.speed += dir *speed;
    }
    // Randomly mutate a single base
    // fn mutate(&mut self) {
    //     self.genome[rand::gen_range(0, self.genome.len() as u32) as usize] = LETTERS[rand::gen_range(0, LETTERS.len() as u32) as usize];
    //     self.set_from_genome();
    // }
    // Original life points of a biot. Also used to determine when the biot will spawn
    fn base_life(&self) -> f32 {
        10.
    }
    // Metabolic cost of various traits. These parameters are aboslutely critical to the
    // simulation
    // fn metabolism(&self) -> f32 {
    //     0.2*(4.5*self.attack + 2.3*self.defense + 2.5*self.motion + 0.1*self.intelligence)
    // }

    // fn weight(&self) -> f32 {
    //     self.attack + self.defense + self.photosynthesis + self.motion
    // }

}

/// Helper structure used for the rstar geometric data structure. This data structure is used for
/// computing interaction between biots fluidly even with thousands of them
pub struct TreePoint {
    pub x:f64,
    pub y:f64,
    pub idx: usize
}

impl RTreeObject for TreePoint {
    type Envelope = AABB<[f64; 2]>;
    fn envelope(&self) -> Self::Envelope
    {
        AABB::from_point([self.x, self.y])
    }
}

impl PointDistance for TreePoint
{
fn distance_2(&self, point: &<<Self as rstar::RTreeObject>::Envelope as rstar::Envelope>::Point) -> <<<Self as rstar::RTreeObject>::Envelope as rstar::Envelope>::Point as rstar::Point>::Scalar
{
    (self.x-point[0])*(self.x-point[0]) + (self.y-point[1])*(self.y-point[1])
}
}

pub struct Colony {
  life:f32,
  food_level:f32,
  pub pos:Vec2,
}
impl Colony {
    pub fn colony(posx:f32, posy:f32) -> Self {

        let q = Colony {
            life:10.,
            food_level:10.,
            pos: vec2(posx, posy)
        };
        q
    }

    pub fn update_food_values(mut self, AntCollection:WorkerAntCollection) {
        if self.food_level <=0. {
            //relese pheromones
            self.life -= 0.01;

        } else if self.food_level >0. || self.food_level < 15. {

            if self.life < 10. {
                while self.food_level > 0.06 && self.life < 10.{
                    self.food_level -= 0.01;
                    self.life += 0.01;
                }
                if self.life > 10. {
                    self.life = 10.;
                }
            }
            if self.food_level > 6.{
                self.food_level -= 0.5;

            }
        }
    }
}

pub struct Food {
    pub pos: Vec2,
    pub size: f32,
    pub exist: bool
}

impl Food {
    pub fn food(pos:Vec2) -> Self{
        let f = Self {
            pos,
            size: 2.,
            exist: true,
        };
        f
    }
    pub fn aggergate(food: &mut Vec<Self>, i:usize, j:usize,) {

        let distx = food[i].pos.x - food[j].pos.x;
        let disty = food[i].pos.y - food[j].pos.y;
        let distr = (((disty*disty)+(distx+distx)).sqrt())-food[i].size-food[j].size;

        if (distr) < 10. {
            if food[i].size > food[j].size {    
                food[i].pos.x += distx*food[j].size/food[i].size;
                food[i].pos.y += disty*food[j].size/food[i].size;
                food[i].size += food[j].size;
                food[j].exist = false;
            } else if food[j].size > food[i].size {
                food[j].pos.x += distx*food[i].size/food[j].size;
                food[j].pos.y += disty*food[i].size/food[j].size;
                food[j].size += food[i].size;
                food[i].exist = false;
            } else {
                food[j].pos.x += distx*0.5;
                food[j].pos.y += disty*0.5;
                food[j].size += food[i].size;
                food[i].exist = false;
            }

        }
    }
        
}


