// target area: x=248..285, y=-85..-56

fn main() {
    let area = Area {
        x1: 248,
        x2: 285,
        y1: -85,
        y2: -56,
    };

    first(&area);
    second(&area);
}

#[derive(Copy, Clone, Debug)]
struct Area {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

#[derive(Copy, Clone, Debug)]

struct Probe {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
    max_height: i32,
}

impl Probe {
    fn step(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;

        self.vel_x = if self.vel_x > 0 {
            self.vel_x - 1
        } else if self.vel_x < 0 {
            self.vel_x + 1
        } else {
            0
        };

        self.vel_y -= 1;

        self.max_height = self.max_height.max(self.y);
    }
}

fn is_in_area(p: &Probe, area: &Area) -> bool {
    return area.x1 <= p.x && area.x2 >= p.x && area.y1 <= p.y && area.y2 >= p.y;
}

fn did_miss(p: &Probe, area: &Area) -> bool {
    if p.vel_x == 0 && (p.x < area.x1 || p.x > area.x2) {
        return true;
    }

    if p.y < area.y1 {
        return true;
    }

    return false;
}

fn find_potential_pos(area: &Area) -> Vec<Probe> {
    let mut potential: Vec<Probe> = Vec::new();

    // make some guesses about the bounds
    for x in 0..300 {
        for y in -150..150 {
            let mut p = Probe {
                x: 0,
                y: 0,
                vel_x: x,
                vel_y: y,
                max_height: 0,
            };

            while !did_miss(&p, area) {
                if is_in_area(&p, area) {
                    potential.push(p);
                    break;
                }

                p.step();
            }
        }
    }

    return potential;
}

fn first(area: &Area) -> u32 {
    let potential = find_potential_pos(area);

    let max_y = potential.iter().map(|p| p.max_height).max().unwrap();

    println!("{:?}", max_y);
    return 0;
}

fn second(area: &Area) -> u32 {
    let potential = find_potential_pos(area);

    println!("{:?}", potential.len());
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::{first, second, Area};

    #[test]
    fn test() {
        let test_area = Area {
            x1: 20,
            x2: 30,
            y1: -10,
            y2: -5,
        };

        assert_eq!(first(&test_area), 0);
        assert_eq!(second(&test_area), 0);
    }
}
