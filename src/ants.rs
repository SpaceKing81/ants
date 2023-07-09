use std::{default};
use macroquad::{
    miniquad::gl::PFNGLCOMPRESSEDTEXIMAGE1DPROC,
    prelude::*,
};

mod old_code {
    // / Elements of the biots' genomes:
    // / a for attack
    // / d for defence
    // / p for photosynthesis
    // / m for motion
    // / i for intelligence
    // n does nothing
    // const LETTERS : &[char] = &['a','d','p','m', 'n', 'n', 'n', 'i'];

    // modulo  operator to get toroidal world topology

    // A single ant
    #[derive(Clone, Debug)]
    pub struct WorkerAnt {
        // Status
        life: f32,
        pub pos: Vec2,
        speed: Vec2,
        age: u32,
        pub dead: bool, // Genome
                        // genome: [char; 13],
                        // pub attack: f32,
                        // pub defense: f32,
                        // pub photosynthesis: f32,
                        // pub motion: f32,
                        // pub intelligence: f32,
    }

    impl WorkerAnt {
        /// Create a worker ant
        pub fn worker_ant(pos: Vec2) -> Self {
            // let mut genome = ['u';13];
            // for letter in genome.iter_mut() {
            //     *letter = LETTERS[rand::gen_range(0, LETTERS.len() as u32) as usize];
            // }
            let mut s = Self {
                life: 0.,
                pos,
                speed: vec2(2., 2.),
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
        pub fn step(
            &mut self,
            rtree: &RTree<TreePoint>,
            feed_dir: Option<Vec2>,
        ) -> Option<WorkerAnt> {
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
            self.pos.x = modulo(self.pos.x, screen_width());
            self.pos.y = modulo(self.pos.y, screen_height());
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
        pub fn dead(&self) -> bool {
            //needs to be dead(&mut self) for it to be able to edit the dead property
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
            self.accelerate(
                vec2(rand::gen_range(0., 1.) - 0.5, rand::gen_range(0., 1.) - 0.5).normalize(),
                speed,
            );
        }
        /// Apply acceleration in a certain direction
        fn accelerate(&mut self, dir: Vec2, speed: f32) {
            self.speed += dir * speed;
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
        pub x: f64,
        pub y: f64,
        pub idx: usize,
    }

    impl RTreeThings for TreePoint {
        type Envelope = AABB<[f64; 2]>;
        fn envelope(&self) -> Self::Envelope {
            AABB::from_point([self.x, self.y])
        }
    }

    impl PointDistance for TreePoint {
        fn distance_2(&self, point: &<<Self as rstar::RTreeThings>::Envelope as rstar::Envelope>::Point) -> <<<Self as rstar::RTreeThings>::Envelope as rstar::Envelope>::Point as rstar::Point>::Scalar
{
            (self.x - point[0]) * (self.x - point[0]) + (self.y - point[1]) * (self.y - point[1])
        }
    }

    pub struct Colony {
        life: f32,
        food_level: f32,
        pub pos: Vec2,
    }
    impl Colony {
        pub fn colony(posx: f32, posy: f32) -> Self {
            let q = Colony {
                life: 10.,
                food_level: 10.,
                pos: vec2(posx, posy),
            };
            q
        }

        pub fn update_food_values(mut self, AntCollection: WorkerAntCollection) {
            if self.food_level <= 0. {
                //relese pheromones
                self.life -= 0.01;
            } else if self.food_level > 0. || self.food_level < 15. {
                if self.life < 10. {
                    while self.food_level > 0.06 && self.life < 10. {
                        self.food_level -= 0.01;
                        self.life += 0.01;
                    }
                    if self.life > 10. {
                        self.life = 10.;
                    }
                }
                if self.food_level > 6. {
                    self.food_level -= 0.5;
                }
            }
        }
    }

    pub struct Food {
        pub pos: Vec2,
        pub size: f32,
        pub exist: bool,
    }

    impl Food {
        pub fn food(pos: Vec2) -> Self {
            let f = Self {
                pos,
                size: 2.,
                exist: true,
            };
            f
        }
        pub fn aggergate(food: &mut Vec<Self>, i: usize, j: usize) {
            let distx = food[i].pos.x - food[j].pos.x;
            let disty = food[i].pos.y - food[j].pos.y;
            let distr = (((disty * disty) + (distx + distx)).sqrt()) - food[i].size - food[j].size;

            if (distr) < 10. {
                if food[i].size > food[j].size {
                    food[i].pos.x += distx * food[j].size / food[i].size;
                    food[i].pos.y += disty * food[j].size / food[i].size;
                    food[i].size += food[j].size;
                    food[j].exist = false;
                } else if food[j].size > food[i].size {
                    food[j].pos.x += distx * food[i].size / food[j].size;
                    food[j].pos.y += disty * food[i].size / food[j].size;
                    food[j].size += food[i].size;
                    food[i].exist = false;
                } else {
                    food[j].pos.x += distx * 0.5;
                    food[j].pos.y += disty * 0.5;
                    food[j].size += food[i].size;
                    food[i].exist = false;
                }
            }
        }
    }
}

fn modulo<T>(a: T, b: T) -> T where T: std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy, {((a % b) + b) % b} // calculate modulus operations
  /* possible ideas/additions to add later:

  */
pub struct Things {
    // What colony it belongs to
    alligence: i32,
    // 'worker' 'queen' 'soldier' 'defender' 'scout' 'food' 'scent'
    otype: str,
    // dead or alive
    dead: bool,
    // amount of health 0 -> dead
    hp: f32,
    // age of ant
    age: u128,
    // speed in x and y
    vel: Vec2,
    // position of x and y
    pos: Vec2,
    // amount of food, 0 -> starving
    hunger: f32,
    // the amount that can be carried
    strength: f32,
    // attack value
    att_str: f32,
    // damage midagation
    armor: f32,
    // visual size on screen, amount of food it provides, weight of ant
    mass: f32,
    // duration before becoming tired 0 -> tired
    stamina: f32,
    // Pheromone Governers
    // Pheromone strength of what the scent conveys: 'f'food 'd'danger 't'tohome 'h'home
    pher_f: f32,
    pher_d: f32,
    pher_t: f32,
    pher_h: f32,
    // Detection Range of Pheromone, the distance an ant can detect different pheromones
    detect: f32,
}
/*Things {
alligence: i32,
otype: str,
dead: bool,
hp: f32,
age: u128
vel: Vec2,
pos: Vec2,
hunger: f32,
detect: f32, 
strength: f32,
att_str: f32,
armor: f32,
mass: f32,
stamina: f32,
pher_f: f32,
pher_d: f32,
pher_t: f32,
pher_h: f32,
*/

impl Things {
    // queen ant
    fn new_queen(posx: f32, posy: f32, pher_d: f32) -> Things {
        // creates new queen
        posx = modulo(posx, screen_width());
        posy = modulo(posy, screen_height());

        let mut q: Things = Things {
            alligence: 0,
            otype: "queen",
            dead: false,
            hp: 10.,
            age: 0,
            vel: vec2(0., 0.),
            pos: vec2(posx, posy),
            hunger: 10.,
            strength: 7.,
            att_str: 1.,
            armor: 0.,
            mass: 10.,
            stamina: 10.,
            pher_f: 3.,
            pher_d,
            pher_t: 0.,
            pher_h: 10.,
            detect: 4.,
        };
        q
    }
    pub fn birth(&self) -> Vec<Things> { // Possible to place fn in collections file, more thinking needed, spits out unsorted kids vec. 
        let kids: Vec<Things> = Vec::new();
        self.vel = vec2(0., 0.);
        self.hunger -= 3.;
        let d = 2;
        let num = rand::gen_range(1, 100);

        if rand::gen_range(0., 1.) * 100. < 0.4 {
            kids.append(Self::new_queen(&self.pos.x, &self.pos.y, &self.pher_d));
        } else {
            let num = Some((rand::gen_range(0.,5.)*&self.hunger + &self.pher_d)/&self.hp);
            for i in 1..20 {
                match num {
                    Some(x) if x <= 3 => kids.append(Self::new_worker(&self)),
                    Some(x) if x > 3 && x <=5 => kids.append(Self::new_scout(&self)),
                    Some(x) if x > 5 && x <= 7 =>kids.append(Self::new_soldier(&self)),
                    _=> kids.append(Self::new_defender(&self)),
                }
            }
        }
        kids
    }
    pub fn feed(queen: Vec<Things>, food: Vec<Vec<Things>>,) -> (Vec<Things>, Vec<Vec<Things>>) { //causes all queens to feed, containes updated queens, updated food

        for i in 0..queen.len() {
            for x in 0..food.len() {    
                for j in 0..food[x].len() {
                    let distx = queen[i].pos.x - food[x][j].pos.x;
                    let disty = queen[i].pos.y - food[x][j].pos.y;
                    let distr = f32::sqrt((distx * distx) + (disty * disty)) - queen[i].mass - food[x][j].mass;

                    if distr < 5. {
                        queen[i].hunger += food[x][j].size;
                        food[x][j].remove();
                        while queen[i].hunger > 10. {
                            let fat = queen[i].hunger - 10.;
                            queen[i].hunger -= fat;
                            queen[i].mass += fat;
                        }
                    }
                }
            }
        }
        return (queen, food);
    }
}
impl Things {
    //worker
    fn new_worker(&self) -> Things {
        let mut w: Things = Things {
            alligence: 0,
            otype: "worker",
            dead: false,
            hp: 8.,
            age: 0,
            vel: vec2(0., 0.),
            pos: &self.pos,
            hunger: 0.,
            strength: 20.,
            att_str: 4.,
            armor: 2.,
            mass: 5.,
            stamina: 30.,
            pher_f: 0.,
            pher_d: &self.pher_d,
            pher_t: 10.,
            pher_h: 0.,
            detect: 7.,
            
        };
        w
    }
}
impl Things {
    // food
    pub fn new_food(n:u128) -> Vec<Things> { //only to be used in the beginning of the simulation
        let raw_food = Vec::new();
        for i in 0..n {
            let f = Things {
                alligence: 0,
                otype: "food",
                dead: true,
                hp: 0,
                age: 0,
                vel: vec2(0., 0.),
                pos: vec2(rand::gen_range(0, screen_width()), rand::gen_range(0, screen_height())),
                hunger: 0,
                strength: 0,
                att_str: 0,
                armor: 0,
                mass: 2,
                stamina: 0,
                pher_f: 0,
                pher_d: 0,
                pher_t: 0,
                pher_h: 0,
                detect: 0.,
            };
            raw_food.push(f);
        }
        raw_food
    }
    fn convert_to_food(deadOnes: Vec<Things>) -> Vec<Things> { //changes dead ants into food
        for i in deadOnes {
                i.alligence = 0;
                i.otype = "food";
                i.dead = true;
                i.hp = 0.;
                i.age = 0.;
                i.vel = vec2(0, 0);
                i.hunger = 0.;
                i.strength = 0.;
                i.att_str = 0.;
                i.armor = 0.;
                i.stamina = 0.;
                i.pher_f = 0.;
                i.pher_d = 0.;
                i.pher_t = 0.;
                i.pher_h = 0.;
                i.pher_detect = 0.;
                i.mass *= 0.9;
        }
        deadOnes
    }
}
impl Things {
    // pheromones?
    fn new_pher(&self) -> Vec<Things>{ //generates a new set of pheremones based on an ant or food piece
        let source = &self;
        let mut new_h = Things{
            alligence: 0,
            otype: "scent",
            dead: false,
            hp: 0.,
            age: 0.,
            vel: vec2(0., 0.),
            detect: 0.,
            hunger: 0.,
            strength: 0.,
            att_str: 0.,
            armor: 0.,
            mass: 0.,
            stamina: 0.,
            pher_f: 0.,
            pher_d: 0.,
            pher_t: 0.,
            pos: vec2(rand::gen_range(0., source.pher_h) + source.pos.x, rand::gen_range(0., source.pher_h) + source.pos.y),
            ..source
        };
        let mut new_f = Things{
            alligence: 0,
            otype: "scent",
            dead: false,
            hp: 0.,
            age: 0.,
            vel: vec2(0., 0.),
            detect: 0.,
            hunger: 0.,
            strength: 0.,
            att_str: 0.,
            armor: 0.,
            mass: 0.,
            stamina: 0.,
            pher_h: 0.,
            pher_d: 0.,
            pher_t: 0.,
            pos: vec2(rand::gen_range(0., source.pher_f) + source.pos.x, rand::gen_range(0., source.pher_f) + source.pos.y),
            ..source
        };
        let mut new_d = Things{
            alligence: 0,
            otype: "scent",
            dead: false,
            hp: 0.,
            age: 0.,
            vel: vec2(0., 0.),
            detect: 0.,
            hunger: 0.,
            strength: 0.,
            att_str: 0.,
            armor: 0.,
            mass: 0.,
            stamina: 0.,
            pher_f: 0.,
            pher_h: 0.,
            pher_t: 0.,
            pos: vec2(rand::gen_range(0., source.pher_d) + source.pos.x, rand::gen_range(0., source.pher_d) + source.pos.y),
            ..source
        };
        let mut new_t = Things{
            alligence: 0,
            otype: "scent",
            dead: false,
            hp: 0.,
            age: 0.,
            vel: vec2(0., 0.),
            detect: 0.,
            hunger: 0.,
            strength: 0.,
            att_str: 0.,
            armor: 0.,
            mass: 0.,
            stamina: 0.,
            pher_f: 0.,
            pher_d: 0.,
            pher_h: 0.,
            pos: vec2(rand::gen_range(0., source.pher_t) + source.pos.x, rand::gen_range(0., source.pher_t) + source.pos.y),
            ..source
        };
        let output = vec![new_d, new_f, new_h, new_t];
        output
    }
    fn disperse(phers: Vec<Things>) -> Vec<Things> { //disperses the pheremones given. 
        let new_phers = Vec::new();
        for i in 0..phers.len() {
            phers[i].pher_d *= 0.3;
            phers[i].pher_f *= 0.3;
            phers[i].pher_t *= 0.3;
            phers[i].pher_h *= 0.3;
            new_phers.append(Self::new_pher(&phers[i]));
            new_phers.append(Self::new_pher(&phers[i]));
            new_phers.append(Self::new_pher(&phers[i]));
        }
        let finish = Vec::new();
        for i in new_phers {
            if i.pher_h < 0.2 && i.pher_t < 0.2 && i.pher_d < 0.2 && i.pher_f < 0.2 {
                None;
            } else {
                finish.append(i);
            }
        }
        finish
    }
}
impl Things {
    // scout
    fn new_scout(&self) -> Things {
        let mut s: Things = Things {
            alligence: 0,
            otype: "scout",
            dead: false,
            hp: 10.,
            age: 0,
            vel: vec2(0., 0.),
            pos: &self.pos,
            hunger: 0.,
            strength: 10.,
            att_str: 2.,
            armor: 3.,
            mass: 4.,
            stamina: 50.,
            pher_f: 0.,
            pher_d: &self.pher_d,
            pher_t: 10.,
            pher_h: 0.,
            detect: 20.,
            
        };
        s
    }
}
impl Things {
    // soldier
    fn new_soldier(&self) -> Things {
        let mut a: Things = Things {
            alligence: 0,
            otype: "soldier",
            dead: false,
            hp: 20.,
            age: 0,
            vel: vec2(0., 0.),
            pos: &self.pos,
            hunger: 0.,
            strength: 10.,
            att_str: 10.,
            armor: 5.,
            mass: 7.,
            stamina: 50.,
            detect: 4.,
            pher_f: 0.,
            pher_t: 10.,
            pher_h: 0.,
            pher_d: &self.pher_d,
            
        };
        a
    }
}
impl Things {
    // defender
    fn new_defender(&self) -> Things {
        let mut d: Things = Things {
            alligence: 0,
            otype: "defender",
            dead: false,
            hp: 50.,
            age: 0,
            vel: vec2(0., 0.),
            pos: &self.pos,
            hunger: 0.,
            strength: 15.,
            att_str: 2.,
            armor: 10.,
            mass: 15.,
            stamina: 30.,
            pher_f: 0.,
            pher_d: &self.pher_d,
            pher_t: 10.,
            pher_h: 0.,
            detect: 3.,
            
        };
        d
    }
}
impl Things {
    // general fn, overlaping classes
    pub fn sorter(chaos: Vec<Things>) -> (Vec<Things>,Vec<Things>,Vec<Things>,Vec<Things>,Vec<Things>,Vec<Things>,Vec<Things>) {
        let queen:Vec<Things> = Vec::new();
        let worker:Vec<Things> = Vec::new();
        let soldier:Vec<Things> = Vec::new();
        let defender:Vec<Things> = Vec::new();
        let scout:Vec<Things> = Vec::new();
        let food :Vec<Things> = Vec::new();
        let scent:Vec<Things> = Vec::new();
        for i in 0..chaos.len() {
            match chaos[i].otype {
                "queen" => queen.append(chaos[i]),
                "worker" => worker.append(chaos[i]),
                "soldier" => soldier.append(chaos[i]),
                "defender" => defender.append(chaos[i]),
                "scout" => scout.append(chaos[i]),
                "food" => food.append(chaos[i]),
                "scent" => scent.append(chaos[i]),
                _ => println!("faliure found, sorter fn sourced"),
            }
        }
        (queen,worker,soldier,defender,scout,food,scent)
    }
    pub fn colorShaper(&self) {
        match self.otype {
            "worker" => draw_circle(&self.pos.x, &self.pos.y, &self.mass, DARKBLUE),
            "queen"=> draw_circle(&self.pos.x, &self.pos.y, &self.mass, GOLD),
            "soldier"=> draw_circle(&self.pos.x, &self.pos.y, &self.mass, RED),
            "defender"=> draw_circle(&self.pos.x, &self.pos.y, &self.mass, YELLOW),
            "scout"=> draw_circle(&self.pos.x, &self.pos.y, &self.mass, SKYBLUE),
            "food"=> draw_circle(&self.pos.x, &self.pos.y, &self.mass, ORANGE),
            "scent"=> if self.pher_t > 0 {
                draw_circle(&self.pos.x, &self.pos.y, &self.pher_t, GRAY);
            } else if self.pher_h > 0 {
                draw_circle(&self.pos.x, &self.pos.y, &self.pher_h, LIME);
            } else if self.pher_f > 0 {
                draw_circle(&self.pos.x, &self.pos.y, &self.pher_f, MAROON);
            } else if self.pher_d > 0 {
                draw_circle(&self.pos.x, &self.pos.y, &self.pher_d, VIOLET);
            }
            _=> println!("error color matcher")
        }
    }
    pub fn orientation(&self) -> f32 { // takes a specifice ant, and returns its angle of orientation based on its velocity
        let x = &self.vel.x;
        let y = &self.vel.y;
        let degree;
        if x == 0 {            
            let y = y.abs() == y;
            if y { 
                return 90.;
            } else {
                return = 270.;
            }
        }
        let degree = f32::to_degrees(f32::atan(y/x));
        let x = x.abs() == x;
        let y = y.abs() == y;
        match y {
            true => {match x {
                true => return degree,
                false => return 180 - degree,
            }},
            false => {match x {
                true => return 360 - degree,
                false => return degree + 180,
            }}
        }
    }
    pub fn degree_finder(x: f32, y: f32) -> f32 { // takes a specifice ant, and returns its angle of orientation based on its velocity
        let degree;
        if x == 0 {            
            let y = y.abs() == y;
            if y { 
                return 90.;
            } else {
                return = 270.;
            }
        }
        let degree = f32::to_degrees(f32::atan(y/x));
        let x = x.abs() == x;
        let y = y.abs() == y;
        match y {
            true => {match x {
                true => return degree,
                false => return 180 - degree,
            }},
            false => {match x {
                true => return 360 - degree,
                false => return degree + 180,
            }}
        }
    }
    pub fn in_detect_range_check(&self, check_pos: Vec2) -> bool {
        /*
        first check if object is in circular detection range
        second find the absolute equation of the circle with a radius of ant detection, with Vec2<0,0> being origin.
        math to find equation describing all points that reside in the circle
        true, false
         */
        let xdis = self.pos.x - check_pos.x;
        let ydis = self.pos.y - check_pos.y;
        if f32::sqrtf32(xdis*xdis + ydis*ydis) > self.detect { return false }

        let degree = modulo(Self::degree_finder(xdis, ydis), 360.) - self.orientation();
        if degree.abs() >= 45. { return false }
        true
    
    }
    pub fn move_to_food(&self) {
        let topSpeed = self.strength *2. - self.mass - self.armor;
        let speed =  self.strength - self.mass - self.armor;
        let stamina = self.stamina;
        
        
        

        self.pos += self.vel;
    }


}

