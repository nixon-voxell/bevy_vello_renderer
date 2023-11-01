//! A shader that samples a texture with view-independent UV coordinates.

use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_math::prelude::*;
use bevy_reflect::{TypePath, TypeUuid};
use bevy_render::{
    extract_resource::ExtractResource,
    mesh::{Indices, MeshVertexBufferLayout},
    prelude::*,
    render_resource::{
        AsBindGroup, Extent3d, PrimitiveTopology, RenderPipelineDescriptor, ShaderRef,
        SpecializedMeshPipelineError, TextureDescriptor, TextureDimension, TextureFormat,
        TextureUsages, VertexBufferLayout, VertexFormat, VertexStepMode,
    },
    texture::Image,
    view::NoFrustumCulling,
};
use bevy_sprite::{Material2d, Material2dKey, MaterialMesh2dBundle, Mesh2dHandle};
use bevy_transform::prelude::*;
use bevy_utils::prelude::*;
use bevy_window::{prelude::*, WindowResized, WindowResolution};

use crate::SSRT_SHADER_HANDLE;

#[derive(Resource, Clone)]
pub struct VelloCanvas {
    pub(crate) material_handle: Handle<CanvasMaterial>,
    pub(crate) image_handle: Handle<Image>,
}

impl ExtractResource for VelloCanvas {
    type Source = Self;

    fn extract_resource(source: &Self::Source) -> Self {
        source.clone()
    }
}

#[derive(AsBindGroup, TypeUuid, TypePath, Clone)]
#[uuid = "b62bb455-a72c-4b56-87bb-81e0554e234f"]
pub struct CanvasMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
}

impl Material2d for CanvasMaterial {
    fn vertex_shader() -> ShaderRef {
        SSRT_SHADER_HANDLE.typed().into()
    }

    fn fragment_shader() -> ShaderRef {
        SSRT_SHADER_HANDLE.typed().into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let formats = vec![
            // Position
            VertexFormat::Float32x3,
            VertexFormat::Float32x2,
        ];

        let vertex_layout =
            VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);

        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}

fn create_image(images: &mut Assets<Image>, window: &WindowResolution) -> Handle<Image> {
    let size = Extent3d {
        width: window.physical_width(),
        height: window.physical_height(),
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::STORAGE_BINDING,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);
    images.add(image)
}

pub fn setup_canvas(
    mut commands: Commands,
    q_windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut canvas_materials: ResMut<Assets<CanvasMaterial>>,
    mut canvas_mesh_handle: Local<Option<Handle<Mesh>>>,
) {
    let Ok(window) = q_windows.get_single() else {
        return;
    };

    // Create or get quad mesh for the canvas to draw to.
    let mesh_handle = canvas_mesh_handle.get_or_insert_with(|| {
        let mut canvas_quad = Mesh::new(PrimitiveTopology::TriangleList);

        let mut v_pos = vec![[-1.0, -1.0, 0.0]];
        v_pos.push([1.0, -1.0, 0.0]);
        v_pos.push([1.0, 1.0, 0.0]);
        v_pos.push([-1.0, 1.0, 0.0]);

        // Set the position attribute
        canvas_quad.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

        let v_pos = vec![[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [1.0, 1.0]];

        canvas_quad.insert_attribute(Mesh::ATTRIBUTE_UV_0, v_pos);

        let indices = vec![0, 1, 2, 0, 2, 3];
        canvas_quad.set_indices(Some(Indices::U32(indices)));

        meshes.add(canvas_quad)
    });

    let image_handle = create_image(&mut images, &window.resolution);

    let mesh = Mesh2dHandle(mesh_handle.clone());
    let material = canvas_materials.add(CanvasMaterial {
        texture: image_handle.clone(),
    });

    commands.insert_resource(VelloCanvas {
        material_handle: material.clone(),
        image_handle,
    });

    commands
        .spawn(MaterialMesh2dBundle {
            mesh,
            material,
            transform: Transform::from_translation(0.001 * Vec3::NEG_Z), // Make sure the vello canvas renders behind Gizmos
            ..Default::default()
        })
        .insert(NoFrustumCulling);
}

pub fn resize_canvas_image(
    q_windows: Query<&Window>,
    vello_canvas: Res<VelloCanvas>,
    mut canvas_materials: ResMut<Assets<CanvasMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut evr_window_resize: EventReader<WindowResized>,
) {
    let Ok(window) = q_windows.get_single() else {
        return;
    };

    if evr_window_resize.iter().last().is_some() {
        let size = Extent3d {
            width: window.resolution.physical_width(),
            height: window.resolution.physical_height(),
            ..default()
        };

        // Skip resizing if dimension is 0
        if size.width == 0 || size.height == 0 {
            return;
        }

        let Some(canvas_material) = canvas_materials.get_mut(&vello_canvas.material_handle) else {
            return;
        };

        let Some(image) = images.get_mut(&vello_canvas.image_handle) else {
            return;
        };

        // Resize the image
        image.resize(size);

        // Reassign the image handle (apparently Bevy require us to do this, no idea why).
        canvas_material.texture = vello_canvas.image_handle.clone();

        debug!(
            "Resized Vello canvas image to {:?}",
            (size.width, size.height)
        );
    }
}
