use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::event;

struct GameState{
    x: f32,
    y: f32,
    speed: f32,
    direction: f32
}
// new function an instance of GameState struct

impl GameState {
    fn new() -> GameState {
        GameState {
            x: 0.0,
            y: 0.0,
            speed: 1.0,
            direction: 0.0
        }
    }
}

//implementation for the eventHandler trait for the GameState struct
// trait provides methods that are called by ggez library to update and draw the game

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.x += self.speed * self.direction.cos();
        self.y += self.speed * self.direction.sin();
        Ok(())
    }

    //update x and y coordinates of the circle based on its speed and direction
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.1,0.2,0.3,1.0].into());
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [self.x, self.y],
            50.0,
            2.0,
            graphics::Color::new(0.4,0.5,0.6,1.0),
        )?;
        graphics::draw(ctx,&circle, ([0.0,0.0],))?;
        graphics::present(ctx)?;
        Ok(())
    }

    //implementation of the draw method for the GameState struct
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods,
        _repeat: bool,
    ){
        // ipmplementation of the key_down_event method of the GameState struct
        // method called by ggez library when a key is pressed
        // updates the direction of the circle based of the key that was pressed

        match keycode {
            event::KeyCode::Up => self.direction = 0.0,
            event::KeyCode::Down => self.direction = std::f32::consts::PI,
            event::KeyCode::Left => self.direction = std::f32::consts::FRAC_PI_2 * 3.0,
            event::KeyCode::Right => self.direction = std::f32::consts::FRAC_PI_2,
            _=> (),

        }
    }
}
// main function, creates a new GameState instance and passes it to the event::run
// function which starts the game loop

 fn main() -> GameResult<()> {
    let cb = ggez::ContextBuilder::new("my_game", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new();
    event::run(ctx, event_loop, state)
 }