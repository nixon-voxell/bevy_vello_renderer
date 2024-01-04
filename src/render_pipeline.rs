use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_render::{
    camera::ExtractedCamera,
    prelude::*,
    render_asset::RenderAssets,
    renderer::{RenderDevice, RenderQueue},
    view::ExtractedView,
    Extract,
};
use bevy_transform::prelude::*;
use bevy_utils::synccell::SyncCell;
use vello::{kurbo::Affine, RenderParams, Renderer, RendererOptions, Scene, SceneBuilder};

use crate::{
    canvas::VelloCanvas,
    vello_fragment::{ExtractedVelloFragmentInstance, VelloFragment},
};

#[derive(Resource)]
pub struct VelloRenderer(SyncCell<Renderer>);

impl FromWorld for VelloRenderer {
    fn from_world(world: &mut World) -> Self {
        let device = world.get_resource::<RenderDevice>().unwrap();

        VelloRenderer(SyncCell::new(
            Renderer::new(
                device.wgpu_device(),
                RendererOptions {
                    surface_format: None,
                    timestamp_period: 0.0,
                    use_cpu: false,
                    antialiasing_support: vello::AaSupport::all(),
                },
            )
            .expect("no gpu device"),
        ))
    }
}

#[derive(Component, Copy, Clone)]
pub struct PreparedAffine(pub Affine);

// Extract ========================================

pub fn extract_fragment_instances(
    mut commands: Commands,
    q_fragments: Extract<
        Query<(
            Entity,
            &Handle<VelloFragment>,
            &GlobalTransform,
            &ViewVisibility,
        )>,
    >,
    mut previous_len: Local<usize>,
) {
    let mut instances: Vec<(Entity, ExtractedVelloFragmentInstance)> =
        Vec::with_capacity(*previous_len);

    for (entity, fragment_handle, global_transform, view_visibilty) in q_fragments.iter() {
        if view_visibilty.get() == false {
            continue;
        }

        instances.push((
            entity,
            ExtractedVelloFragmentInstance {
                fragment_handle: fragment_handle.clone(),
                global_transform: global_transform.clone(),
            },
        ))
    }

    *previous_len = instances.len();
    commands.insert_or_spawn_batch(instances);
}

// Prepare ========================================

pub fn prepare_fragment_affines(
    mut commands: Commands,
    q_camera: Query<(&ExtractedCamera, &ExtractedView)>,
    q_fragment_instances: Query<(Entity, &ExtractedVelloFragmentInstance)>,
) {
    let Ok((camera, view)) = q_camera.get_single() else {
        return;
    };

    let size_pixels: UVec2 = camera.physical_viewport_size.unwrap();
    let (pixels_x, pixels_y) = (size_pixels.x as f32, size_pixels.y as f32);

    for (entity, fragment_instance) in q_fragment_instances.iter() {
        let ndc_to_pixels_matrix = Mat4::from_cols_array_2d(&[
            [pixels_x / 2.0, 0.0, 0.0, pixels_x / 2.0],
            [0.0, pixels_y / 2.0, 0.0, pixels_y / 2.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
        .transpose();

        // The vello scene transform is world-space
        let raw_transform = {
            let mut model_matrix = fragment_instance.global_transform.compute_matrix();
            model_matrix.w_axis.y *= -1.0;

            let (projection_mat, view_mat) = {
                let mut view_mat = view.transform.compute_matrix();
                view_mat.w_axis.y *= -1.0;

                (view.projection, view_mat)
            };

            let view_proj_matrix = projection_mat * view_mat.inverse();

            ndc_to_pixels_matrix * view_proj_matrix * model_matrix
        };

        let transform: [f32; 16] = raw_transform.to_cols_array();

        // | a c e |
        // | b d f |
        // | 0 0 1 |
        let transform: [f64; 6] = [
            transform[0] as f64,  // a
            -transform[1] as f64, // b
            -transform[4] as f64, // c
            transform[5] as f64,  // d
            transform[12] as f64, // e
            transform[13] as f64, // f
        ];

        commands
            .entity(entity)
            .insert(PreparedAffine(Affine::new(transform)));
    }
}

// Render ========================================

/// Transforms all the fragments extracted from the game world and places them in
/// a scene, and renders the scene to a texture with WGPU
#[allow(clippy::complexity)]
pub fn render_scene(
    q_fragment_instances: Query<(&ExtractedVelloFragmentInstance, &PreparedAffine)>,
    fragments: Res<RenderAssets<VelloFragment>>,
    mut vello_renderer: ResMut<VelloRenderer>,
    vello_canvas: Res<VelloCanvas>,
    gpu_images: Res<RenderAssets<Image>>,
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
) {
    let Some(gpu_image) = gpu_images.get(&vello_canvas.image_handle) else {
        return;
    };

    let mut scene = Scene::default();
    let mut builder = SceneBuilder::for_scene(&mut scene);

    // Background items: z ordered
    let mut vector_render_queue: Vec<(&ExtractedVelloFragmentInstance, &PreparedAffine)> =
        q_fragment_instances.iter().collect();
    vector_render_queue.sort_by(|(a, _), (b, _)| {
        let a = a.global_transform.translation().z;
        let b = b.global_transform.translation().z;
        a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
    });

    // Apply transforms to the respective fragments and add them to the
    // scene to be rendered
    for (
        ExtractedVelloFragmentInstance {
            fragment_handle, ..
        },
        PreparedAffine(affine),
    ) in vector_render_queue.iter()
    {
        let Some(fragment) = fragments.get(fragment_handle) else {
            continue;
        };

        builder.append(&fragment.fragment, Some(*affine));
    }

    if !vector_render_queue.is_empty() {
        vello_renderer
            .0
            .get()
            .render_to_texture(
                device.wgpu_device(),
                &queue,
                &scene,
                &gpu_image.texture_view,
                &RenderParams {
                    base_color: vello::peniko::Color::TRANSPARENT,
                    width: gpu_image.size.x as u32,
                    height: gpu_image.size.y as u32,
                    antialiasing_method: vello::AaConfig::Area,
                },
            )
            .unwrap();
    }
}
