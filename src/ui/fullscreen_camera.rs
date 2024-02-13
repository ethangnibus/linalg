

use super::components::example_block;
use super::components::example_header;
use super::components::example_footer;
use super::option_bar;
use super::routes;
use super::theme;
use super::under_navbar;
use super::util::style;
use super::util::subsection;
use super::util::subsection::SubsectionGameEntity;
use super::{routes::RoutingEvent, view::UiResizeEvent};
use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseWheel;
use bevy::utils::intern;
use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        clear_color::ClearColorConfig,
    },
    ecs::event::ManualEventReader,
    prelude::*,
    render::{
        camera::{
            self, CameraProjection, ComputedCameraValues, RenderTarget, ScalingMode, Viewport,
        },
        primitives::Frustum,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::{RenderLayers, VisibleEntities},
    },
    ui::FocusPolicy,
};
use bevy_mod_picking::prelude::*;
// use rand::Rng;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(
        //     Update,
        //     (
               
        //     ),
        // );
    }
}


#[derive(Component)]
pub struct FullscreenCameraBanner;



#[derive(Component)]
pub struct TextbookCameraBanner;