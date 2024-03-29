use tge::prelude::*;

const TITLE: &str = "Stroke Text";

struct App {
    font: Font,
    text: String,
    text_size: f32,
    position: Position,
    angle: Angle,
    color: Color,
    stroke_color: Color,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let font = Font::load(engine, "assets/ark-pixel-font/ark-pixel-12px-zh_cn.otf")?;
        let text = "Hello, world!".to_owned();
        let text_size = 60.0;
        let position = Position::new(512.0, 300.0);
        let angle = Angle::zero();
        let color = Color::RED;
        let stroke_color = Color::WHITE;
        Ok(Self {
            font,
            text,
            text_size,
            position,
            angle,
            color,
            stroke_color,
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
        engine.graphics().clear(Color::BLACK);

        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .horizontal_gravity(TextLayoutGravity::Center)
                .vertical_gravity(TextLayoutGravity::Center)
                .color(self.stroke_color),
            Transform::default()
                .rotate(self.angle)
                .translate((self.position.x - 2.0, self.position.y)),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .horizontal_gravity(TextLayoutGravity::Center)
                .vertical_gravity(TextLayoutGravity::Center)
                .color(self.stroke_color),
            Transform::default()
                .rotate(self.angle)
                .translate((self.position.x - 2.0, self.position.y - 2.0)),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .horizontal_gravity(TextLayoutGravity::Center)
                .vertical_gravity(TextLayoutGravity::Center)
                .color(self.stroke_color),
            Transform::default()
                .rotate(self.angle)
                .translate((self.position.x, self.position.y - 2.0)),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .horizontal_gravity(TextLayoutGravity::Center)
                .vertical_gravity(TextLayoutGravity::Center)
                .color(self.stroke_color),
            Transform::default()
                .rotate(self.angle)
                .translate((self.position.x + 2.0, self.position.y - 2.0)),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .horizontal_gravity(TextLayoutGravity::Center)
                .vertical_gravity(TextLayoutGravity::Center)
                .color(self.stroke_color),
            Transform::default()
                .rotate(self.angle)
                .translate((self.position.x + 2.0, self.position.y)),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .horizontal_gravity(TextLayoutGravity::Center)
                .vertical_gravity(TextLayoutGravity::Center)
                .color(self.stroke_color),
            Transform::default()
                .rotate(self.angle)
                .translate((self.position.x + 2.0, self.position.y + 2.0)),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .horizontal_gravity(TextLayoutGravity::Center)
                .vertical_gravity(TextLayoutGravity::Center)
                .color(self.stroke_color),
            Transform::default()
                .rotate(self.angle)
                .translate((self.position.x, self.position.y + 2.0)),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .horizontal_gravity(TextLayoutGravity::Center)
                .vertical_gravity(TextLayoutGravity::Center)
                .color(self.stroke_color),
            Transform::default()
                .rotate(self.angle)
                .translate((self.position.x - 2.0, self.position.y + 2.0)),
        );
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .horizontal_gravity(TextLayoutGravity::Center)
                .vertical_gravity(TextLayoutGravity::Center)
                .color(self.color),
            Transform::default()
                .rotate(self.angle)
                .translate(self.position),
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
