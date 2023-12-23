use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use std::str::Lines;
use std::usize;
use three_d::*;

type BrickID = usize;
type Point3D = (usize, usize, usize);

fn color_palette() -> Vec<(u8, u8, u8)> {
    vec![
        (255, 0, 0),   // Red
        (0, 255, 0),   // Green
        (0, 0, 255),   // Blue
        (255, 255, 0), // Yellow
        (255, 0, 255), // Magenta
        (0, 255, 255), // Cyan
        (128, 0, 128), // Purple
        (255, 165, 0), // Orange
        (0, 128, 0),   // Dark Green
        (0, 0, 128),   // Dark Blue
    ]
}

fn render(brick_points: &HashMap<Point3D, BrickID>, connections: &HashMap<Point3D, Point3D>) {
    let window = Window::new(WindowSettings {
        title: "Shapes!".to_string(),
        max_size: Some((600, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(5.0, 2.0, 2.5),
        vec3(0.0, 0.0, -0.5),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);
    let light = AmbientLight::new(&context, 0.7, Srgba::WHITE);

    let axes = Axes::new(&context, 0.05, 20.0);

    let colors = color_palette();
    let mut shapes: Vec<Gm<Mesh, PhysicalMaterial>> = Vec::new();
    for (&(x, y, z), brick_id) in brick_points.iter() {
        let mut cube = Gm::new(
            Mesh::new(&context, &CpuMesh::cube()),
            PhysicalMaterial::new_transparent(
                &context,
                &CpuMaterial {
                    albedo: Srgba {
                        r: colors[brick_id % colors.len()].0,
                        g: colors[brick_id % colors.len()].1,
                        b: colors[brick_id % colors.len()].2,
                        a: 150,
                    },
                    ..Default::default()
                },
            ),
        );
        cube.set_transformation(
            Mat4::from_translation(vec3(x as f32, z as f32, y as f32)) * Mat4::from_scale(0.5),
        );
        shapes.push(cube);

        let mut sphere = Gm::new(
            Mesh::new(&context, &CpuMesh::sphere(2)),
            PhysicalMaterial::new_transparent(
                &context,
                &CpuMaterial {
                    albedo: Srgba::BLACK,
                    ..Default::default()
                },
            ),
        );
        sphere.set_transformation(
            Mat4::from_translation(vec3(x as f32, z as f32, y as f32)) * Mat4::from_scale(0.17),
        );
        shapes.push(sphere);
    }

    for (from, &to) in connections.iter() {
        let brick_id = brick_points.get(from).unwrap();
        let mut connection = Gm::new(
            Mesh::new(&context, &CpuMesh::arrow(10.0, 0.3, 5)),
            PhysicalMaterial::new_transparent(
                &context,
                &CpuMaterial {
                    albedo: Srgba {
                        r: colors[brick_id % colors.len()].0,
                        g: colors[brick_id % colors.len()].1,
                        b: colors[brick_id % colors.len()].2,
                        a: 200,
                    },
                    ..Default::default()
                },
            ),
        );
        connection.set_transformation(
            Mat4::from_translation(vec3(to.0 as f32, to.2 as f32, to.1 as f32))
                * Mat4::from_scale(0.1 as f32)
                * Mat4::from_angle_z(Rad(PI / 2.0))
                * Mat4::from_nonuniform_scale((from.2 - to.2) as f32, 1.0, 1.0),
        );
        shapes.push(connection);
    }

    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &camera,
                axes.into_iter()
                    .chain(shapes.iter().map(|c| c as &dyn Object)),
                &[&light],
            );

        FrameOutput::default()
    });
}

fn parse_brick(s: &str) -> Point3D {
    match s
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()[..]
    {
        [x, y, z] => (x, y, z),
        _ => panic!("Invalid brick {}", s),
    }
}

fn parse_bricks(lines: Lines) -> Vec<(Point3D, Point3D)> {
    lines
        .map(|line| match line.split("~").collect::<Vec<&str>>()[..] {
            [from, to] => (parse_brick(from), parse_brick(to)),
            _ => panic!("Invalid line {:?}", line),
        })
        .collect()
}

fn brick_points(bricks: &Vec<(Point3D, Point3D)>) -> HashMap<Point3D, BrickID> {
    bricks
        .iter()
        .enumerate()
        .flat_map(|(brick_id, (brick_from, brick_to))| {
            (brick_from.0..=brick_to.0).flat_map(move |x| {
                (brick_from.1..=brick_to.1).flat_map(move |y| {
                    (brick_from.2..=brick_to.2).map(move |z| ((x, y, z), brick_id))
                })
            })
        })
        .collect()
}

fn detect_connections(brick_points: &HashMap<Point3D, BrickID>) -> HashMap<Point3D, Point3D> {
    let max_x = brick_points.iter().max_by_key(|(p, _)| p.0).unwrap().0 .0;
    let max_y = brick_points.iter().max_by_key(|(p, _)| p.1).unwrap().0 .1;
    let max_z = brick_points.iter().max_by_key(|(p, _)| p.2).unwrap().0 .2;
    let mut connections: HashMap<Point3D, Point3D> = HashMap::new();
    // Iterate in plane from top to bottom, keeping track of last seen cube above
    let mut plane: HashMap<(usize, usize), (Point3D, BrickID)> = HashMap::new();
    dbg!(&max_x, &max_y, &max_z);

    for z in (0..=max_z).rev() {
        for x in 0..=max_x {
            for y in 0..=max_y {
                let current_pos = (x, y, z);
                if let Some(&brick_id) = brick_points.get(&current_pos) {
                    if let Some(&(prev_p, prev_brick_id)) = plane.get(&(x, y)) {
                        if brick_id != prev_brick_id {
                            connections.insert(prev_p, current_pos);
                        }
                    }
                    plane.insert((x, y), (current_pos, brick_id));
                }
            }
        }
    }

    connections
}

fn part_1(lines: Lines) -> usize {
    let bricks: Vec<(Point3D, Point3D)> = parse_bricks(lines);
    let brick_points: HashMap<Point3D, BrickID> = brick_points(&bricks);
    let connections: HashMap<Point3D, Point3D> = detect_connections(&brick_points);
    let connection_targets: HashSet<&Point3D> = connections.values().collect();

    println!("bricks: {:?}\n", bricks);
    println!("brick_points: {:?}\n", brick_points);
    println!("connections: {:?}\n", connections);

    let mut nr_of_connections_per_brick: HashMap<usize, usize> = HashMap::new();
    for (p, brick_id) in brick_points.iter() {
        if connections.contains_key(p) {
            nr_of_connections_per_brick
                .entry(*brick_id)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }
    println!(
        "nr_of_connections_per_brick: {:?}\n",
        nr_of_connections_per_brick
    );
    println!(
        "sum of children - 1 for each node: {}",
        nr_of_connections_per_brick
            .iter()
            .fold(0, |result, (_, c)| result + c - 1)
    );
    let bricks_with_parents: HashSet<usize> = brick_points
        .iter()
        .filter_map(|(k, v)| {
            if connection_targets.contains(k) {
                Some(*v)
            } else {
                None
            }
        })
        .collect();
    println!("total bricks: {}", bricks.len());
    println!("bricks with parents: {:?}", bricks_with_parents.len());
    println!("bricks without parents: {:?}", bricks.len() - bricks_with_parents.len());
    println!("result: {:?}", bricks.len() - bricks_with_parents.len() +
    nr_of_connections_per_brick
        .iter()
        .fold(0, |result, (_, c)| result + c - 1));

    render(&brick_points, &connections);

    0
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_22");
    (
        Solution::from(part_1(
            input
                .lines(),
        )),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 5);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_22").lines()), 0);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_22").lines()), 0);
    }
}
