#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
struct Moon {
    pos: (isize, isize, isize),
    vel: (isize, isize, isize),
}

impl Moon {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Moon {
            pos: (x, y, z),
            vel: (0, 0, 0),
        }
    }

    fn total_energy(&self) -> isize {
        let pe = self.pos.0.abs() + 
        self.pos.1.abs() + 
        self.pos.2.abs();
        let ke = self.vel.0.abs() + 
        self.vel.1.abs() + 
        self.vel.2.abs();

        pe * ke
    }

    fn apply_gravity(&mut self, others: &[Moon]) {
        for other in others.iter() {
            if self.pos.0 < other.pos.0 {
                self.vel.0 += 1;
            }
            if self.pos.0 > other.pos.0 {
                self.vel.0 -= 1;
            }
            if self.pos.1 < other.pos.1 {
                self.vel.1 += 1;
            }
            if self.pos.1 > other.pos.1 {
                self.vel.1 -= 1;
            }
            if self.pos.2 < other.pos.2 {
                self.vel.2 += 1;
            }
            if self.pos.2 > other.pos.2 {
                self.vel.2 -= 1;
            }
        }
    }

    fn apply_velocity(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
        self.pos.2 += self.vel.2;
    }
}

const N: usize = 1000;

fn main() {
/*
<x=-1, y=-4, z=0>
<x=4, y=7, z=-1>
<x=-14, y=-10, z=9>
<x=1, y=2, z=17>
*/
    let mut a = Moon::new(-1,-4,0);
    let mut b = Moon::new(4, 7, -1);
    let mut c = Moon::new(-14,-10,9);
    let mut d = Moon::new(1, 2, 17);

    for _ in 0..N {
        a.apply_gravity(&[b,c,d]);
        b.apply_gravity(&[a,c,d]);
        c.apply_gravity(&[a,b,d]);
        d.apply_gravity(&[a,b,c]);
        a.apply_velocity();
        b.apply_velocity();
        c.apply_velocity();
        d.apply_velocity();
    }

    println!("{}", a.total_energy() + b.total_energy() + c.total_energy() + d.total_energy());

}
