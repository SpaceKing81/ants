use std::{default, collections::HashMap};
use macroquad::{
    miniquad::{gl::PFNGLCOMPRESSEDTEXIMAGE1DPROC, native::apple::frameworks::Object},
    prelude::*, rand,
};


fn modulo<T>(a: T, b: T) -> T where T: std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy, {((a % b) + b) % b} // calculate modulus operations
/* possible ideas/additions to add later:

  */
#[derive(Clone)]
#[derive(Copy)]
pub struct Things {
    // What colony it belongs to
    pub alligence: u32,
    // w = 'worker' q = 'queen' k = 'soldier' d = 'defender' s = 'scout' f = 'food' p = 'scent'
    pub otype: char,
    // dead or alive
    pub dead: bool,
    // amount of health 0 -> dead
    pub hp: f32,
    // age of ant
    pub age: u128,
    // speed in x and y
    vel: Vec2,
    // position of x and y
    pub pos: Vec2,
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
    pher: f32,
    pher_h: f32,
    // Detection Range of Pheromone, the distance an ant can detect different pheromones
    detect: f32,
}
/*Things {
alligence: u32,
otype: char,
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
pher: f32,
pher_h: f32,
*/

impl Things {
    // queen ant
    pub fn new_queen(posx: f32, posy: f32, pher_d: f32) -> Things {
        // creates new queen
        let posx = modulo(posx, screen_width());
        let posy = modulo(posy, screen_height());

        let mut q: Things = Things {
            alligence: 0,
            otype: 'q',
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
            pher: 0.,
            pher_h: 10.,
            detect: 4.,
        };
        q
    }
    pub fn birth(&mut self) -> Vec<Things> { // Possible to place fn in collections file, more thinking needed, spits out unsorted kids vec. 
        let mut kids: Vec<Things> = Vec::new();
        self.vel = vec2(0., 0.);
        self.hunger -= 3.;
        let d = 2;
        let num = rand::gen_range(1, 100);

        if rand::gen_range(0., 1.) * 100. < 0.4 {
            kids.push(Self::new_queen(self.pos.x, self.pos.y, self.pher_d));
        } else {
            let num = Some((rand::gen_range(0.,5.)*&self.hunger + &self.pher_d)/&self.hp);
            for i in 1..20 {
                match num {
                    Some(x) if x <= 3. => kids.push(Self::new_worker(&self)),
                    Some(x) if x > 3. && x <=5. => kids.push(Self::new_scout(&self)),
                    Some(x) if x > 5. && x <= 7. =>kids.push(Self::new_soldier(&self)),
                    _=> kids.push(Self::new_defender(&self)),
                }
            }
        }
        kids
    }
    pub fn feed(mut queen: Vec<Things>, mut food: Vec<Vec<Things>>,) -> (Vec<Things>, Vec<Vec<Things>>) { //causes all queens to feed, containes updated queens, updated food

        for i in 0..queen.len() {
            for x in 0..food.len() {    
                for j in 0..food[x].len() {
                    let distx = queen[i].pos.x - food[x][j].pos.x;
                    let disty = queen[i].pos.y - food[x][j].pos.y;
                    let distr = f32::sqrt((distx * distx) + (disty * disty)) - queen[i].mass - food[x][j].mass;

                    if distr < 5. {
                        queen[i].hunger += food[x][j].mass;
                        food[x].remove(j);
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
            otype: 'w',
            dead: false,
            hp: 8.,
            age: 0,
            vel: vec2(0., 0.),
            pos: self.pos,
            hunger: 0.,
            strength: 20.,
            att_str: 4.,
            armor: 2.,
            mass: 5.,
            stamina: 30.,
            pher_f: 0.,
            pher_d: self.pher_d,
            pher: 10.,
            pher_h: 0.,
            detect: 7.,
            
        };
        w
    }
}
impl Things {
    // food
    pub fn new_food(amount:u128) -> Vec<Things> { //only to be used in the beginning of the simulation
        let mut raw_food = Vec::new();
        for i in 0..amount {
            let f = Things {
                alligence: 0,
                otype: 'f',
                dead: true,
                hp: 0.,
                age: 0,
                vel: vec2(0., 0.),
                pos: vec2(rand::gen_range(0., screen_width()), rand::gen_range(0., screen_height())),
                hunger: 0.,
                strength: 0.,
                att_str: 0.,
                armor: 0.,
                mass: 2.,
                stamina: 0.,
                pher_f: 0.,
                pher_d: 0.,
                pher: 0.,
                pher_h: 0.,
                detect: 0.,
            };
            raw_food.push(f);
        }
        raw_food
    }
    pub fn convert_to_food(dead_ones: Vec<Things>) -> Vec<Things> { //changes dead ants into food
        let mut new_food = Vec::new();
        for mut i in dead_ones {
                i.alligence = 0;
                i.otype = 'f';
                i.dead = true;
                i.hp = 0.;
                i.age = 0;
                i.vel = vec2(0., 0.);
                i.hunger = 0.;
                i.strength = 0.;
                i.att_str = 0.;
                i.armor = 0.;
                i.stamina = 0.;
                i.pher_f = 0.;
                i.pher_d = 0.;
                i.pher = 0.;
                i.pher_h = 0.;
                i.detect = 0.;
                i.mass *= 0.9;
                new_food.push(i);
            }
            new_food
        }
    pub fn glob_food(food_pieces: &mut Vec<Things>, max_distance: f32) {
        let mut grouped_indices = Vec::new();

        for i in 0..food_pieces.len() {
            if grouped_indices.contains(&i) {
                continue; // Skip if already grouped
            }

            let mut group_mass = food_pieces[i].mass;
            let mut group_center = food_pieces[i].pos * food_pieces[i].mass;

            for (j, food2) in food_pieces.iter().enumerate() {
                if i == j || grouped_indices.contains(&j) {
                    continue; // Skip if same food piece or already grouped
                }

                let distance = (food_pieces[i].pos - food2.pos).length();

                if distance <= max_distance {
                    grouped_indices.push(j);
                    group_mass += food2.mass;
                    group_center += food2.pos * food2.mass;
                }
            }

            // Update position of grouped food pieces
            if group_mass > food_pieces[i].mass {
                food_pieces[i].pos = group_center / group_mass;
                food_pieces[i].mass = group_mass;
            }
        }

        // Remove grouped food pieces
        grouped_indices.sort_by(|a, b| b.cmp(a)); // Sort in descending order
        for i in grouped_indices {
            food_pieces.remove(i);
        }
    
    }
    
    fn siphon_food(&self, mass:f32) -> Things { //allows for spliting a large Raw-food bundle up for carrying. Used in pick_food()
            let f = Things {
                alligence: 0,
                otype: 'f',
                dead: true,
                hp: 0.,
                age: 0,
                vel: vec2(self.vel.x, self.vel.y),
                pos: vec2(self.pos.x, self.pos.y),
                hunger: 0.,
                strength: 0.,
                att_str: 0.,
                armor: 0.,
                mass,
                stamina: 0.,
                pher_f: 0.,
                pher_d: 0.,
                pher: 0.,
                pher_h: 0.,
                detect: 0.,
            };
            f
    }
    fn pick_food(mut self, mut food_piece:Things) -> Things { //ants picks up specific food piece, makes a group of ant-food that can be split up when food is delivered
        let taken_piece_size = &self.strength - &self.mass;
        let small_enough = food_piece.mass < taken_piece_size;
        if small_enough {
            food_piece.pos = self.pos;
            food_piece.vel = self.vel;
            self.mass += food_piece.mass;
            return food_piece;
        }
        food_piece.mass = food_piece.mass - taken_piece_size;
        let taken_piece = self.siphon_food(taken_piece_size);
        self.mass += taken_piece.mass;
        taken_piece

    }

}
impl Things {
    // pheromones
    fn new_pher(&self) -> Vec<Things>{ //generates a new set of pheremones based on an ant or food piece;
        let mut new_h = Things{
            alligence: 0,
            otype: 'p',
            dead: false,
            hp: 0.,
            age: 0,
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
            pher: 0.,
            pos: vec2(rand::gen_range(0., self.pher_h) + self.pos.x, rand::gen_range(0., self.pher_h) + self.pos.y),
            pher_h: self.pher_h,
        };
        let mut new_f = Things{
            alligence: 0,
            otype: 'p',
            dead: false,
            hp: 0.,
            age: 0,
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
            pher: 0.,
            pos: vec2(rand::gen_range(0., self.pher_f) + self.pos.x, rand::gen_range(0., self.pher_f) + self.pos.y),
            pher_f: self.pher_f,
        };
        let mut new_d = Things{
            alligence: 0,
            otype: 'p',
            dead: false,
            hp: 0.,
            age: 0,
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
            pher: 0.,
            pos: vec2(rand::gen_range(0., self.pher_d) + self.pos.x, rand::gen_range(0., self.pher_d) + self.pos.y),
            pher_d: self.pher_d,
        };
        let mut new_t = Things{
            alligence: 0,
            otype: 'p',
            dead: false,
            hp: 0.,
            age: 0,
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
            pos: vec2(rand::gen_range(0., self.pher) + self.pos.x, rand::gen_range(0., self.pher) + self.pos.y),
            pher: self.pher,
        };
        let mut output = vec![new_d, new_f, new_h, new_t];
        output
    }
    pub fn pher_sorter<'a>(scents: Vec<Things>, foodp: &'a str, dangerp: &'a str, thome: &'a str, homep: &'a str) -> HashMap<&'a str, Vec<Things>> {
        let mut pher_h = Vec::new();
        let mut pher_f = Vec::new();
        let mut pher = Vec::new();
        let mut pher_d = Vec::new();
        for i in scents {
            if i.pher_d > 0.02 {
                pher_d.push(i.clone())
            }
            if i.pher_f > 0.02 {
                pher_f.push(i.clone())
            }
            if i.pher_h > 0.02 {
                pher_h.push(i.clone())
            }
            if i.pher > 0.02 {
                pher.push(i.clone())
            }
        }
        let mut output:HashMap<&str, Vec<Things>> = [(foodp,pher_f), (dangerp,pher_d), (thome,pher), (homep,pher_h)]
        .iter()
        .cloned()
        .collect();
        output
    }
    pub fn disperse(mut all_scents: Vec<Things>) -> Vec<Things> { //disperses the pheremones given. 
        let mut new_phers: Vec<Things> = Vec::new();
        for i in 0..all_scents.len() {
            let random = vec2(rand::gen_range(-3., 3.), rand::gen_range(-3., 3.));
            
            all_scents[i].pher_d *= 0.67;
            all_scents[i].pher_f *= 0.67;
            all_scents[i].pher *= 0.67;
            all_scents[i].pher_h *= 0.67;
            
            all_scents[i].pos + random;

            
        }

        new_phers.retain(|i| i.pher_h < 0.02 && i.pher < 0.02 && i.pher_d < 0.02 && i.pher_f < 0.02);
        Self::combine_phers(&mut new_phers);
        
        new_phers

        
    }
    fn combine_phers(new_phers: &mut Vec<Things>) {
        let mut grouped_indices = Vec::new();

        for i in 0..new_phers.len() {
            if grouped_indices.contains(&i) {
                continue; // Skip if already grouped
            }

            let  group_center = new_phers[i].pos;
            let mut group_pher_d = new_phers[i].pher_d;
            let mut group_pher_f = new_phers[i].pher_f;
            let mut group_pher_h = new_phers[i].pher_h;
            let mut group_pher = new_phers[i].pher;

            for (j, food2) in new_phers.iter().enumerate() {
                if i == j || grouped_indices.contains(&j) {
                    continue; // Skip if same pher or already grouped
                }

            
                if group_center == food2.pos {
                    grouped_indices.push(j);
                    group_pher_d += food2.pher_d;
                    group_pher_h += food2.pher_h;
                    group_pher_f += food2.pher_f;
                    group_pher += food2.pher;
                }
            }

            // Update value of combined phers
            if grouped_indices.len() > 0 {
                new_phers[i].pher = group_pher;
                new_phers[i].pher_h = group_pher_h;
                new_phers[i].pher_f = group_pher_f;
                new_phers[i].pher_d = group_pher_d;
            }
        }

        // Remove redundent phers
        grouped_indices.sort_by(|a, b| b.cmp(a)); // Sort in descending order
        for i in grouped_indices {
            new_phers.remove(i);
        }
    
    }
    
}
impl Things {
    // scout
    fn new_scout(&self) -> Things {
        let mut s: Things = Things {
            alligence: 0,
            otype: 's',
            dead: false,
            hp: 10.,
            age: 0,
            vel: vec2(0., 0.),
            pos: self.pos,
            hunger: 0.,
            strength: 10.,
            att_str: 2.,
            armor: 3.,
            mass: 4.,
            stamina: 50.,
            pher_f: 0.,
            pher_d: self.pher_d,
            pher: 10.,
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
            otype: 'k',
            dead: false,
            hp: 20.,
            age: 0,
            vel: vec2(0., 0.),
            pos: self.pos,
            hunger: 0.,
            strength: 10.,
            att_str: 10.,
            armor: 5.,
            mass: 7.,
            stamina: 50.,
            detect: 4.,
            pher_f: 0.,
            pher: 10.,
            pher_h: 0.,
            pher_d: self.pher_d,
            
        };
        a
    }
}
impl Things {
    // defender
    fn new_defender(&self) -> Things {
        let mut d: Things = Things {
            alligence: 0,
            otype: 'd',
            dead: false,
            hp: 50.,
            age: 0,
            vel: vec2(0., 0.),
            pos: self.pos,
            hunger: 0.,
            strength: 15.,
            att_str: 2.,
            armor: 10.,
            mass: 15.,
            stamina: 30.,
            pher_f: 0.,
            pher_d: self.pher_d,
            pher: 10.,
            pher_h: 0.,
            detect: 3.,
            
        };
        d
    }
}
impl Things {
    // general fn, overlaping classes
    pub fn sorter(chaos: Vec<Things>) -> (Vec<Things>,Vec<Things>,Vec<Things>,Vec<Things>,Vec<Things>,Vec<Things>,Vec<Things>) {
        let mut queen:Vec<Things> = Vec::new();
        let mut worker:Vec<Things> = Vec::new();
        let mut soldier:Vec<Things> = Vec::new();
        let mut defender:Vec<Things> = Vec::new();
        let mut scout:Vec<Things> = Vec::new();
        let mut food :Vec<Things> = Vec::new();
        let mut scent:Vec<Things> = Vec::new();
        for mut i in chaos{
            match i.otype {
                'q' => queen.push(i),
                'w' => worker.push(i),
                'k' => soldier.push(i),
                'd' => defender.push(i),
                's' => scout.push(i),
                'f' => food.push(i),
                'p' => scent.push(i),
                _ => println!("faliure in the sorter fn"),
            }
        }
        (queen,worker,soldier,defender,scout,food,scent)
    }
    pub fn color_shaper(&self) {
        match self.otype {
            'w'=> draw_circle(self.pos.x, self.pos.y, self.mass, DARKBLUE),
            'q'=> draw_circle(self.pos.x, self.pos.y, self.mass, GOLD),
            'k'=> draw_circle(self.pos.x, self.pos.y, self.mass, RED),
            'd'=> draw_circle(self.pos.x, self.pos.y, self.mass, YELLOW),
            's'=> draw_circle(self.pos.x, self.pos.y, self.mass, SKYBLUE),
            'f'=> draw_circle(self.pos.x, self.pos.y, self.mass, ORANGE),
            'p'=> if self.pher > 0. {
                draw_circle(self.pos.x, self.pos.y, self.pher, GRAY);
            } else if self.pher_h > 0. {
                draw_circle(self.pos.x, self.pos.y, self.pher_h, LIME);
            } else if self.pher_f > 0. {
                draw_circle(self.pos.x, self.pos.y, self.pher_f, MAROON);
            } else if self.pher_d > 0. {
                draw_circle(self.pos.x, self.pos.y, self.pher_d, VIOLET);
            }
            _=> println!("error within the color matcher fn")
        }
    }
    pub fn orientation(&self) -> f32 { // takes a specifice ant, and returns its angle of orientation based on its velocity
        let x = self.vel.x;
        let y = self.vel.y;
        let degree: f32;
        if x == 0. {            
            let y = y.abs() == y;
            if y { 
                return 90.;
            } else {
                return 270.;
            }
        }
        let degree = f32::to_degrees(f32::atan(y/x));
        let x = x.abs() == x;
        let y = y.abs() == y;
        match y {
            true => {match x {
                true => return degree,
                false => return 180. - degree,
            }},
            false => {match x {
                true => return 360. - degree,
                false => return degree + 180.,
            }}
        }
    }
    pub fn degree_finder(x: f32, y: f32) -> f32 { // takes an x and y movment, and returns its angle of orientation
        let degree:f32;
        if x == 0. {            
            let y = y.abs() == y;
            if y { 
                return 90.;
            } else {
                return 270.;
            }
        }
        let degree = f32::to_degrees(f32::atan(y/x));
        let x = x.abs() == x;
        let y = y.abs() == y;
        match y {
            true => {match x {
                true => return degree,
                false => return 180. - degree,
            }},
            false => {match x {
                true => return 360. - degree,
                false => return degree + 180.,
            }}
        }
    }
    pub fn in_detect_range_check(&self, check_pos: Vec2) -> bool {
        /*
        first checks if object is in circular detection range
        second find the absolute equation of the circle with a radius of ant detection, with Vec2<0,0> being origin.
        math to find equation describing all points that reside in the circle
        true, false
        done
         */
        let xdis = self.pos.x - check_pos.x;
        let ydis = self.pos.y - check_pos.y;
        if (xdis*xdis + ydis*ydis).sqrt() > self.detect { return false }

        let degree = modulo(Self::degree_finder(xdis, ydis), 360.) - self.orientation();
        if degree.abs() >= 45. { return false }
        true
    
    }
    fn far_away(&self, check_pos: Vec2) -> bool {
        let a = self.pos.x - check_pos.x;
        let b = self.pos.y - check_pos.y;
        let c = ((a*a) + (b*b)).sqrt();
        if c <= self.detect {return true}
        false
    }
    fn turn_right(&mut self, far: bool) {
        let a = self.vel.x;
        let b = self.vel.y;
        let c = (a*a)+(b*b).sqrt();
        let current_degree = self.orientation();
        let new_degree = current_degree - 18.;
        if far { let new_degree = new_degree - 18.; }
        self.vel = vec2(new_degree.cos()*c, new_degree.sin()*c)
    }
    fn turn_left(&mut self, far: bool) {
        let a = self.vel.x;
        let b = self.vel.y;
        let c = (a*a)+(b*b).sqrt();
        let current_degree = self.orientation();
        let new_degree = current_degree + 18.;
        if far { let new_degree = new_degree + 18.; }
        self.vel = vec2(new_degree.cos()*c, new_degree.sin()*c)
    }
    
    
    
    pub fn check_dead_mut(&mut self) {
        if self.age > 1000 || self.hp <= 0. {
            self.dead = true;
        }
    }    
}

impl Things {
    // Moving functions
    fn trig_calculator(&self, i: Things) -> (f32,f32,f32,f32) {
        let x = self.pos.x - i.pos.x;
        let y = self.pos.y - i.pos.y;
        let r = (x*x + y*y).sqrt();
        let theta = Self::degree_finder(x, y);
        (x,y,r,theta)
    }

    fn food_direction_convert(&self, temp_holder: Vec<Things>, amountFood: i32) -> Vec<(char, f32)> {
        let mut output: Vec<(char, f32)> = Vec::new();
        let mut q = 0;
        for i in temp_holder {
            let (_x,_y,r,theta) = self.trig_calculator(i);
            // let (u,r,s,l,b,w) = (45.0,63.0,81.0,99.0,117.0,135.0);
            let weight = 1./r;
            let mut direction: char = 's';
            match theta { // u=far right, r=right, s=straight, l=left, b=far left
                45.0..=63. => direction = 'u',
                63.0..=81. => direction = 'r',
                81.0..=99.0 => direction = 's',
                99.0..=117.0 => direction = 'l',
                117.0..=135.0 => direction = 'b',
                _=> println!("Error with direction fn for food"),
            }
            if q < amountFood { let weight = weight * 4.; }
            output.push((direction, weight));
            q+=1;
        }
        output
    }
    pub fn move_to_food(&mut self, Raw_food: Vec<Things>, Pher_f: Vec<Things>) {
        let mut tempFoodHolder = Vec::new();
        let mut foodCount = 0;
        
        for i in Raw_food {
            if Self::in_detect_range_check(&self, i.pos) {
                tempFoodHolder.push(i.clone());
                foodCount = foodCount + 1;
            }
        }

        for i in Pher_f {
            if Self::in_detect_range_check(&self, i.pos) {
                tempFoodHolder.push(i.clone());
            }
        }

        let new_vec: Vec<(char, f32)> = Self::food_direction_convert(&self, tempFoodHolder, foodCount);
        
        let (
                mut amountFarLeft, 
                mut amountLeft, 
                mut amountStraight, 
                mut amountRight, 
                mut amountFarRight
            ) = 
                (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());

        for &(direction, value) in &new_vec {
            match direction {
                'b' => amountFarLeft.push((direction, value)),
                'l' => amountLeft.push((direction, value)),
                's' => amountStraight.push((direction, value)),
                'r' => amountRight.push((direction, value)),
                'u' => amountFarRight.push((direction, value)),
                _ => println!("error when sorting the directions of food travel")
            }
        }

        let (mut numberFarLeft, mut numberLeft, mut numberStraight, mut numberRight, mut numberFarRight) = (0.,0.,0.,0.,0.,);

        amountFarLeft.iter().for_each(|x| numberFarLeft += x.1);
        amountLeft.iter().for_each(|x| numberLeft += x.1);
        amountStraight.iter().for_each(|x| numberStraight += x.1);
        amountRight.iter().for_each(|x| numberRight += x.1);
        amountFarRight.iter().for_each(|x| numberFarRight += x.1);

        numberFarLeft += rand::gen_range(0., 10.);
        numberFarRight += rand::gen_range(0., 10.);
        numberLeft += rand::gen_range(0., 10.);
        numberRight += rand::gen_range(0., 10.);
        numberStraight += rand::gen_range(0., 10.);

        let (farLeft, left, straight, right, farRight) = (
            numberFarLeft/amountFarLeft.len() as f32,
            numberLeft/amountLeft.len() as f32,
            numberStraight/amountStraight.len() as f32,
            numberRight/amountRight.len() as f32,
            numberFarRight/amountFarRight.len() as f32,
        );

        let best = farLeft.max(farRight).max(left).max(right).max(straight);
        match best {
            straight => self.pos += self.vel,
            farRight => {self.turn_right(true)},
            farLeft => {self.turn_left(true)},
            right => {self.turn_right(false)},
            left => {self.turn_left(false)},

            _=> println!("error at the turning end lol")
        }
    }
    fn pher_direction_convert(&self, temp_holder: Vec<Things>) -> Vec<(char, f32)>{
        let mut output: Vec<(char, f32)> = Vec::new();
        for i in temp_holder {
            let (_x,_y,r,theta) = self.trig_calculator(i);
            let weight = 1./r;
            let mut direction: char = 's';
            match theta { // u=far right, r=right, s=straight, l=left, b=far left
                45.0..=63. => direction = 'u',
                63.0..=81. => direction = 'r',
                81.0..=99.0 => direction = 's',
                99.0..=117.0 => direction = 'l',
                117.0..=135.0 => direction = 'b',
                _=> println!("Error with direction fn for food"),
            }
            output.push((direction, weight));
        }
        output
    }
    pub fn move_to_home(&mut self, pher: Vec<Things>) {
        let mut temp_holder = Vec::new(); 
        for i in pher {
            if Self::in_detect_range_check(&self, i.pos) {
                temp_holder.push(i.clone());
            }
        }

        let new_vec: Vec<(char, f32)> = self.pher_direction_convert(temp_holder);
        
        let (
                mut amountFarLeft, 
                mut amountLeft, 
                mut amountStraight, 
                mut amountRight, 
                mut amountFarRight
            ) = 
                (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());

        for &(direction, value) in &new_vec {
            match direction {
                'b' => amountFarLeft.push((direction, value)),
                'l' => amountLeft.push((direction, value)),
                's' => amountStraight.push((direction, value)),
                'r' => amountRight.push((direction, value)),
                'u' => amountFarRight.push((direction, value)),
                _ => println!("error when sorting the directions of food travel")
            }
        }

        let (mut numberFarLeft, mut numberLeft, mut numberStraight, mut numberRight, mut numberFarRight) = (0.,0.,0.,0.,0.,);

        amountFarLeft.iter().for_each(|x| numberFarLeft += x.1);
        amountLeft.iter().for_each(|x| numberLeft += x.1);
        amountStraight.iter().for_each(|x| numberStraight += x.1);
        amountRight.iter().for_each(|x| numberRight += x.1);
        amountFarRight.iter().for_each(|x| numberFarRight += x.1);

        numberFarLeft += rand::gen_range(0., 10.);
        numberFarRight += rand::gen_range(0., 10.);
        numberLeft += rand::gen_range(0., 10.);
        numberRight += rand::gen_range(0., 10.);
        numberStraight += rand::gen_range(0., 10.);

        let (far_left, left, straight, right, far_right) = (
            numberFarLeft/amountFarLeft.len() as f32,
            numberLeft/amountLeft.len() as f32,
            numberStraight/amountStraight.len() as f32,
            numberRight/amountRight.len() as f32,
            numberFarRight/amountFarRight.len() as f32,
        );

        let best = far_left.max(far_right).max(left).max(right).max(straight);
        match best {
            straight => self.pos += self.vel,
            far_right => {self.turn_right(true)},
            far_left => {self.turn_left(true)},
            right => {self.turn_right(false)},
            left => {self.turn_left(false)},

            _=> println!("error at the turning end lol")
        }
    }    


}

impl Things {
    // Testing fn
    pub fn test_move(&mut self, speed: Vec2) {
        self.pos += speed;
    }

}  


