use ggez::{event::KeyCode, Context, GameResult};

// Input event
pub enum InputEvent {
    KeyDown { keycode: KeyCode },
    MouseMotion { x: f32, y: f32 },
    MouseButtonDown {},
}

// A layer is a basic trait that can be used to represent a
// layer of the game. Layers can be simple layers, like UI-only
// or complex, like a game + UI layer on top.
pub trait Layer {
    // update the layer, this can also transition to a new layer
    fn update(&mut self, context: &mut Context) -> GameResult<Option<Box<dyn Layer>>>;

    // draw the layer, this should not transition
    fn draw(&self, context: &mut Context) -> GameResult<()>;

    // consume input events, this can also transition to a new layer
    fn on_input_event(
        &mut self,
        context: &mut Context,
        input_event: InputEvent,
    ) -> GameResult<Option<Box<dyn Layer>>>;
}
