use std::{
    collections::{HashMap, HashSet},
    error::{self, Error},
    fs, result,
    str::FromStr,
    vec,
};

// Config
const INPUT_FILE: &str = "input.txt";
const MAX_SUM_DIST: usize = 10000;

type Result<T> = result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string(INPUT_FILE)?;
    let points = input_to_points(&input)?;
    let grid = Grid::new(points);

    //Part 1
    let largest_observable = grid.op_largest_observable();
    println!("Part 1: {}", largest_observable);
    
    //Part 2
    let central_mass = grid.op_central_mass();
    println!("Part 2: {}", central_mass);

    Ok(())
}

fn input_to_points(input: &str) -> Result<Vec<Point>> {
    let mut points = vec![];

    for line in input.lines() {
        points.push(Point::from_str(line)?);
    }

    Ok(points)
}

#[derive(Debug)]
struct Grid {
    // List of input points on outer edge of grid (infinite area)
    perimeter: HashSet<Point>,

    // All points on a grid, enclosing all input locations,
    // pointing to closest input location to it. EX:
    //  `area_map[0][0]` = input location closest to (0, 0) on grid`
    area_map: Vec<Vec<Point>>,

    // All points on a grid, enclosing all input locations,
    // indicating whether the point is within range of all input points
    // range is 10000 manhattan dist or less to all inputs
    range_map: Vec<Vec<bool>>,
}

impl Grid {
    fn new(input_locations: Vec<Point>) -> Self {
        // Calculate bounds for grid
        let (bound_x, bound_y) = Self::get_border(&input_locations);

        // Part 1: Find closest input location to each point on full grid,
        // recording perimeter points along the way.
        let mut closest_locations: Vec<Vec<Point>> = Vec::with_capacity(bound_x);
        let mut perimeter_points: HashSet<Point> = HashSet::new();
        
        // Part 2: Track points on grid that are within range of all input points
        let mut locations_in_range: Vec<Vec<bool>> = Vec::with_capacity(bound_x);
        
        for x in 0..=bound_x {
            let mut p1_row: Vec<Point> = Vec::with_capacity(bound_y);
            let mut p2_row: Vec<bool> = Vec::with_capacity(bound_y);

            for y in 0..=bound_y {
                let grid_point = Point { x, y };

                let mut min_dist = usize::MAX;
                let mut ptr = 0;

                let mut sum_dist = 0;
                
                // Check this point on grid against all input points:
                //  p1: for the closest input point to it
                //  p2: to sum up distances from this point to all inputs
                for (loc_ptr, loc) in input_locations.iter().enumerate() {
                    
                    let dist = Self::manhattan_dist(&grid_point, &loc);

                    // P1
                    if dist < min_dist {
                        ptr = loc_ptr;
                        min_dist = dist;
                    }

                    // P2
                    sum_dist += dist;
                }

                // Record an input loc as perimeter if bounding box is close to it
                if x == 0 || y == 0 || x == bound_x || y == bound_y {
                    perimeter_points.insert(input_locations[ptr].clone());
                }

                p1_row.push(input_locations[ptr].clone());
                p2_row.push(sum_dist < MAX_SUM_DIST);
            }

            closest_locations.push(p1_row);
            locations_in_range.push(p2_row);
        }

        // Create & return grid
        Grid {
            perimeter: perimeter_points,
            area_map: closest_locations,
            range_map: locations_in_range,
        }
    }

    fn op_largest_observable(&self) -> i32 {
        let mut finite_areas: HashMap<Point, i32> = HashMap::new();

        for row in &self.area_map {
            for closest_to in row {
                if self.perimeter.contains(&closest_to) {
                    continue;
                } else {
                    finite_areas
                        .entry(closest_to.clone())
                        .and_modify(|area| *area += 1)
                        .or_insert(1);
                }
            }
        }

        *finite_areas
            .iter()
            .max_by_key(|entry| entry.1)
            .expect("failed to find max area")
            .1
    }

    fn op_central_mass(&self) -> i32 {

        let mut area = 0;

        for row in &self.range_map {
            for in_range in row {
                if *in_range { area += 1; }
            }
        }

        area
    }

    fn get_border(points: &Vec<Point>) -> (usize, usize) {
        let max_x = points
            .iter()
            .max_by(|p1, p2| p1.x.cmp(&p2.x))
            .expect("failed to find max x")
            .x as usize;

        let max_y = points
            .iter()
            .max_by(|p1, p2| p1.y.cmp(&p2.y))
            .expect("failed to find max y")
            .y as usize;

        (max_x + 1, max_y + 1)
    }

    fn manhattan_dist(p1: &Point, p2: &Point) -> usize {
        p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let (x, y) = s.split_once(", ").expect("failed to split coordinates");

        let x_fromstr = x.parse::<usize>()?;
        let y_fromstr = y.parse::<usize>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}
