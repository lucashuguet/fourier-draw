use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use fft::fourier_coefficients;
use std::collections::HashMap;
use std::f32::consts::PI;

#[derive(Resource)]
struct Data {
    circles: HashMap<i32, FourierCircleData>,
    lines: Vec<(Vec2, Vec2)>,
    last: (i32, Vec2)
}

#[allow(dead_code)]
struct FourierCircleData {
    coords: Vec2,
    radius: f32,
    phase: f32
}

#[derive(Component)]
struct Circle(i32);

impl Data {
    fn new(file: &str, scale: f32) -> Self {
	let mut coefficients = fourier_coefficients(file, Box::new(-150..150));
	coefficients.sort_by(|a, b| {
	    b.radius.abs().partial_cmp(&a.radius.abs()).unwrap()
	});
	coefficients.remove(0);

	let mut circles: HashMap<i32, FourierCircleData> = HashMap::new();
	
	for circle in coefficients.iter() {
	    let circle_data = FourierCircleData {
		coords: Vec2::ZERO,
		radius: circle.radius * scale,
		phase: circle.phase,
	    };

	    circles.insert(circle.speed, circle_data);
	}
	
	Data {
	    circles,
	    lines: Vec::new(),
	    last: (coefficients.last().unwrap().speed, Vec2::ZERO),
	}
    }

    fn update(&mut self, angle: f32) {
	let mut circles: HashMap<i32, FourierCircleData> = HashMap::new();
	let mut old_coords = Vec2::ZERO;
	let mut old_radius = 0.;

	let mut sorted_circles: Vec<(&i32, &FourierCircleData)> = self.circles.iter().map(|(i, circle)| (i, circle)).collect();

	sorted_circles.sort_by(|a, b| {
	    b.1.radius.abs().partial_cmp(&a.1.radius.abs()).unwrap()
	});
	
	for circle in sorted_circles.iter() {
	    let circle_data = FourierCircleData {
		coords: Vec2::new(
		    old_coords.x + old_radius * circle.1.phase.cos(),
		    old_coords.y + old_radius * circle.1.phase.sin()
		),
		radius: circle.1.radius,
		phase: circle.1.phase -*circle.0 as f32 * angle,
	    };

	    old_coords = circle_data.coords;
	    old_radius = circle_data.radius;

	    circles.insert(*circle.0, circle_data);
	}

	let lastcoords = self.circles.get(&self.last.0).unwrap().coords;
	self.circles = circles;
	self.last.1 = lastcoords;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Data::new("fourier.svg", 0.5))
        .add_startup_system(setup)
        .add_system(frame)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    data: Res<Data>
) {
    commands.spawn(Camera2dBundle::default());

    for circle in data.circles.iter() {
	let x = circle.1.coords.x;
	let y = circle.1.coords.y;

	commands.spawn((MaterialMesh2dBundle {
	    mesh: meshes.add(shape::Torus{radius: circle.1.radius, ring_radius: 2., subdivisions_segments: 100, subdivisions_sides: 100}.into()).into(),
	    material: materials.add(ColorMaterial::from(Color::BLACK)),
	    transform: Transform {
		translation: Vec3::new(x, y, 0.),
		rotation: Quat::from_rotation_x(PI/2.),
		..default()
	    },
	    ..default()
	}, Circle(*circle.0)));
    }
}

fn frame(mut data: ResMut<Data>, time: Res<Time>, mut query: Query<(&mut Transform, &Circle)>) {
    for entity in query.iter_mut() {
	let (mut transform, circle) = entity;
	let circle_data = data.circles.get(&circle.0).unwrap();
	let x = circle_data.coords.x;
	let y = circle_data.coords.y;

	transform.translation = Vec3::new(x, y, 0.);
    }

    data.update(2. * PI * time.delta_seconds() * 0.2);
}
