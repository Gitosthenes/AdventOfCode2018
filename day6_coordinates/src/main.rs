use std::{
    collections::{HashMap, HashSet},
    error, fs,
    num::ParseIntError,
    result,
    str::FromStr,
};

type Result<T> = result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("test_input.txt")?;

    part1(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let points = input_to_points(&input)?;

    // TESTING ----------------------------------------------------
    let mut named_points: HashMap<Point, String> = HashMap::new();

    for (idx, name) in "ABCDEF".chars().enumerate() {
        named_points.insert(points[idx].clone(), name.to_string());
    }
    //--------------------------------------------------------------

    let perimeter = get_perimeter(&points)?;

    for point in perimeter {
        print!("{}, ", named_points[&point]);
    }

    Ok(())
}

fn get_perimeter(points: &Vec<Point>) -> Result<HashSet<Point>> {
    let mut set: HashSet<Point> = HashSet::new();

    let mut far_x = -1000;
    let mut far_y = -1000;

    let mut closest_point = &points[0];
    let mut min_dist = i32::MAX;

    //Top
    for x in -1000..=1000 {
        far_x = x;
        let far_point = Point { x: far_x, y: far_y };

        for point in points {
            let dist = manhattan_dist(point, &far_point);

            if dist < min_dist {
                closest_point = point;
                min_dist = dist;
            }
        }

        set.insert(closest_point.clone());
    }

    //Right
    for y in -1000..=1000 {
        far_y = y;
        let far_point = Point { x: far_x, y: far_y };

        for point in points {
            let dist = manhattan_dist(point, &far_point);

            if dist < min_dist {
                closest_point = point;
                min_dist = dist;
            }
        }

        set.insert(closest_point.clone());
    }

    //Bottom
    for x in (-1000..=1000).rev() {
        far_x = x;
        let far_point = Point { x: far_x, y: far_y };

        for point in points {
            let dist = manhattan_dist(point, &far_point);

            if dist < min_dist {
                closest_point = point;
                min_dist = dist;
            }
        }

        set.insert(closest_point.clone());
    }

    //Left
    for y in (-1000..=1000).rev() {
        far_y = y;
        let far_point = Point { x: far_x, y: far_y };

        for point in points {
            let dist = manhattan_dist(point, &far_point);

            if dist < min_dist {
                closest_point = point;
                min_dist = dist;
            }
        }

        set.insert(closest_point.clone());
    }

    Ok(set)
}

fn manhattan_dist(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn input_to_points(input: &str) -> Result<Vec<Point>> {
    let mut points = vec![];

    for line in input.lines() {
        points.push(Point::from_str(line)?);
    }

    Ok(points)
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let (x, y) = s.split_once(", ").expect("failed to split coordinates");

        let x_fromstr = x.parse::<i32>()?;
        let y_fromstr = y.parse::<i32>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}
