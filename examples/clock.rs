use tge::prelude::*;
use chrono::{Local, Timelike};

struct App {
    texture_clock_disk: Texture,
    hour_angle: Angle,
    minute_angle: Angle,
    second_angle: Angle,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let texture_clock_disk = Texture::load(engine, "assets/clock-disk.png")?;
        Ok(Self {
            texture_clock_disk,
            hour_angle: Angle::zero(),
            minute_angle: Angle::zero(),
            second_angle: Angle::zero(),
        })
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let now_time = Local::now().time();

        let title = format!("{} - FPS: {}", now_time.format("%H:%M:%S"), engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        self.second_angle = Angle::degrees(now_time.second() as f32 * 6.0);
        self.minute_angle = Angle::degrees(now_time.minute() as f32 * 6.0 + now_time.second() as f32 / 10.0);
        self.hour_angle = Angle::degrees(now_time.hour() as f32 * 30.0 + now_time.minute() as f32 / 2.0 + now_time.second() as f32 / 120.0);

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear((0.9, 0.9, 0.9, 1.0));

        engine.graphics().draw_sprite(
            &self.texture_clock_disk,
            None,
            None,
        );
        engine.graphics().draw_sprite(
            TextureRef::None,
            SpriteDrawParams::default()
                .region((0.0, 0.0, 30.0, 200.0))
                .origin((15.0, 190.0))
                .color((0.2, 0.2, 1.0, 1.0)),
            Transform::default()
                .rotate(self.hour_angle)
                .translate((300.0, 300.0)),
        );
        engine.graphics().draw_sprite(
            TextureRef::None,
            SpriteDrawParams::default()
                .region((0.0, 0.0, 20.0, 240.0))
                .origin((10.0, 220.0))
                .color((0.2, 1.0, 0.2, 1.0)),
            Transform::default()
                .rotate(self.minute_angle)
                .translate((300.0, 300.0)),
        );
        engine.graphics().draw_sprite(
            TextureRef::None,
            SpriteDrawParams::default()
                .region((0.0, 0.0, 10.0, 270.0))
                .origin((5.0, 240.0))
                .color((1.0, 0.2, 0.2, 1.0)),
            Transform::default()
                .rotate(self.second_angle)
                .translate((300.0, 300.0)),
        );
        engine.graphics().draw_sprite(
            TextureRef::None,
            SpriteDrawParams::default()
                .region((0.0, 0.0, 6.0, 6.0))
                .origin((3.0, 3.0))
                .color(Color::BLACK),
            Transform::default()
                .translate((300.0, 300.0)),
        );

        Ok(())
    }
}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .inner_size((600.0, 600.0))
            .resizable(false))
        .build()?
        .run_with(App::new)
}
