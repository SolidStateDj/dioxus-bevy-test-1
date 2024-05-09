#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

use bevy::{
    app::ScheduleRunnerPlugin,
    log::LogPlugin,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    utils::Duration,
};

use bevy_image_export::*;

use std::f32::consts::PI;

use std::thread;

fn main() {
    thread::spawn(|| {
        // let export_plugin = ImageExportPlugin::default();
        // let export_threads = export_plugin.threads.clone();

        App::new()
            // .add_plugins(DefaultPlugins)
            // .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
            .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
            // .add_plugins(export_plugin)
            // .add_plugins(plugins)
            // .add_systems(Startup, setup)
            .add_systems(Update, hello_world_system)
            .run();
        // export_threads.finish();
        //
        App::new()
            .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
                Duration::from_secs_f64(1.0 / 60.0),
            )))
            .add_systems(Update, counter)
            .run();
    });

    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    dioxus::launch(App);
}

fn hello_world_system() {
    println!("hello world");
}

fn counter(mut state: Local<CounterState>) {
    let mut counter = use_signal(|| 0);
    if state.count % 60 == 0 {
        println!("{}", state.count);
    }
    state.count += 1;
    counter += 1;
}

#[derive(Default, PartialEq, Props, Clone)]
struct CounterState {
    count: u32,
}

fn App() -> Element {
    rsx! {
        Counting { count: 0 }
    }
}

fn Counting(props: CounterState) -> Element {
    rsx! {
        b { "{props.count}" }
    }
}

#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 12.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let debug_material = materials.add(StandardMaterial {
    //     base_color_texture: Some(images.add(uv_debug_texture())),
    //     ..default()
    // });

    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                // material: debug_material.clone(),
                transform: Transform::from_xyz(
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    2.0,
                    0.0,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..Default::default()
    });
}
