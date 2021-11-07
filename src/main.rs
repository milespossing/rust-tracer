mod rays;
mod position_system;

use amethyst::{
    prelude::*, 
    renderer::{
        types::DefaultBackend,
        RenderingBundle,
        plugins::{RenderToWindow,RenderPbr3D},
    },
    window::DisplayConfig,
    utils::application_root_dir,
    ecs::{prelude::*, WorldExt, Entity},
    core::math::Vector3,
};

use position_system::PositionSystem;


struct RayTracer {
    entity1: Option<Entity>,
    entity2: Option<Entity>,
}

impl SimpleState for RayTracer {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.entity1 = Some(world
            .create_entity()
            .with(rays::Ray{
                pos: Vector3::new(0_f32,0_f32,0_f32), 
                vel: Vector3::new(1_f32,0_f32,0_f32)
            })
            .build());
        self.entity2 = Some(world
            .create_entity()
            .with(rays::Ray{
                pos: Vector3::new(0_f32,0_f32,0_f32), 
                vel: Vector3::new(0_f32,0.5_f32,0.5_f32)
            })
            .build());
        println!("Begin!");
    }

    fn on_stop(&mut self, _: StateData<'_, GameData<'_, '_>>) {
        println!("End!");
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let storage = data.world.read_storage::<rays::Ray>();
        let ray1 = storage.get(self.entity1.expect("Entity was not loaded")).expect("Failed to get entity");
        let ray2 = storage.get(self.entity2.expect("Entity was not loaded")).expect("Failed to get entity");
        println!("Ray1: {:?}", ray1.pos);
        println!("Ray2: {:?}", ray2.pos);
        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    let display_config = DisplayConfig {
        title: "Amethyst".to_string(),
        dimensions: Some((1024, 720)),
        ..Default::default()
    };
    amethyst::start_logger(Default::default());
    let assets_dir = application_root_dir()?.join("examples/hello_world/assets");
    let game_data = GameDataBuilder::default()
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(RenderToWindow::from_config(display_config)
                .with_clear([0.1,0.1,0.1,1.0]),)
            .with_plugin(RenderPbr3D::default()))?
        .with(PositionSystem, "Position System", &[]);
    let mut game = Application::new(assets_dir, RayTracer {entity1: None, entity2: None}, game_data)?;
    game.run();

    Ok(())
}
