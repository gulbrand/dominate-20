use amethyst::utils::application_root_dir;
use amethyst::{ GameDataBuilder, DataInit, SimpleState, Application };
use amethyst::renderer::{ RenderingBundle };
use amethyst::renderer::plugins::{ RenderFlat2D, RenderToWindow };
use amethyst::renderer::types::{ DefaultBackend };

pub struct SimpleWindow;

impl SimpleState for SimpleWindow {}

pub fn main() -> amethyst::Result<()> {
    println!("simple_window");
    let app_root = application_root_dir()?;
    amethyst::start_logger(Default::default());
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");

    println!("{:?}", app_root);
    println!("{:?}", display_config_path);
    println!("{:?}", assets_dir);
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.98, 0.98, 0.96, 1.0]),

                )
                .with_plugin(RenderFlat2D::default()),
        )?
        ;
    let mut game =
        Application::new(assets_dir, SimpleWindow, game_data)?;
    game.run();

    Ok(())
}