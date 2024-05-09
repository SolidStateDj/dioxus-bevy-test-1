#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

// use bevy_image_export::*;

// use std::f32::consts::PI;

// use bevy::{
//     prelude::*,
//     render::{
//         render_resource::{
//             Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
//         },
//         view::RenderLayers,
//     },
//     text::{BreakLineOn, Text2dBounds},
// };
// use iyes_perf_ui::prelude::*;

fn main() {
    // let export_plugin = ImageExportPlugin::default();
    // let export_threads = export_plugin.threads.clone();

    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);

    // App::new()
    //     // .add_plugins(DefaultPlugins.set(WindowPlugin {
    //     //     primary_window: Some(Window {
    //     //         // provide the ID selector string here
    //     //         canvas: Some("#mygame-canvas".into()),
    //     //         // ... any other window properties ...
    //     //         ..default()
    //     //     }),
    //     //     ..default()
    //     // }))
    //     .add_plugins(DefaultPlugins)
    //     // Performance Overlay
    //     .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
    //     .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    //     .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
    //     .add_plugins(PerfUiPlugin)
    //     .add_plugins(export_plugin)
    //     // Systems
    //     // .add_systems(Startup, setup)
    //     // .add_systems(Update, bevy::window::close_on_esc)
    //     .add_systems(Startup, gen_texture)
    //     .run();

    // launch(App);
    // export_threads.finish();
}

// Marks the first pass cube (rendered to a texture.)
// #[derive(Component)]
// struct FirstPassCube;
//
// // Marks the main pass cube, to which the texture is applied.
// #[derive(Component)]
// struct MainPassCube;

fn App() -> Element {
    // Build cool things ✌️
    //
    // let ws: Coroutine<()> = use_coroutine(|rx| async move {
    //     println!("Hello World!");
    // });

    rsx! {
        div { "Hello World" }
        // ImageTest {}
    }
}

// #[server(PostServerData)]
// async fn post_server_data(data: String) -> Result<(), ServerFnError> {
//     info!("Server received: {}", data);
//     Ok(())
// }
//
// #[server(GetServerData)]
// async fn get_server_data() -> Result<String, ServerFnError> {
//     Ok("Hello from the server!".to_string())
// }

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     // let texture_handle: Handle<Image> = asset_server.load("Fear.png");
//     let font = asset_server.load("fonts/FiraSans-Bold.ttf");
//
//     // commands.spawn(Camera2dBundle::default());
//     commands.spawn(SpriteSheetBundle {
//         texture: asset_server.load("Fear.png"),
//         ..default()
//     });
//     // commands.spawn(Text2dBundle {
//     //     text: Text::from_section("rotation", text_style.clone()),
//     //     transform: Transform::from_translation(250. * Vec3::Y),
//     //     ..default()
//     // });
//
//     // commands.spawn(PerfUiCompleteBundle::default());
//     // commands.spawn((
//     //     PerfUiRoot::default(),
//     //     PerfUiEntryFPS::default(),
//     //     // PerfUiEntryClock::default(),
//     // ));
//     //
//     let slightly_smaller_text_style = TextStyle {
//         font,
//         font_size: 42.0,
//         color: Color::WHITE,
//     };
//
//     let box_size = Vec2::new(300.0, 200.0);
//     let box_position = Vec2::new(0.0, -250.0);
//
//     commands
//         .spawn(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::rgb(0.25, 0.25, 0.75),
//                 custom_size: Some(Vec2::new(box_size.x, box_size.y)),
//                 ..default()
//             },
//             transform: Transform::from_translation(box_position.extend(0.0)),
//             ..default()
//         })
//         .with_children(|builder| {
//             builder.spawn(Text2dBundle {
//                 text: Text {
//                     sections: vec![TextSection::new(
//                         "this text wraps in the box\n(Unicode linebreaks)",
//                         slightly_smaller_text_style.clone(),
//                     )],
//                     justify: JustifyText::Left,
//                     linebreak_behavior: BreakLineOn::WordBoundary,
//                 },
//                 text_2d_bounds: Text2dBounds {
//                     // Wrap text in the rectangle
//                     size: box_size,
//                 },
//                 // ensure the text is drawn on top of the box
//                 transform: Transform::from_translation(Vec3::Z),
//                 ..default()
//             });
//         });
// }
//
// fn gen_texture(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut images: ResMut<Assets<Image>>,
//     mut export_sources: ResMut<Assets<ImageExportSource>>,
// ) {
//     let size = Extent3d {
//         width: 512,
//         height: 512,
//         ..default()
//     };
//
//     let mut image = Image {
//         texture_descriptor: TextureDescriptor {
//             label: None,
//             size,
//             dimension: TextureDimension::D2,
//             format: TextureFormat::Bgra8UnormSrgb,
//             mip_level_count: 1,
//             sample_count: 1,
//             usage: TextureUsages::TEXTURE_BINDING
//                 | TextureUsages::COPY_DST
//                 | TextureUsages::COPY_SRC
//                 | TextureUsages::RENDER_ATTACHMENT,
//             view_formats: &[],
//         },
//         ..default()
//     };
//
//     image.resize(size);
//
//     let image_handle = images.add(image);
//
//     let cube_handle = meshes.add(Cuboid::new(4.0, 4.0, 4.0));
//     let cube_material_handle = materials.add(StandardMaterial {
//         base_color: Color::rgb(0.8, 0.7, 0.6),
//         reflectance: 0.02,
//         unlit: false,
//         ..default()
//     });
//
//     let first_pass_layer = RenderLayers::layer(1);
//
//     commands.spawn((
//         PbrBundle {
//             mesh: cube_handle,
//             material: cube_material_handle,
//             transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
//             ..default()
//         },
//         FirstPassCube,
//         first_pass_layer,
//     ));
//
//     commands.spawn((
//         PointLightBundle {
//             transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
//             ..default()
//         },
//         RenderLayers::all(),
//     ));
//
//     commands.spawn((
//         Camera3dBundle {
//             camera: Camera {
//                 // render before the "main pass" camera
//                 order: -1,
//                 target: image_handle.clone().into(),
//                 clear_color: Color::WHITE.into(),
//                 ..default()
//             },
//             transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
//                 .looking_at(Vec3::ZERO, Vec3::Y),
//             ..default()
//         },
//         first_pass_layer,
//     ));
//
//     let cube_size = 4.0;
//     let cube_handle = meshes.add(Cuboid::new(cube_size, cube_size, cube_size));
//
//     // This material has the texture that has been rendered.
//     let material_handle = materials.add(StandardMaterial {
//         base_color_texture: Some(image_handle.clone()),
//         reflectance: 0.02,
//         unlit: false,
//         ..default()
//     });
//
//     // Main pass cube, with material containing the rendered first pass texture.
//     commands.spawn((
//         PbrBundle {
//             mesh: cube_handle,
//             material: material_handle,
//             transform: Transform::from_xyz(0.0, 0.0, 1.5)
//                 .with_rotation(Quat::from_rotation_x(-PI / 5.0)),
//             ..default()
//         },
//         MainPassCube,
//     ));
//
//     // The main pass camera.
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     });
//
//     commands.spawn(ImageExportBundle {
//         source: export_sources.add(image_handle),
//         settings: ImageExportSettings {
//             // Frames will be saved to "./out/[#####].png".
//             output_dir: "out".into(),
//             // Choose "exr" for HDR renders.
//             extension: "png".into(),
//         },
//     });
// }

// #[component]
// fn ImageTest() -> Element {
//     rsx! {
//         div { "Hello World!" }
//         img { src: "Fear.png", id: "test" }
//         // canvas { height: "720", width: "1280", id: "mygame-canvas" }
//     }
// }
