pub use vello;
pub use vello_svg;

use bevy_app::prelude::*;
use bevy_asset::{load_internal_asset, prelude::*};
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_render::{
    extract_resource::ExtractResourcePlugin, prelude::*, render_asset::RenderAssetPlugin, Render,
    RenderApp, RenderSet,
};
use bevy_sprite::Material2dPlugin;
use canvas::CanvasMaterial;
use vello_fragment::VelloFragment;

pub mod canvas;
pub mod render_pipeline;
pub mod vello_fragment;

pub mod prelude {
    pub use super::vello_fragment::{VelloFragment, VelloFragmentBundle};
    pub use super::VelloRenderPlugin;
}

const SSRT_SHADER_HANDLE: Handle<Shader> =
    Handle::<Shader>::weak_from_u128(297150281341545723940939177574271201838);

pub struct VelloRenderPlugin;

impl Plugin for VelloRenderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            SSRT_SHADER_HANDLE,
            "./shaders/ss_rendertarget.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins((
            RenderAssetPlugin::<VelloFragment>::default(),
            Material2dPlugin::<canvas::CanvasMaterial>::default(),
            ExtractResourcePlugin::<canvas::VelloCanvas>::default(),
        ))
        .init_asset::<VelloFragment>()
        .init_asset::<CanvasMaterial>()
        .add_systems(PreStartup, canvas::setup_canvas)
        .add_systems(PreUpdate, canvas::resize_canvas_image);

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            error!("No render app available!");
            return;
        };

        // TODO: reconsider this
        // render_app.insert_resource(ExtractedPixelScale(1.0));
        render_app
            .add_systems(
                ExtractSchedule,
                (
                    // TODO: reconsider this
                    // render_pipeline::extract_pixel_scale.in_set(RenderSet::ExtractCommands),
                    render_pipeline::extract_fragment_instances,
                ),
            )
            .add_systems(
                Render,
                (
                    render_pipeline::prepare_fragment_affines.in_set(RenderSet::Prepare),
                    render_pipeline::render_scene.in_set(RenderSet::Render),
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            error!("No render app available!");
            return;
        };

        render_app.init_resource::<render_pipeline::VelloRenderer>();
    }
}
