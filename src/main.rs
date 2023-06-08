#![allow(dead_code)]

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::window::PrimaryWindow;

/* use bevy::window::WindowTheme; */
use bevy_prototype_lyon::prelude::*;
use std::collections::HashMap;

const ALPHABET: [char; 1] = ['F'];
const S: [char; 4] = ['+', '-', '[', ']'];
const Þ: &str = "F";
const P: [&'static str; 1] = ["F -> F-F++F-F"];
const DERIVATION: usize = 6;
const ANGLE: f32 = 60.0;
const ENTITY_SPEED: f32 = 0.5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "FLOAT".into(),
                resolution: (1920., 1080.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: false,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                /*  window_theme: Some(WindowTheme::Dark), */
                ..default()
            }),
            ..default()
        }))
        .add_plugin(ShapePlugin)
        .add_startup_systems((spawn_camera, spawn_turtle))
        .add_system(turtle_mouvement)
        .run();
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
                custom_size: Some(Vec2::new(100.0, 100.0)),
                anchor: Default::default(),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 3.0, window.height() / 2.0, 0.0),
            /*             transform: Transform::from_xyz(0.0, window.height() , 0.0), */
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

fn dessiner(mut turtle: &mut Turtle, angle: f32, transform: &mut Transform) {
    let mut remove_chars = 0;

    for character in turtle.instruction.chars() {
        remove_chars += 1;
        match character {
            '+' => turtle.horizontale += angle,

            '-' => turtle.horizontale -= angle,

            '[' => {
                turtle.pos_enregistre = (turtle.horizontale.clone(), transform.translation.clone())
            }

            ']' => (turtle.horizontale, transform.translation) = turtle.pos_enregistre,

            _ => break,
        }
    }

    turtle.instruction.replace_range(..remove_chars, "");
}

pub fn turtle_mouvement(
    mut commands: Commands,
    mut turtle_query: Query<(&mut Transform, &mut Turtle)>,
) {
    for (mut transform, mut turtle) in turtle_query.iter_mut() {
        let direction;

        println!("____________");
        dessiner(&mut turtle, ANGLE, transform.as_mut());

        direction = Vec3::new(
            turtle.horizontale.to_radians().cos(),
            turtle.horizontale.to_radians().sin(),
            0.0,
        );

        println!("chaine = {}", &turtle.instruction);
        println!("turtle looking: {}", &turtle.horizontale);
        println!("cord: {}", transform.translation);
        println!("direction :{}", direction);
        let old_position = transform.translation.clone();
        transform.translation += direction * ENTITY_SPEED;

        // Spawn a line segment entity

        let shape = shapes::Line(
            Vec2::new(old_position.x, old_position.y),
            Vec2::new(transform.translation.x, transform.translation.y),
        );
        println!(
            "segment coord {},{}",
            Vec2::new(old_position.x, old_position.y),
            Vec2::new(transform.translation.x, transform.translation.y)
        );
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            Stroke::new(Color::BLACK, 2.0),
        ));

        println!("____________");
    }
}
