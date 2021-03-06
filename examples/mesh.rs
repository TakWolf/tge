use tge::prelude::*;

const TITLE: &str = "Mesh";

struct App {
    texture: Texture,
    angle: Angle,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let texture = Texture::load(engine, "assets/sky.png")?;
        Ok(Self {
            texture,
            angle: Angle::zero(),
        })
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        self.angle += Angle::radians(engine.timer().delta_time().as_secs_f32() / 2.0);

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::WHITE);

        engine.graphics().draw_mesh(
            &self.texture,
            MeshDrawParams::default()
                .primitive(PrimitiveType::Triangles)
                .vertices(vec![
                    Vertex {
                        position: Position::new(-100.0, 0.0),
                        uv: Vector::new(0.0, 0.0),
                        color: Color::WHITE,
                    },
                    Vertex {
                        position: Position::new(-100.0, 200.0),
                        uv: Vector::new(0.0, 1.0),
                        color: Color::GREEN,
                    },
                    Vertex {
                        position: Position::new(100.0, 0.0),
                        uv: Vector::new(1.0, 0.0),
                        color: Color::BLUE,
                    },
                ]),
            Transform::default()
                .scale((1.2, 0.5))
                .rotate(self.angle)
                .translate((400.0, 300.0)),
        );

        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((1024.0, 600.0)))
        .build()?
        .run_with(App::new)
}
