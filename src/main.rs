use core::f32;
use std::{collections::HashMap, rc::Rc};
use rand::Rng;
use colored::*;

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<bool>
}

type Position = (u32, u32);

struct PathNode{
    pos: Position,
    f: f32,
    g: f32,
    parent: Option<Rc<PathNode>>
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
    // Test with random map
    let map1 = rand_map(10, 10);

    println!("{}", visualize_map(&map1));

    
    match a_star((0,0), (9,9), &map1, sl_distance) {
        Some(path) => println!("{}", visualize_path_on_map(&map1, &path)),
        _ => println!("{}","No Path Found!".red())
    };
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
    let mut open_set: HashMap<Position, PathNode> = HashMap::new();
    let start_node = PathNode {pos: start, f: 0.0, g:0.0, parent: None };
    open_set.insert(start, start_node);
    let mut came_from: Vec<Rc<PathNode>> = vec![];

    loop {
        if open_set.len() == 0{
            return None
        }

        let smallest = find_smallest_f(&open_set);
        let temp_q = open_set.remove(&smallest).expect("This should never happen!");
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
                let h = dist(n_pos, end) as f32;
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
            match open_set.get(&c.pos) {
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

            open_set.insert(c.pos, c);

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

