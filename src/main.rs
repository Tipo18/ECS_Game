use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        Player,
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
    ));
}

const MOVE_SPEED: f32 = 180.0;

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Z) {
            direction.y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Q) {
            direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 0.0 {
            transform.translation += direction.normalize() * MOVE_SPEED * time.delta_seconds();
        }
    }
}
