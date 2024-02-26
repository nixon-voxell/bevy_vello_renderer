use std::sync::Arc;

use bevy_asset::prelude::*;
use bevy_ecs::{prelude::*, system::SystemParamItem};
use bevy_reflect::TypePath;
use bevy_render::{
    prelude::*,
    render_asset::{RenderAsset, RenderAssetUsages},
};
use bevy_transform::prelude::*;
use vello::Scene;

#[derive(Bundle, Clone, Default)]
pub struct VelloSceneBundle {
    pub scene: Handle<VelloScene>,
    /// Local transform of the entity,
    pub transform: Transform,
    /// Global transform of the entity,
    pub global_transform: GlobalTransform,
    /// The visibility of the entity.
    pub visibility: Visibility,
    // The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,
    // The computed visibility of the entity.
    pub view_visibility: ViewVisibility,
}

#[derive(Asset, TypePath, Clone, Default)]
pub struct VelloScene {
    pub scene: Arc<Scene>,
}

impl RenderAsset for VelloScene {
    type PreparedAsset = Self;

    type Param = ();

    fn asset_usage(&self) -> RenderAssetUsages {
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
    }

    fn prepare_asset(
        self,
        _param: &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, bevy_render::render_asset::PrepareAssetError<Self>> {
        Ok(self)
    }
}

#[derive(Component, Clone)]
pub struct ExtractedVelloSceneInstance {
    pub scene_handle: Handle<VelloScene>,
    pub global_transform: GlobalTransform,
}
