use super::layer::{InputEvent, Layer};
use crate::{
    audio::initialize_sounds, components::register_components, map::load_map,
    resources::register_resources, resources::InputQueue, resources::Time, systems::*,
};
use ggez::{graphics, timer, Context, GameResult};
use specs::{RunNow, World, WorldExt};

pub struct GameLayer {
    world: World,
}

impl GameLayer {
    pub fn new(context: &mut Context) -> GameLayer {
        let mut world = World::new();
        register_components(&mut world);
        register_resources(&mut world);
        initialize_level(&mut world);

        initialize_sounds(&mut world, context);

        GameLayer { world }
    }
}

impl Layer for GameLayer {
    fn update(&mut self, context: &mut Context) -> GameResult<Option<Box<dyn Layer>>> {
        // Run input system
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        // Run gameplay state system
        {
            let mut gss = GameplayStateSystem {};
            gss.run_now(&self.world);
        }

        // Get and update time resource
        {
            let mut time = self.world.write_resource::<Time>();
            time.delta += timer::delta(context);
        }

        // Run event system
        {
            let mut es = EventSystem {};
            es.run_now(&self.world);
        }

        Ok(None)
    }

    fn draw(&self, context: &mut Context) -> GameResult<()> {
        // Clearing the screen (this gives us the backround colour)
        graphics::clear(context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Render game entities
        {
            let mut rs = RenderingSystem { context };
            rs.run_now(&self.world);
        }

        Ok(graphics::present(context)?)
    }

    fn on_input_event(
        &mut self,
        context: &mut Context,
        input_event: InputEvent,
    ) -> GameResult<Option<Box<dyn Layer>>> {
        match input_event {
            InputEvent::KeyDown { keycode } => {
                println!("Key pressed: {:?}", keycode);

                let mut input_queue = self.world.write_resource::<InputQueue>();
                input_queue.keys_pressed.push(keycode);
            }
            _ => (),
        }
        Ok(None)
    }
}

// Initialize the level
pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . RB . . . W 
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    load_map(world, MAP.to_string());
}
