use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::str::Lines;
use std::usize;
use three_d::*;

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

fn render(brick_points: &HashMap<Point3D, usize>) {
    let window = Window::new(WindowSettings {
        title: "Shapes!".to_string(),
        max_size: Some((1280, 720)),
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
                    albedo: Srgba {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 200,
                    },
                    ..Default::default()
                },
            ),
        );
        sphere.set_transformation(
            Mat4::from_translation(vec3(x as f32, z as f32, y as f32)) * Mat4::from_scale(0.17),
        );
        shapes.push(sphere);
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

fn brick_points(bricks: &Vec<(Point3D, Point3D)>) -> HashMap<Point3D, usize> {
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

fn part_1(lines: Lines) -> usize {
    let bricks = parse_bricks(lines);
    let brick_points = brick_points(&bricks);

    println!("{:?}", bricks);
    println!("{:?}", brick_points);

    render(&brick_points);

    0
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_22");
    (
        Solution::from(part_1(
            "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
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
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 0);
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
