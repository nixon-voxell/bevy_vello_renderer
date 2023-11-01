use bevy::prelude::*;
use bevy_log::LogPlugin;
use bevy_vello_renderer::prelude::*;
use vello::{kurbo, peniko, SceneBuilder, SceneFragment};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugins(VelloRenderPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, simple_animation)
        .run();
}

fn setup(mut commands: Commands, mut fragments: ResMut<Assets<VelloFragment>>) {
    let mut fragment: SceneFragment = SceneFragment::new();
    let mut sb: SceneBuilder = SceneBuilder::for_fragment(&mut fragment);

    sb.fill(
        peniko::Fill::NonZero,
        kurbo::Affine::default(),
        &peniko::Color::WHITE_SMOKE,
        None,
        &kurbo::Circle::new((0.0, 0.0), 100.0),
    );

    commands.spawn(Camera2dBundle::default());

    commands.spawn(VelloFragmentBundle {
        fragment: fragments.add(VelloFragment {
            fragment: fragment.into(),
        }),
        ..default()
    });
}

fn simple_animation(
    mut q_transforms: Query<&mut Transform, With<Handle<VelloFragment>>>,
    time: Res<Time>,
) {
    for mut transform in q_transforms.iter_mut() {
        transform.scale = Vec3::lerp(
            Vec3::ONE * 0.5,
            Vec3::ONE * 1.0,
            time.elapsed_seconds().sin().mul_add(0.5, 0.5),
        );
    }
}
