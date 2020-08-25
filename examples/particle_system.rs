use tge::prelude::*;

const TITLE: &str = "Particle System";

struct ParticleSystem {



}

impl ParticleSystem {




}

struct App {
    texture_particle: Texture,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let texture_particle = Texture::load(engine, "assets/particle.png")?;
        Ok(Self {
            texture_particle,
        })
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        // TODO

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLACK);

        // TODO

        engine.graphics().draw_sprite(
            &self.texture_particle,
            None,
            None,
        );

        // TODO

        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((800.0, 600.0)))
        .build()?
        .run_with(App::new)
}
