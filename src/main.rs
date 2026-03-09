use std::vec;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy)]
struct Node {
    point: Point,
    cost: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let grid = vec![
        vec![false, false, false, false, false],
        vec![false, true, true, true, false],
        vec![false, false, false, true, false],
        vec![false, true, false, false, false],
        vec![false, true,false, false, false],
    ];

    for row in &grid {
        for cell in row {
            if *cell {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    let start = Point { x: 0, y: 0 };
    let goal = Point { x: 4, y: 4 };
    
    match astar(start, goal, &grid){
        Some(path) => {
            println!("\nPath found:");
            for point in &path {
                println! ("({}, {})", point.x, point.y);
            }
        }
        None => println! ("No Path found"),
    }

    if let Some(path) = astar(start, goal, &grid) {
        println!("\nVisualized:");
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let point = Point{x, y };
                if point == start {
                    print!("S ");
                } else if point == goal {
                    print!("G ");
                } else if path.contains(&point) {
                    print!("* ");
                } else if *cell {
                    print!("# ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }
}

fn heuristic(a: Point, b: Point) -> f32 {
    let dx = (a.x as f32 - b.x as f32).abs();
    let dy = (a.y as f32 - b.y as f32).abs();
    dx + dy
}

fn neighbors (point: Point, grid: &Vec<Vec<bool>>) -> Vec<Point> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut result = Vec::new();
    let directions: [(i32, i32); 4] = [
        (0, -1), //up
        (0, 1), //down
        (-1, 0), //left
        (1, 0), //right
    ]; 

    for (dx, dy) in directions{
        let nx = point.x as i32 + dx;
        let ny = point.y as i32 + dy;

        //bounds check
        if nx < 0 || ny <0 {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if nx < cols && ny < rows && !grid[ny][nx] {
            result.push(Point{x: nx, y: ny});
        } 
    }
    result
}

fn astar(
    start: Point, 
    goal: Point, 
grid: &Vec<Vec<bool>>,
) -> Option<Vec<Point>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut g_score: HashMap<Point, f32> = HashMap::new();

    g_score.insert(start, 0.0);
    open_set.push(Node {
        point:start,
        cost: heuristic(start,goal),
    });

    while let Some(current) = open_set.pop() {
        if current.point == goal {
            //reconstruct path by walking back through came_from
            let mut path = vec![goal];
            let mut current_point = goal;

            while let Some(&prev) = came_from.get(&current_point) {
                path.push(prev);
                current_point = prev;
            }
            path.reverse();
            return Some(path);
        }
        
        for neighbor in neighbors(current.point, grid){
            let tentative_g = g_score[&current.point] + 1.0;

            if tentative_g < *g_score.get(&neighbor).unwrap_or(&f32::INFINITY){
                came_from.insert(neighbor, current.point);
                g_score.insert(neighbor, tentative_g);

                open_set.push(Node {
                    point: neighbor, 
                    cost: tentative_g + heuristic(neighbor, goal),
                });
            }
        }
    }
    None
}
