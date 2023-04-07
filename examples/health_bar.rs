use bevy::prelude::*;
use bevy_mod_simplest_healthbar::{HealthBar, HealthBarPlugin, HealthTrait};

#[derive(Component)]
struct Health {
    current: u32,
    max: u32,
}
impl HealthTrait for Health {
    fn current(&self) -> u32 {
        self.current
    }

    fn max(&self) -> u32 {
        self.max
    }
}

#[derive(Component)]
struct BarCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // custom
        .add_plugin(
            // Need to define which camera we are going to be spawning the stuff in relation to, as well as what is the "health" component
            HealthBarPlugin::<Health, BarCamera>::new("fonts/FiraMono-Medium.ttf")
                // to automatically spawn bars on stuff with Health and a Transform
                .automatic_bar_creation(true),
        )
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(40., 40., 40.))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BarCamera,
    ));

    for i in 0..5 {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(shape::Cube::default().into()),
                material: materials.add(Color::BLUE.into()),
                transform: Transform::from_translation(Vec3::new(0.0, 1.0, (i * 5) as _)),
                ..default()
            },
            Health { current: i, max: 6 },
        ));
    }
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::default().into()),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, -5.)),
            ..default()
        },
        Health { current: 4, max: 6 },
        // Create custom size and color and offset for the "bar"
        HealthBar {
            offset: Vec2::new(-4., 10.),
            size: 100.,
            color: Color::GREEN,
        },
    ));
}
