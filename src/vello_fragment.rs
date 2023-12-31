use std::sync::Arc;

use bevy_asset::prelude::*;
use bevy_ecs::{prelude::*, system::SystemParamItem};
use bevy_reflect::{TypePath, TypeUuid};
use bevy_render::{prelude::*, render_asset::RenderAsset};
use bevy_transform::prelude::*;
use vello::SceneFragment;

#[derive(Bundle, Clone, Default)]
pub struct VelloFragmentBundle {
    pub fragment: Handle<VelloFragment>,
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

#[derive(Asset, TypeUuid, TypePath, Clone, Default)]
#[uuid = "0ee4b8fa-fbee-49f6-bcfe-7d517ff94d40"]
pub struct VelloFragment {
    pub fragment: Arc<SceneFragment>,
}

impl RenderAsset for VelloFragment {
    type ExtractedAsset = Self;

    type PreparedAsset = Self;

    type Param = ();

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        _param: &mut SystemParamItem<Self::Param>,
    ) -> Result<
        Self::PreparedAsset,
        bevy_render::render_asset::PrepareAssetError<Self::ExtractedAsset>,
    > {
        Ok(extracted_asset)
    }
}

#[derive(Component, Clone)]
pub struct ExtractedVelloFragmentInstance {
    pub fragment_handle: Handle<VelloFragment>,
    pub global_transform: GlobalTransform,
}
