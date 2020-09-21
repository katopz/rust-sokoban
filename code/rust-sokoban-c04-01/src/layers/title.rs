use super::{
    game::GameLayer,
    layer::{InputEvent, Layer},
};
use ggez::graphics;
use ggez::{event::KeyCode, Context, GameResult};

pub struct TitleLayer {}

impl TitleLayer {
    pub fn new() -> TitleLayer {
        TitleLayer {}
    }
}

impl Layer for TitleLayer {
    fn update(&mut self, context: &mut Context) -> GameResult<Option<Box<dyn Layer>>> {
        Ok(None)
    }

    fn draw(&self, context: &mut Context) -> GameResult<()> {
        // Clearing the screen (this gives us the backround colour)
        graphics::clear(context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // renderer::draw_text(context, &String::from("Sokoban"), 50.0, (100.0, 100.0));

        // renderer::draw_text(
        //     context,
        //     &String::from("Press space to play"),
        //     20.0,
        //     (100.0, 250.0),
        // );

        // renderer::draw_text(
        //     context,
        //     &String::from("Press escape to quit"),
        //     20.0,
        //     (100.0, 300.0),
        // );

        Ok(graphics::present(context)?)
    }

    fn on_input_event(
        &mut self,
        context: &mut Context,
        input_event: InputEvent,
    ) -> GameResult<Option<Box<dyn Layer>>> {
        match input_event {
            InputEvent::KeyDown { keycode } => match keycode {
                KeyCode::Escape => {
                    ggez::event::quit(context);
                    Ok(None)
                }
                KeyCode::Space => Ok(Some(Box::new(GameLayer::new(context)))),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}
