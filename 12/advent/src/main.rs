#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Moon {
    x: isize,
    y: isize,
    z: isize,
    vx: isize,
    vy: isize,
    vz: isize,
}

impl Moon {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Moon {
            x,
            y,
            z,
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }

    fn apply_gravity(&mut self, other: &mut Moon) {
        if self.x < other.x {
            self.vx += 1;
            other.vx -= 1;
        } else if self.x > other.x {
            self.vx -= 1;
            other.vx += 1;
        }

        if self.y < other.y {
            self.vy += 1;
            other.vy -= 1;
        } else if self.y > other.y {
            self.vy -= 1;
            other.vy += 1;
        }

        if self.z < other.z {
            self.vz += 1;
            other.vz -= 1;
        } else if self.z > other.z {
            self.vz -= 1;
            other.vz += 1;
        }
    }

    #[inline]
    fn apply_velocity(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }
}

fn lcm(a: usize, b: usize, c: usize) -> usize {
    let top = a * b * c;
    let x1 = a * b;
    let x2 = b * c;
    let x3 = a * c;
    let gcd1 = gcd(x1, x2);
    let bottom = gcd(gcd1, x3);
    top / bottom
}

fn gcd(a: usize, b: usize) -> usize {
    if a > b {
        return gcd(b, a - b);
    }
    if b > a {
        return gcd(a, b - a);
    }
    return a;
}

fn main() {
    /*
    <x=-1, y=-4, z=0>
    <x=4, y=7, z=-1>
    <x=-14, y=-10, z=9>
    <x=1, y=2, z=17>
    */
    let a = Moon::new(-1, -4, 0);
    let b = Moon::new(4, 7, -1);
    let c = Moon::new(-14, -10, 9);
    let d = Moon::new(1, 2, 17);

    let x_period = find_period(a.clone(), b.clone(), c.clone(), d.clone(), Dimension::X);
    let y_period = find_period(a.clone(), b.clone(), c.clone(), d.clone(), Dimension::Y);
    let z_period = find_period(a.clone(), b.clone(), c.clone(), d.clone(), Dimension::Z);

    println!("{}, {}, {}", x_period, y_period, z_period);
    println!("{}", lcm(x_period, y_period, z_period));
}

enum Dimension {
    X,
    Y,
    Z,
}

fn find_period(mut a: Moon, mut b: Moon, mut c: Moon, mut d: Moon, dimension: Dimension) -> usize {
    let mut seen = HashSet::new();
    let mut counter = 0;
    loop {
        let key = match dimension {
            Dimension::X => (a.x, a.vx, b.x, b.vx, c.x, c.vx, d.x, d.vx),
            Dimension::Y => (a.y, a.vy, b.y, b.vy, c.y, c.vy, d.y, d.vy),
            Dimension::Z => (a.z, a.vz, b.z, b.vz, c.z, c.vz, d.z, d.vz),
        };
        if !seen.insert(key) {
            break;
        }
        counter += 1;

        a.apply_gravity(&mut b);
        a.apply_gravity(&mut c);
        a.apply_gravity(&mut d);
        b.apply_gravity(&mut c);
        b.apply_gravity(&mut d);
        c.apply_gravity(&mut d);

        a.apply_velocity();
        b.apply_velocity();
        c.apply_velocity();
        d.apply_velocity();
    }
    counter
}
