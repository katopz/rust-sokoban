use ggez;
use ggez::event::KeyCode;
use ggez::event::KeyMods;
use ggez::{conf, event, Context, GameResult};
use layers::{layer::InputEvent, layer::Layer, title::TitleLayer};
use std::path;

mod audio;
mod components;
mod constants;
mod entities;
mod events;
mod layers;
mod map;
mod resources;
mod systems;

struct Game<'a> {
    current_layer: Box<dyn Layer + 'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game {
            current_layer: Box::new(TitleLayer::new()),
        }
    }
}

impl<'a> event::EventHandler for Game<'a> {
    fn update(&mut self, context: &mut Context) -> GameResult {
        // Change the state to a new one if told by the current layer
        match self.current_layer.update(context)? {
            Some(new_state) => self.current_layer = new_state,
            None => (),
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        self.current_layer.draw(context)
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        // Change the state to a new one if told by the current layer
        match self
            .current_layer
            .on_input_event(context, InputEvent::KeyDown { keycode })
        {
            Ok(Some(new_state)) => self.current_layer = new_state,
            Ok(None) => (),
            Err(_) => panic!("key_down_event failed"),
        }
    }
}

pub fn main() -> GameResult {
    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = &mut context_builder.build()?;
    // Create the game state
    let game = &mut Game::new();
    // Run the main event loop
    event::run(context, event_loop, game)
}
