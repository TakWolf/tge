use tge::prelude::*;

const TITLE: &str = "Parallax Forest";

struct App {
    view_size: Size,
    scene_size: Size,
    canvas: Canvas,
    texture_back_trees: Texture,
    texture_lights: Texture,
    texture_middle_trees: Texture,
    texture_front_trees: Texture,
    camera: Position,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let view_size = Size::<f32>::new(272.0, 160.0);
        let scene_size = Size::new(1000.0, 160.0);
        let canvas = Canvas::new(engine.graphics(), Size::new(view_size.width.round() as u32, view_size.height.round() as u32))?;
        let texture_back_trees = Texture::load(engine, "assets/parallax-forest/back-trees.png")?;
        let texture_lights = Texture::load(engine, "assets/parallax-forest/lights.png")?;
        let texture_middle_trees = Texture::load(engine, "assets/parallax-forest/middle-trees.png")?;
        let texture_front_trees = Texture::load(engine, "assets/parallax-forest/front-trees.png")?;
        let camera = Position::new(scene_size.width / 2.0, scene_size.height / 2.0);
        Ok(Self {
            view_size,
            scene_size,
            canvas,
            texture_back_trees,
            texture_lights,
            texture_middle_trees,
            texture_front_trees,
            camera,
        })
    }

    fn draw_scene(&mut self, engine: &mut Engine) {
        let offset = Position::new(
            self.view_size.width / 2.0 - self.camera.x,
            self.view_size.height / 2.0 - self.camera.y,
        );

        engine.graphics().draw_sprite(
            &self.texture_back_trees,
            SpriteDrawParams::default()
                .region((0.0, 0.0, 1000.0, 160.0)),
            Transform::default()
                .translate((offset.x / 8.0, 0.0)),
        );
        engine.graphics().draw_sprite(
            &self.texture_lights,
            SpriteDrawParams::default()
                .region((0.0, 0.0, 1000.0, 160.0)),
            Transform::default()
                .translate((offset.x / 4.0, 0.0)),
        );
        engine.graphics().draw_sprite(
            &self.texture_middle_trees,
            SpriteDrawParams::default()
                .region((0.0, 0.0, 1000.0, 160.0)),
            Transform::default()
                .translate((offset.x / 2.0, 0.0)),
        );
        engine.graphics().draw_sprite(
            &self.texture_front_trees,
            SpriteDrawParams::default()
                .region((0.0, 0.0, 1000.0, 160.0)),
            Transform::default()
                .translate((offset.x, 0.0)),
        );
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        if let Some(mouse_position) = engine.mouse().position() {
            let graphics_size = engine.graphics().size();
            let speed = (mouse_position.x - graphics_size.width / 2.0) * (self.view_size.width / graphics_size.width);
            let delta_time = engine.timer().delta_time().as_secs_f32();
            self.camera.x += speed * delta_time;
            if self.camera.x < self.view_size.width / 2.0 {
                self.camera.x = self.view_size.width / 2.0;
            }
            if self.camera.x > self.scene_size.width - self.view_size.width / 2.0 {
                self.camera.x = self.scene_size.width - self.view_size.width / 2.0;
            }
        }

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::BLACK);

        engine.graphics().set_canvas(Some(&self.canvas));
        engine.graphics().clear(Color::BLACK);
        self.draw_scene(engine);
        engine.graphics().set_canvas(None);

        let graphics_size = engine.graphics().size();
        let position;
        let scale;
        if graphics_size.width / graphics_size.height <= self.view_size.width / self.view_size.height {
            scale = graphics_size.width / self.view_size.width;
            position = Position::new(0.0, (graphics_size.height - self.view_size.height * scale) / 2.0);
        } else {
            scale = graphics_size.height / self.view_size.height;
            position = Position::new((graphics_size.width - self.view_size.width * scale) / 2.0, 0.0);
        }

        engine.graphics().draw_sprite(
            &self.canvas,
            None,
            Transform::default()
                .scale((scale, scale))
                .translate(position),
        );

        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((272.0 * 2.0, 160.0 * 2.0)))
        .build()?
        .run_with(App::new)
}
