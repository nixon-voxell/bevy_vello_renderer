use std::sync::Arc;

use bevy_asset::prelude::*;
use bevy_ecs::{prelude::*, system::SystemParamItem};
use bevy_reflect::{TypePath, TypeUuid};
use bevy_render::{prelude::*, render_asset::RenderAsset};
use bevy_transform::prelude::*;
use vello::SceneFragment;

#[derive(Bundle, Default, Clone)]
pub struct VelloFragmentBundle {
    pub fragment: Handle<VelloFragment>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// Enables or disables the fragment
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

#[derive(TypeUuid, TypePath, Clone)]
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
