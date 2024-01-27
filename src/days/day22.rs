use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use std::str::Lines;
use std::usize;
use three_d::*;

type BrickID = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos3D {
    fn from(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn from_str(s: &str) -> Self {
        match s
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()[..]
        {
            [x, y, z] => Self { x, y, z },
            _ => panic!("Invalid point {}", s),
        }
    }
}

impl std::ops::Add for Pos3D {
    type Output = Pos3D;

    fn add(self, rhs: Self) -> Self::Output {
        Pos3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Pos3D {
    type Output = Pos3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
struct Brick {
    start: Pos3D,
    end: Pos3D,
    orientation: Orientation,
}

impl Brick {
    fn from(mut start: Pos3D, mut end: Pos3D) -> Self {
        if start.z > end.z {
            std::mem::swap(&mut start, &mut end);
        }

        Brick {
            start,
            end,
            orientation: if start.z == end.z {
                Orientation::Horizontal
            } else {
                Orientation::Vertical
            },
        }
    }

    fn from_str(s: &str) -> Self {
        match s.split("~").collect::<Vec<&str>>()[..] {
            [from, to] => Brick::from(Pos3D::from_str(from), Pos3D::from_str(to)),
            _ => panic!("Invalid brick {:?}", s),
        }
    }

    fn translate(&self, p: Pos3D) -> Self {
        Brick {
            start: self.start + p,
            end: self.end + p,
            orientation: self.orientation,
        }
    }

    fn points(&self) -> Vec<Pos3D> {
        let mut out = Vec::new();
        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                for z in self.start.z..=self.end.z {
                    out.push(Pos3D { x, y, z });
                }
            }
        }
        out
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn render(brick_points: &HashMap<Pos3D, BrickID>, connections: &HashMap<Pos3D, Pos3D>) {
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
    for (pos, brick_id) in brick_points.iter() {
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
            Mat4::from_translation(vec3(pos.x as f32, pos.z as f32, pos.y as f32))
                * Mat4::from_scale(0.5),
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
            Mat4::from_translation(vec3(pos.x as f32, pos.z as f32, pos.y as f32))
                * Mat4::from_scale(0.17),
        );
        shapes.push(sphere);
    }

    for (from, to) in connections.iter() {
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
            Mat4::from_translation(vec3(to.x as f32, to.z as f32, to.y as f32))
                * Mat4::from_scale(0.1 as f32)
                * Mat4::from_angle_z(Rad(PI / 2.0))
                * Mat4::from_nonuniform_scale((from.z - to.z) as f32, 1.0, 1.0),
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

// Drop bricks down the z axis as far as possible
fn drop_bricks(mut bricks: Vec<Brick>) -> Vec<Brick> {
    // sort bricks by height. Each brick will be dropped as low as possible
    bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
    // keep track of highest z occupied by brick for each x,y position
    let mut height_map: HashMap<(i32, i32), i32> = HashMap::new();

    let mut new_bricks: Vec<Brick> = Vec::new();
    for b in bricks {
        let dz: i32 = match b.orientation {
            Orientation::Horizontal => {
                let prev_heights: Vec<&i32> = b
                    .points()
                    .into_iter()
                    .filter_map(|p| height_map.get(&(p.x, p.y)))
                    .collect();

                if let Some(max_height) = prev_heights.into_iter().max() {
                    1 - (b.start.z - max_height)
                } else {
                    1 - b.start.z
                }
            }
            Orientation::Vertical => match height_map.get(&(b.start.x, b.start.y)) {
                Some(prev_z) => 1 - (b.start.z - prev_z),
                None => 1 - b.start.z,
            },
        };

        let dropped = b.translate(Pos3D::from(0, 0, dz));
        dropped.points().iter().for_each(|p| {
            height_map.insert((p.x, p.y), p.z);
        });
        new_bricks.push(dropped);
    }

    new_bricks
}

// Map all points of each brick to the ID of the brick, ID being the index in the vec of bricks.
fn brick_points(bricks: &Vec<Brick>) -> HashMap<Pos3D, BrickID> {
    bricks
        .iter()
        .enumerate()
        .flat_map(|(brick_id, brick)| brick.points().into_iter().map(move |p| (p, brick_id)))
        .collect()
}

// Build graph of connections between brick points. Points are connected if one is directly above another with
// no other points between
fn point_support_graph(brick_points: &HashMap<Pos3D, BrickID>) -> HashMap<Pos3D, Pos3D> {
    let max_x = brick_points.iter().max_by_key(|(p, _)| p.x).unwrap().0.x;
    let max_y = brick_points.iter().max_by_key(|(p, _)| p.y).unwrap().0.y;
    let max_z = brick_points.iter().max_by_key(|(p, _)| p.z).unwrap().0.z;
    let mut connections: HashMap<Pos3D, Pos3D> = HashMap::new();
    // Iterate in xy planes from top to bottom, keeping track of last seen cube above
    let mut height_map: HashMap<(i32, i32), (Pos3D, BrickID)> = HashMap::new();

    for z in (0..=max_z).rev() {
        for x in 0..=max_x {
            for y in 0..=max_y {
                let current_pos = Pos3D { x, y, z };
                if let Some(&brick_id) = brick_points.get(&current_pos) {
                    if let Some((prev_p, prev_brick_id)) = height_map.get(&(x, y)) {
                        if brick_id != *prev_brick_id && prev_p.z - z == 1 {
                            connections.insert(prev_p.clone(), current_pos.clone());
                        }
                    }
                    height_map.insert((x, y), (current_pos, brick_id));
                }
            }
        }
    }

    connections
}

// Figure how the blocks will settle based on the snapshot. Once they've settled, consider
// disintegrating a single brick; how many bricks could be safely chosen as the one to get disintegrated?
fn part_1(lines: Lines) -> usize {
    let bricks: Vec<Brick> = drop_bricks(lines.map(|line| Brick::from_str(line)).collect());
    // map each brick point/block to the ID of the brick it belongs to
    let brick_points: HashMap<Pos3D, BrickID> = brick_points(&bricks);
    // map parent blocks to the child blocks they lie directly on top of
    let connections: HashMap<Pos3D, Pos3D> = point_support_graph(&brick_points);
    let mut parents_to_children: HashMap<BrickID, HashSet<BrickID>> = HashMap::new();
    let mut children_to_parents: HashMap<BrickID, HashSet<BrickID>> = HashMap::new();

    for (parent_point, child_point) in connections.clone() {
        if let Some(&parent_id) = brick_points.get(&parent_point) {
            if let Some(&child_id) = brick_points.get(&child_point) {
                parents_to_children
                    .entry(parent_id)
                    .or_insert(HashSet::new())
                    .insert(child_id);

                children_to_parents
                    .entry(child_id)
                    .or_insert(HashSet::new())
                    .insert(parent_id);
            }
        }
    }

    let result = children_to_parents
        .values()
        .filter(|parents| {
            // each child can be disintegrated if its parent has more than 1 children
            parents
                .iter()
                .all(|p| parents_to_children.get(p).unwrap().len() > 1)
        })
        .count();
    let orphans = bricks.len() - children_to_parents.len();

    // render(&brick_points, &connections);

    result + orphans
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_22");
    (
        Solution::from(part_1(input.lines())),
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
        assert_eq!(part_1(load_input("inputs/day_22").lines()), 499);
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
