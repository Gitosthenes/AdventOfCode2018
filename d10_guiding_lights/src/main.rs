use std::collections::HashMap;
use std::error::Error;
use std::{fs, result};

type Result<T> = result::Result<T, Box<dyn Error>>;
type LightMap = HashMap<(i32, i32), Vec<(i32, i32)>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("test_input.txt")?;
    let mut sky = Sky::from_input(&input)?;

    println!("{}", sky.to_string());
    sky.next_second();
    println!("{}", sky.to_string());
    sky.next_second();
    println!("{}", sky.to_string());
    sky.next_second();
    println!("{}", sky.to_string());
    sky.next_second();
    println!("{}", sky.to_string());

    Ok(())
}

struct Sky {
    bounds: (i32, i32, i32, i32),
    lights: LightMap,
}

impl Sky {
    fn from_input(input: &str) -> Result<Self> {
        let rgx = regex::Regex::new(
            r"<\s*(?P<px>-?\d+),\s+(?P<py>-?\d+)>.+<\s*(?P<vx>-?\d),\s+(?P<vy>-?\d)>",
        )
        .unwrap();

        // Can't use i32::MAX or i32::MIN using IntelliJ plugin:
        // https://github.com/intellij-rust/intellij-rust/issues/10483
        let (mut min_x, mut min_y, mut max_x, mut max_y) =
            (2147483647i32, 2147483647i32, -2147483648i32, -2147483648i32);

        let mut map: LightMap = HashMap::new();

        // Capture initial light positions and sky bounds
        for line in input.lines() {
            let caps = rgx.captures(line).unwrap();

            let (px, py, vx, vy) = (
                caps["px"].parse::<i32>()?,
                caps["py"].parse::<i32>()?,
                caps["vx"].parse::<i32>()?,
                caps["vy"].parse::<i32>()?,
            );

            min_x = if px < min_x { px } else { min_x };
            min_y = if py < min_y { py } else { min_y };
            max_x = if px > max_x { px } else { max_x };
            max_y = if py > max_y { py } else { max_y };

            map.entry((px, py))
                .and_modify(|points| points.push((vx, vy)))
                .or_insert(vec![(vx, vy)]);
        }

        Ok(Sky {
            bounds: (min_x, min_y, max_x, max_y),
            lights: map,
        })
    }

    fn next_second(&mut self) {
        let mut updated_lights: LightMap = HashMap::new();
        let (min_x, min_y, max_x, max_y) = self.bounds;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if let Some(points) = self.lights.get(&(x, y)) {
                    for p in points {
                        let new_x = x + p.0;
                        let new_y = y + p.1;

                        //Check bounds increased
                        self.bounds.0 = if new_x < min_x { new_x } else { min_x };
                        self.bounds.1 = if new_y < min_y { new_y } else { min_y };
                        self.bounds.2 = if new_x > max_x { new_x } else { max_x };
                        self.bounds.3 = if new_y > max_y { new_y } else { max_y };

                        //Add point to new position in updated light map
                        updated_lights
                            .entry((new_x, new_y))
                            .and_modify(|points| points.push(*p))
                            .or_insert(vec![(*p)]);
                    }
                }
            }
        }

        // Update lights in sky
        self.lights = updated_lights;
    }
}

impl ToString for Sky {
    fn to_string(&self) -> String {
        let mut s = String::new();

        let (min_x, min_y, max_x, max_y) = self.bounds;
        s.push_str(&format!(
            "x: [{}, {}]; y: [{}, {}]\n",
            min_x, max_x, min_y, max_y
        ));

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                match self.lights.get(&(x, y)) {
                    Some(_) => s.push('#'),
                    None => s.push('.'),
                }
            }

            s.push('\n');
        }

        s
    }
}
