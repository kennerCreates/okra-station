use bevy::prelude::*;
use std::vec;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use rand::seq::IndexedRandom;

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

#[derive(Resource)]
struct Grid {
    cells: Vec<Vec<bool>>,
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
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Grid {
            cells: generate_maze(21,21),
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, grid: Res<Grid>){
    commands.spawn(Camera2d);
    let start = Point { x: 1, y: 1 };
    let goal = Point { x: 19, y: 19 };
    if let Some(path) = astar(start, goal, &grid.cells){
    for (y, row) in grid.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let x_position = x as f32 * 10.0;
                let y_position = -(y as f32) * 10.0;
                let point = Point{x, y };
                
                if path.contains(&point) {
                    commands.spawn((
                        Sprite{
                            color: Color::srgb(0.75,0.0,0.05),
                            custom_size: Some(Vec2::new(10.0, 10.0)),
                            ..default()
                        },
                        Transform::from_xyz(x_position, y_position, 0.0),
                        ));
                } else if  *cell {
                    commands.spawn((
                        Sprite{
                            color: Color::srgb(0.0,0.8,0.2),
                            custom_size: Some(Vec2::new(10.0, 10.0)),
                            ..default()
                        },
                        Transform::from_xyz(x_position, y_position, 0.0),
                    ));
                }else {
                    commands.spawn((
                        Sprite{
                            color: Color::srgb(0.85,0.85,0.85),
                            custom_size: Some(Vec2::new(10.0, 10.0)),
                            ..default()
                        },
                        Transform::from_xyz(x_position, y_position, 0.0),
                    ));
                }   
            }
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

fn generate_maze(width: usize, height: usize) -> Vec<Vec<bool>>{
    let mut grid = vec![vec![true; width]; height];
    let mut stack: Vec<Point> = Vec::new();
    //carve starting cell
    grid[1][1] = false;
    stack.push(Point {x:1, y:1});
    //main loop
    while let Some(&current) = stack.last(){
        let neighbors: Vec<Point> = unvisited_neighbors(&current, &grid);
        if neighbors.len() > 0 {
            let mut rng = rand::rng();
            let new_point = neighbors.choose(&mut rng).unwrap();
            let mid_point = Point {
                x:(current.x + new_point.x)/2,
                y:(current.y + new_point.y)/2,
            };
            grid[new_point.y][new_point.x] = false;
            grid[mid_point.y][mid_point.x] = false;
            stack.push(*new_point);
        }
        else{
            //stack pop and backtrack
            stack.pop();
        } 
    }
    grid
}

fn unvisited_neighbors(point: &Point, grid: &Vec<Vec<bool>>) -> Vec<Point> {
    let mut result = Vec::new();
    let directions: [(i32, i32); 4] = [
        (0,-2), //up
        (0,2), //down
        (-2,0), //left
        (2, 0), //right        
    ];

    for (dx, dy) in directions{
        let nx = point.x as i32 + dx;
        let ny = point.y as i32 + dy;

        if nx < 0 || ny < 0{
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        //in bounds AND still a wall(unvisited)
        if ny < grid.len() && nx < grid[0].len() && grid [ny][nx]{
            result.push(Point {x: nx, y: ny});
        }
    }
    result
}
