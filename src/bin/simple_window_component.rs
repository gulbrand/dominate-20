use std::borrow::BorrowMut;

use amethyst::{Application, DataInit, GameData, GameDataBuilder, SimpleState, StateData};
use amethyst::core::transform::{Transform, TransformBundle};
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::StringBindings;
use amethyst::prelude::{Builder, World, WorldExt};
use amethyst::renderer::{Camera, RenderingBundle};
use amethyst::renderer::plugins::{RenderFlat2D, RenderToWindow};
use amethyst::renderer::types::DefaultBackend;
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::utils::application_root_dir;

pub struct SimpleWindow;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub struct Thing;

impl Thing {
    pub fn new() -> Thing {
        Thing
    }
}

impl Component for Thing {
    type Storage = DenseVecStorage<Self>;
}


fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn init_thing(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);
    world
        .create_entity()
        .with(Thing)
        .with(transform)
        .build();
}

impl SimpleState for SimpleWindow {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("SimpleState for SimpleWindow :: on_start");
        let world = data.world;
        world.register::<Thing>();
        init_camera(world);
        init_thing(world);
    }
}

pub fn main() -> amethyst::Result<()> {
    println!("simple_window");
    let app_root = application_root_dir()?;
    amethyst::start_logger(Default::default());
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");

    println!("{:?}", app_root);
    println!("{:?}", display_config_path);
    println!("{:?}", assets_dir);
    let clear_color = [0.98, 0.98, 0.96, 1.0];
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        // .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear(clear_color),
                )
                .with_plugin(RenderFlat2D::default())
                // .with_plugin(RenderUi::default())
            ,
        )?
        ;
    let mut game =
        Application::new(assets_dir, SimpleWindow, game_data)?;
    game.run();

    Ok(())
}