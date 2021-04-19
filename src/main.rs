use core::f32;
use std::{cmp::Ordering, collections::{HashMap, BinaryHeap}, rc::Rc};
use image::{ImageBuffer};
use rand::Rng;
use colored::*;
use std::time::Instant;

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<bool>
}

type Position = (u32, u32);

#[derive(Debug, Clone)]
struct PathNode{
    pos: Position,
    f: f32,
    g: f32,
    parent: Option<Rc<PathNode>>
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos &&
        self.f == other.f &&
        self.g == other.g
    }
}

impl Eq for PathNode {
    
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.partial_cmp(&self.f).unwrap().then_with(|| self.pos.cmp(&other.pos))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn rand_map(width: usize, height: usize) -> Map{
    let mut rng = rand::thread_rng();

    let mut map = Map {
        height: width,
        width: height,
        tiles: vec![]
    };
    for _ in 0..(width * height) {
        map.tiles.push(rng.gen_bool(0.2));
    }
    map.tiles[0] = false;
    let last = map.tiles.len() - 1;
    map.tiles[last] = false;
    map
}

fn main() {
    println!("Hello!");
    // Test with random map
    let map1 = rand_map(1000, 1000);

    // println!("{}", visualize_map(&map1));

    let now = Instant::now();
    match a_star((0,0), (999,999), &map1, sl_distance) {
        Some(path) => save_path_on_map(&map1, &path),
        _ => println!("{}","No Path Found!".red())
    };
    println!("Ran A* on map1 in {}ms", now.elapsed().as_millis());

    let t = true;
    let f = false;
    // Test with fixed map
    let map2 = Map {
        width: 10,
        height: 10,
        tiles: vec![
            f, f, f, f, f, f, f, f, f, f,
            f, f, f, f, f, f, f, f, f, f,
            f, f, f, f, f, f, t, f, f, f,
            f, f, f, f, f, t, t, f, f, f,
            f, f, f, f, f, t, f, f, f, f,
            f, f, f, f, t, t, f, f, f, f,
            f, f, f, f, t, f, f, f, f, f,
            f, f, f, f, f, f, f, f, f, f,
            f, f, f, f, f, f, f, f, f, f,
            f, f, f, f, f, f, f, f, f, f,
        ]
    };

    println!("{}", visualize_map(&map2));

    
    match a_star((0,0), (9,9), &map2, sl_distance) {
        Some(path) => println!("{}", visualize_path_on_map(&map2, &path)),
        _ => println!("{}","No Path Found!".red())
    };

    // Run performance test
    let now = Instant::now();
    let iterations = 10000;
    for _ in 0..iterations {
        a_star((0,0), (9,9), &map2, sl_distance);
    }
    println!("Ran A* on map2 {} times in {}ms", iterations, now.elapsed().as_millis());
}

fn visualize_map(map: &Map) -> String {
    let mut output = String::from("");
    let tiles_copy: Vec<String> = map.tiles.clone().into_iter().map(|v| {
        match v {
            true => "#".blue().to_string(),
            false => String::from("0")
        }
    }).collect();

    for y in 0..map.height {
        for x in 0..map.width {
            output.push_str(tiles_copy[x + y * map.width].as_str());
            output.push(' ');
        }
        output.push('\n');
    }

    return output;
}

fn visualize_path_on_map(map: &Map, path: &Vec<Position>) -> String {
    let mut output = String::from("");
    let mut tiles_copy: Vec<String> = map.tiles.clone().into_iter().map(|v| {
        match v {
            true => "#".blue().to_string(),
            false => String::from("0")
        }
    }).collect();

    for p in path {
        tiles_copy[(p.0 + p.1 * map.width as u32) as usize] = "X".red().to_string();
    }

    for y in 0..map.height {
        for x in 0..map.width {
            output.push_str(tiles_copy[x + y * map.width].as_str());
            output.push(' ');
        }
        output.push('\n');
    }

    return output;
}

fn save_path_on_map(map: &Map, path: &Vec<Position>) {
    let mut tiles_copy: Vec<u32> = map.tiles.clone().into_iter().map(|v| {
        match v {
            true => 1,
            false => 0
        }
    }).collect();

    for p in path {
        tiles_copy[(p.0 + p.1 * map.width as u32) as usize] = 3;
    }

    // a default (black) image containing Rgb values
    let mut img: image::RgbImage = ImageBuffer::new(map.width as u32, map.height as u32);
    for x in 0..map.width as u32 {
        for y in 0..map.width as u32 {
            *img.get_pixel_mut(x, y) = match tiles_copy[x as usize + y as usize * map.height] {
                0 => image::Rgb([255,255,255]),
                1 => image::Rgb([0,0,255]),
                _ => image::Rgb([255,0,0])
            };
        }
    }

    img.save("output.png").unwrap();
}



// fn manhattan_distance(a: Position, b: Position) -> f32{
//     (i32::abs((a.0 as i32) - (b.0 as  i32)) + i32::abs((a.1 as i32) - (b.1 as i32))) as f32
// }

fn sl_distance(a: Position, b: Position) -> f32{
    f32::sqrt(((a.0 as i32) - (b.0 as  i32)).pow(2) as f32 + 
              ((a.1 as i32) - (b.1 as i32)).pow(2) as f32)
}

fn find_smallest_f<'a>(nodes: &'a HashMap<Position, PathNode>) -> (u32, u32) {
    let min = nodes.into_iter()
         .min_by(|a,b| a.1.f.partial_cmp(&b.1.f).expect("Nan!!!"))
         .clone().expect("There was no min!");
    min.0.clone()
}

fn backtrace_path(path: &Vec<Rc<PathNode>>) -> Vec<Position>{
    let mut out = match path.last() {
        Some(x) => {
            let mut out: Vec<Position> = vec![];
            let mut node = x.clone();
            loop {
                out.push(node.pos);
                match node.parent.clone() {
                    Some(x) => node = x,
                    _ => break
                };
            }
            out.push(node.pos);
            out
        },
        _ => vec![]
    };
    out.reverse();
    out
}

fn a_star(start: Position, end: Position, map: &Map, dist: fn(Position, Position) -> f32) -> Option<Vec<Position>> {
    let mut open_set: BinaryHeap<PathNode> = BinaryHeap::new();
    let mut open_set_contains: HashMap<Position, PathNode> = HashMap::new();

    let start_node = PathNode {pos: start, f: 0.0, g:0.0, parent: None };
    open_set.push(start_node.clone());
    open_set_contains.insert(start, start_node);

    let mut came_from: Vec<Rc<PathNode>> = vec![];

    loop {
        if open_set.len() == 0{
            return None
        }
        
        let temp_q = open_set.pop().unwrap();
        // println!("pos: ({}, {})", temp_q.pos.0, temp_q.pos.1);
        open_set_contains.remove_entry(&temp_q.pos);
        
        let q = Rc::from(temp_q);
        
        let mut next_nodes: Vec<PathNode> = vec![];
        for x in -1..=1_i32 {

            if (q.pos.0 == 0 && x == -1) || (q.pos.0 as i32 + x) >= map.width as i32 {
                continue;
            }
            let nx = (q.pos.0 as i32 + x) as u32;

            for y in -1..=1_i32 {
                if (q.pos.1 == 0 && y == -1) || (q.pos.1 as i32 + y) >= map.height as i32  {
                    continue;
                }
                if x == 0 && y == 0 {
                    continue;
                }

                let ny = (q.pos.1 as i32 + y) as u32;

                if map.tiles[(nx + ny * map.width as u32) as usize] == true {
                    continue;
                }
                
                let n_pos = (nx,ny);
                let g = q.g + dist(n_pos, q.pos) as f32;
                let h = dist(n_pos, end) * 2 as f32;
                next_nodes.push(PathNode {
                    pos: n_pos,
                    f: g+h,
                    g,
                    parent: Some(q.clone())
                });
            }
        }
        let mut hit_end = false;
        while let Some(c) = next_nodes.pop() {
            if c.pos == end{
                hit_end = true;
                break;
            }

            // Check if there isn't the same position but with better f
            match open_set_contains.get(&c.pos) {
                Some(x) => if x.f < c.f {continue;}
                _ => ()
            };

            // Check to see if we have already visited it, but there is a better value
            match (&came_from).into_iter().find(|x| x.pos == c.pos) {
                Some(x) => {
                    if x.f < c.f {continue;}
                }
                _ => ()
            };

            open_set_contains.insert(c.pos, c.clone());
            open_set.push( c);

        }
        came_from.push(q.clone());
        if hit_end {
            came_from.push(Rc::from(PathNode {
                f: 0.0,
                g: 0.0,
                pos: end,
                parent: Some(came_from.last().expect("").clone())
            }));
            break;
        };
    }


    Some(backtrace_path(&came_from))
}