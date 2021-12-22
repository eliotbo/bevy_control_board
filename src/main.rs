// use inputs::*;
// use util::*;

// use bevy::{
//     prelude::*,
//     reflect::TypeRegistry,
//     render::camera::OrthographicProjection,
//     render::pipeline::{PipelineDescriptor, RenderPipeline, RenderPipelines},
// };

// fn main() {
//     let mut app = App::new();

//     app.insert_resource(WindowDescriptor {
//         title: "I am a window!".to_string(),
//         width: 2200.,
//         height: 1600.,
//         vsync: true,
//         ..Default::default()
//     })
//     .add_plugins(DefaultPlugins)
//     // .add_plugin(DashboardPlugin)
//     .add_startup_system(main_setup);

//     add_dashboard_component![app, DoubleInner, MyEnum, Inner, Globals];
//     // add_dashboard_component_updates![DoubleInner, Inner, Globals];
//     add_dashboard_resource![app, Globals];

//     app.run();
// }

// fn main_setup(mut commands: Commands) {
//     commands.spawn_bundle(OrthographicCameraBundle {
//         transform: Transform::from_translation(Vec3::new(00.0, 0.0, 10.0))
//             .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
//         orthographic_projection: OrthographicProjection {
//             scale: 1.0,
//             far: 100000.0,
//             near: -100000.0,
//             ..Default::default()
//         },
//         ..OrthographicCameraBundle::new_2d()
//     });
// }

// pub fn record_mouse_events_system(
//     mut cursor_moved_events: EventReader<CursorMoved>,
//     mouse_button_input: Res<Input<MouseButton>>,
//     mut cursor_res: ResMut<Cursor>,
//     mut windows: ResMut<Windows>,
//     cam_transform_query: Query<&Transform, With<OrthographicProjection>>,
//     cam_ortho_query: Query<&OrthographicProjection>,
// ) {
//     for event in cursor_moved_events.iter() {
//         let cursor_in_pixels = event.position; // lower left is origin
//         let window_size = Vec2::new(
//             windows.get_primary_mut().unwrap().width(),
//             windows.get_primary_mut().unwrap().height(),
//         );

//         let screen_position = cursor_in_pixels - window_size / 2.0;

//         let cam_transform = cam_transform_query.iter().next().unwrap();

//         // this variable currently has no effect
//         let mut scale = 1.0;

//         for ortho in cam_ortho_query.iter() {
//             scale = ortho.scale;
//         }

//         let cursor_vec4: Vec4 = cam_transform.compute_matrix()
//             * screen_position.extend(0.0).extend(1.0 / (scale))
//             * scale;

//         let cursor_pos = Vec2::new(cursor_vec4.x, cursor_vec4.y);
//         cursor_res.position = cursor_pos;
//         cursor_res.pos_relative_to_click = cursor_res.position - cursor_res.last_click_position;
//     }

//     if mouse_button_input.just_pressed(MouseButton::Left) {
//         cursor_res.last_click_position = cursor_res.position;
//         cursor_res.pos_relative_to_click = Vec2::ZERO;
//     }
// }

use bevy::{
    core_pipeline::Transparent3d,
    pbr::{
        DrawMesh, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup,
        SetMeshViewBindGroup,
    },
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_component::{ExtractComponent, ExtractComponentPlugin},
        render_phase::{AddRenderCommand, DrawFunctions, RenderPhase, SetItemPipeline},
        render_resource::{
            RenderPipelineCache, RenderPipelineDescriptor, SpecializedPipeline,
            SpecializedPipelines,
        },
        view::ExtractedView,
        RenderApp, RenderStage,
    },
};

pub struct IsRedPlugin;

impl Plugin for IsRedPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractComponentPlugin::<IsRed>::default());
        app.sub_app(RenderApp)
            .add_render_command::<Transparent3d, DrawIsRed>()
            .init_resource::<IsRedPipeline>()
            .init_resource::<SpecializedPipelines<IsRedPipeline>>()
            .add_system_to_stage(RenderStage::Queue, queue_custom);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(IsRedPlugin)
        .add_startup_system(setup)
        .run();
}

#[derive(Component, Hash, PartialEq, Eq, Copy, Clone)]
struct IsRed(bool);

impl ExtractComponent for IsRed {
    type Query = &'static IsRed;

    type Filter = ();

    fn extract_component(item: bevy::ecs::query::QueryItem<Self::Query>) -> Self {
        *item
    }
}

/// set up a simple 3D scene
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // red cube
    commands.spawn().insert_bundle((
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        IsRed(true),
        Transform::from_xyz(-1.0, 0.5, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
    ));

    // blue cube
    commands.spawn().insert_bundle((
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        IsRed(false),
        Transform::from_xyz(1.0, 0.5, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
    ));

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

struct IsRedPipeline {
    mesh_pipline: MeshPipeline,
    shader: Handle<Shader>,
}

impl FromWorld for IsRedPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let mesh_pipeline = world.get_resource::<MeshPipeline>().unwrap();
        let shader = asset_server.load("shaders/shader_defs.wgsl");
        IsRedPipeline {
            mesh_pipline: mesh_pipeline.clone(),
            shader,
        }
    }
}

impl SpecializedPipeline for IsRedPipeline {
    type Key = (IsRed, MeshPipelineKey);

    fn specialize(&self, (is_red, pbr_pipeline_key): Self::Key) -> RenderPipelineDescriptor {
        let mut shader_defs = Vec::new();
        if is_red.0 {
            shader_defs.push("IS_RED".to_string());
        }
        let mut descriptor = self.mesh_pipline.specialize(pbr_pipeline_key);
        descriptor.vertex.shader = self.shader.clone();
        descriptor.vertex.shader_defs = shader_defs.clone();
        let fragment = descriptor.fragment.as_mut().unwrap();
        fragment.shader = self.shader.clone();
        fragment.shader_defs = shader_defs;
        descriptor.layout = Some(vec![
            self.mesh_pipline.view_layout.clone(),
            self.mesh_pipline.mesh_layout.clone(),
        ]);
        descriptor
    }
}

type DrawIsRed = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    DrawMesh,
);

#[allow(clippy::too_many_arguments)]
fn queue_custom(
    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    render_meshes: Res<RenderAssets<Mesh>>,
    custom_pipeline: Res<IsRedPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedPipelines<IsRedPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    material_meshes: Query<(Entity, &Handle<Mesh>, &MeshUniform, &IsRed)>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
) {
    let draw_custom = transparent_3d_draw_functions
        .read()
        .get_id::<DrawIsRed>()
        .unwrap();
    let key = MeshPipelineKey::from_msaa_samples(msaa.samples);
    for (view, mut transparent_phase) in views.iter_mut() {
        let view_matrix = view.transform.compute_matrix();
        let view_row_2 = view_matrix.row(2);
        for (entity, mesh_handle, mesh_uniform, is_red) in material_meshes.iter() {
            if let Some(mesh) = render_meshes.get(mesh_handle) {
                let key = key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
                let pipeline =
                    pipelines.specialize(&mut pipeline_cache, &custom_pipeline, (*is_red, key));
                transparent_phase.add(Transparent3d {
                    entity,
                    pipeline,
                    draw_function: draw_custom,
                    distance: view_row_2.dot(mesh_uniform.transform.col(3)),
                });
            }
        }
    }
}
