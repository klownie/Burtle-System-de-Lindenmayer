#![allow(dead_code)]

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_prototype_lyon::prelude::*;
use std::collections::HashMap;

const ALPHABET: [char; 1] = ['F'];
const S: [char; 4] = ['+', '-', '[', ']'];
const Þ: &str = "F";
const P: [&'static str; 1] = ["F -> FF-[-F+F+F]+[+F-F-F]"];
const DERIVATION: usize = 3;
const ANGLE: f32 = 25.7;
const ENTITY_SPEED: f32 = 900.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_turtle)
        .add_system(turtle_mouvement)
        .run();
}

fn derive_iter<'a>(þ: &'a mut String, n: usize, p: &[&'static str; 1]) {
    let mut rules: HashMap<char, String> = HashMap::new();

    for rule in p {
        let parts: Vec<&str> = rule.split("->").map(|s| s.trim()).collect();
        if let [lhs, rhs] = parts.as_slice() {
            let key = lhs.chars().next().unwrap();
            rules.insert(key, rhs.to_string());
        }
    }

    for _ in 0..n {
        let mut deriv_þ = String::new();
        for character in þ.chars() {
            if let Some(rule) = rules.get(&character) {
                deriv_þ.push_str(rule);
            } else {
                deriv_þ.push(character);
            }
        }
        *þ = deriv_þ;
    }
}

fn dessiner(mut turtle: &mut Turtle, angle: f32, pos: &mut Transform) {
    let mut remove_chars = 0;

    for character in turtle.instruction.chars() {
        remove_chars += 1;
        match character {
            '+' => turtle.horizontale += angle,

            '-' => turtle.horizontale -= angle,

            '[' => turtle.pos_enregistre = (turtle.horizontale.clone(), pos.translation.clone()),

            ']' => {
                (turtle.horizontale, pos.translation) = turtle.pos_enregistre;
                turtle.crayon = false
            }

            _ => break,
        }
    }

    turtle.instruction.replace_range(..remove_chars, "");
}

#[derive(Component)]
pub struct Turtle {
    horizontale: f32,
    crayon: bool,
    pos_enregistre: (f32, Vec3),
    instruction: String,
}

impl Turtle {
    pub fn right(&mut self, angle: f32) {
        self.horizontale += angle
    }
    pub fn left(&mut self, angle: f32) {
        self.horizontale -= angle
    }
}

pub fn spawn_turtle(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let mut þ = Þ.to_owned();
    derive_iter(&mut þ, DERIVATION, &P);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                flip_x: false,
                flip_y: false,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                anchor: Default::default(),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/turtle.png"),
            ..default()
        },
        Turtle {
            horizontale: 0.0,
            crayon: true,
            pos_enregistre: (
                0.0,
                Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
            ),
            instruction: þ,
        },
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn turtle_mouvement(
    mut commands: Commands,
    mut turtle_query: Query<(&mut Transform, &mut Turtle)>,
    time: Res<Time>,
) {
    for (mut transform, mut turtle) in turtle_query.iter_mut() {
        let mut direction;
        let old_position = transform.translation.clone();

        dessiner(&mut turtle, ANGLE, transform.as_mut());
        println!("chaine = {}", &turtle.instruction);
        println!("turtle looking: {}", &turtle.horizontale);

        direction = Vec3::new(
            turtle.horizontale.to_radians().cos(),
            turtle.horizontale.to_radians().sin(),
            0.0,
        );
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * ENTITY_SPEED * time.delta_seconds();

        // Spawn a line segment entity
        /*
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(2.0, 2.0)),
                ..default()
            },
            transform: Transform::from_translation(old_position),
            ..default()
        });
        */
        if turtle.crayon {
            let shape = shapes::Line(
                Vec2::new(old_position.x, old_position.y),
                Vec2::new(transform.translation.x, transform.translation.y),
            );
            commands.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Stroke::new(Color::BLACK, 1.0),
            ));
        } else {
            turtle.crayon = true
        }
    }
}
