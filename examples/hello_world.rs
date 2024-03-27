use bevy::prelude::*;
use bevy_vello_renderer::prelude::*;
use vello::{kurbo, peniko, Scene};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VelloRenderPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, simple_animation)
        .run();
}

fn setup(mut commands: Commands, mut scenes: ResMut<Assets<VelloScene>>) {
    let mut scene = Scene::new();

    scene.fill(
        peniko::Fill::NonZero,
        kurbo::Affine::default(),
        &peniko::Color::WHITE_SMOKE,
        None,
        &kurbo::RoundedRect::new(-50.0, -50.0, 50.0, 50.0, 10.0),
    );

    commands.spawn(Camera2dBundle::default());

    commands.spawn(VelloSceneBundle {
        scene: scenes.add(VelloScene {
            scene: scene.into(),
        }),
        ..default()
    });
}

fn simple_animation(
    mut q_transforms: Query<&mut Transform, With<Handle<VelloScene>>>,
    time: Res<Time>,
) {
    let sin_time = time.elapsed_seconds().sin().mul_add(0.5, 0.5);

    for mut transform in q_transforms.iter_mut() {
        transform.scale = Vec3::lerp(Vec3::ONE * 0.5, Vec3::ONE * 1.0, sin_time);
        transform.translation = Vec3::lerp(Vec3::X * -100.0, Vec3::X * 100.0, sin_time);
        transform.rotation = Quat::from_rotation_z(-std::f32::consts::TAU * sin_time);
    }
}
