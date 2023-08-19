use std::{default};
use macroquad::{
    miniquad::{gl::PFNGLCOMPRESSEDTEXIMAGE1DPROC, native::apple::frameworks::Object},
    prelude::*, rand,
};


fn modulo<T>(a: T, b: T) -> T where T: std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy, {((a % b) + b) % b} // calculate modulus operations
/* possible ideas/additions to add later:

  */
#[derive(Clone)]
pub struct Things {
    // What colony it belongs to
    alligence: i32,
    // 'worker' 'queen' 'soldier' 'defender' 'scout' 'food' 'scent'
    otype: String,
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
        let posx = modulo(posx, screen_width());
        let posy = modulo(posy, screen_height());

        let mut q: Things = Things {
            alligence: 0,
            otype: "queen".to_string(),
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
            otype: "worker".to_string(),
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
            pher_t: 10.,
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
                otype: "food".to_string(),
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
                pher_t: 0.,
                pher_h: 0.,
                detect: 0.,
            };
            raw_food.push(f);
        }
        raw_food
    }
    fn convert_to_food(mut deadOnes: Vec<Things>) -> Vec<Things> { //changes dead ants into food
        let mut newFood = Vec::new();
        for mut i in deadOnes {
                i.alligence = 0;
                i.otype = "food".to_string();
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
                i.pher_t = 0.;
                i.pher_h = 0.;
                i.detect = 0.;
                i.mass *= 0.9;
                newFood.push(i);
            }
            newFood
        }
    fn siphon_food(&self, mass:f32) -> Things { //allows for spliting a large Raw-food bundle up for carrying. Used in pick_food()
            let f = Things {
                alligence: 0,
                otype: "food".to_string(),
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
                pher_t: 0.,
                pher_h: 0.,
                detect: 0.,
            };
            f
    }
    fn pick_food(mut self, mut foodPiece:Things) -> Things { //ants picks up specific food piece, makes a group of ant-food that can be split up when food is delivered
        let takenPieceSize = &self.strength - &self.mass;
        let smallEnough = foodPiece.mass < takenPieceSize;
        if smallEnough {
            foodPiece.pos = self.pos;
            foodPiece.vel = self.vel;
            self.mass += foodPiece.mass;
            return foodPiece;
        }
        foodPiece.mass = foodPiece.mass - takenPieceSize;
        let takenPiece = self.siphon_food(takenPieceSize);
        self.mass += takenPiece.mass;
        takenPiece

    }
}
impl Things {
    // pheromones?
    fn new_pher(&self) -> Vec<Things>{ //generates a new set of pheremones based on an ant or food piece;
        let mut new_h = Things{
            alligence: 0,
            otype: "scent".to_string(),
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
            pher_t: 0.,
            pos: vec2(rand::gen_range(0., self.pher_h) + self.pos.x, rand::gen_range(0., self.pher_h) + self.pos.y),
            pher_h: self.pher_h,
        };
        let mut new_f = Things{
            alligence: 0,
            otype: "scent".to_string(),
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
            pher_t: 0.,
            pos: vec2(rand::gen_range(0., self.pher_f) + self.pos.x, rand::gen_range(0., self.pher_f) + self.pos.y),
            pher_f: self.pher_f,
        };
        let mut new_d = Things{
            alligence: 0,
            otype: "scent".to_string(),
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
            pher_t: 0.,
            pos: vec2(rand::gen_range(0., self.pher_d) + self.pos.x, rand::gen_range(0., self.pher_d) + self.pos.y),
            pher_d: self.pher_d,
        };
        let mut new_t = Things{
            alligence: 0,
            otype: "scent".to_string(),
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
            pos: vec2(rand::gen_range(0., self.pher_t) + self.pos.x, rand::gen_range(0., self.pher_t) + self.pos.y),
            pher_t: self.pher_t,
        };
        let mut output = vec![new_d, new_f, new_h, new_t];
        output
    }
    fn disperse(mut All_scents: Vec<Things>) -> Vec<Things> { //disperses the pheremones given. 
        let mut new_phers = Vec::new();
        for i in 0..All_scents.len() {
            All_scents[i].pher_d *= 0.3;
            All_scents[i].pher_f *= 0.3;
            All_scents[i].pher_t *= 0.3;
            All_scents[i].pher_h *= 0.3;
            new_phers.append(&mut Self::new_pher(&All_scents[i]));
            new_phers.append(&mut Self::new_pher(&All_scents[i]));
            new_phers.append(&mut Self::new_pher(&All_scents[i]));
        }
        new_phers.retain(|i| i.pher_h < 0.2 && i.pher_t < 0.2 && i.pher_d < 0.2 && i.pher_f < 0.2);
        new_phers

        
    }
}
impl Things {
    // scout
    fn new_scout(&self) -> Things {
        let mut s: Things = Things {
            alligence: 0,
            otype: "scout".to_string(),
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
            otype: "soldier".to_string(),
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
            pher_t: 10.,
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
            otype: "defender".to_string(),
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
        let mut queen:Vec<Things> = Vec::new();
        let mut worker:Vec<Things> = Vec::new();
        let mut soldier:Vec<Things> = Vec::new();
        let mut defender:Vec<Things> = Vec::new();
        let mut scout:Vec<Things> = Vec::new();
        let mut food :Vec<Things> = Vec::new();
        let mut scent:Vec<Things> = Vec::new();
        for mut i in chaos{
            match i.otype.as_str() {
                "queen" => queen.push(i),
                "worker" => worker.push(i),
                "soldier" => soldier.push(i),
                "defender" => defender.push(i),
                "scout" => scout.push(i),
                "food" => food.push(i),
                "scent" => scent.push(i),
                _ => println!("faliure in the sorter fn"),
            }
        }
        (queen,worker,soldier,defender,scout,food,scent)
    }
    pub fn colorShaper(&self) {
        match self.otype.as_str() {
            "worker"=> draw_circle(self.pos.x, self.pos.y, self.mass, DARKBLUE),
            "queen"=> draw_circle(self.pos.x, self.pos.y, self.mass, GOLD),
            "soldier"=> draw_circle(self.pos.x, self.pos.y, self.mass, RED),
            "defender"=> draw_circle(self.pos.x, self.pos.y, self.mass, YELLOW),
            "scout"=> draw_circle(self.pos.x, self.pos.y, self.mass, SKYBLUE),
            "food"=> draw_circle(self.pos.x, self.pos.y, self.mass, ORANGE),
            "scent"=> if self.pher_t > 0. {
                draw_circle(self.pos.x, self.pos.y, self.pher_t, GRAY);
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
    fn food_direction_convert(&self, tempHolder: Vec<Things>, amountFood: i32) -> Vec<(char, f32)> {
        let mut output: Vec<(char, f32)> = Vec::new();
        let mut q = 0;
        for i in tempHolder {
            let x = self.pos.x - i.pos.x;
            let y = self.pos.y - i.pos.y;
            let r = (x*x + y*y).sqrt();
            let theta = Self::degree_finder(x, y);
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
    fn to_home_direction_convert(&self, tempHolder: Vec<Things>) -> Vec<(char, f32)>{
        let mut output: Vec<(char, f32)> = Vec::new();
        for i in tempHolder {
            let x = self.pos.x - i.pos.x;
            let y = self.pos.y - i.pos.y;
            let r = (x*x + y*y).sqrt();
            let theta = Self::degree_finder(x, y);
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
    pub fn move_to_home(&mut self, Pher_t: Vec<Things>) {
        let mut tempHolder = Vec::new(); 
        for i in Pher_t {
            if Self::in_detect_range_check(&self, i.pos) {
                tempHolder.push(i.clone());
            }
        }

        let new_vec: Vec<(char, f32)> = self.to_home_direction_convert(tempHolder);
        
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

}

