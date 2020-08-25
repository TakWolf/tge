use tge::prelude::*;

const TITLE: &str = "Text Layout";

struct App {
    font: Font,
    text: String,
    text_size: f32,
    horizontal_gravity: TextLayoutGravity,
    vertical_gravity: TextLayoutGravity,
    show_background: bool,
}

impl App {
    fn new(engine: &mut Engine) -> GameResult<Self> {
        let font = Font::load(engine, "assets/SourceHanSansSC/SourceHanSansSC-Regular.otf")?;
        Ok(Self {
            font,
            text: "⇦, ⇨, ⇧ and ⇩ to change layout gravity\n'+' and '-' to change text size\n'Num0' to change background visibility\nInput something here...".to_owned(),
            text_size: 24.0,
            horizontal_gravity: TextLayoutGravity::default(),
            vertical_gravity: TextLayoutGravity::default(),
            show_background: false,
        })
    }
}

impl Game for App {
    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        if engine.keyboard().is_key_down(KeyCode::Left) {
            match self.horizontal_gravity {
                TextLayoutGravity::Center => self.horizontal_gravity = TextLayoutGravity::Start,
                TextLayoutGravity::End => self.horizontal_gravity = TextLayoutGravity::Center,
                _ => (),
            }
        }
        if engine.keyboard().is_key_down(KeyCode::Right) {
            match self.horizontal_gravity {
                TextLayoutGravity::Center => self.horizontal_gravity = TextLayoutGravity::End,
                TextLayoutGravity::Start => self.horizontal_gravity = TextLayoutGravity::Center,
                _ => (),
            }
        }
        if engine.keyboard().is_key_down(KeyCode::Up) {
            match self.vertical_gravity {
                TextLayoutGravity::Center => self.vertical_gravity = TextLayoutGravity::Start,
                TextLayoutGravity::End => self.vertical_gravity = TextLayoutGravity::Center,
                _ => (),
            }
        }
        if engine.keyboard().is_key_down(KeyCode::Down) {
            match self.vertical_gravity {
                TextLayoutGravity::Center => self.vertical_gravity = TextLayoutGravity::End,
                TextLayoutGravity::Start => self.vertical_gravity = TextLayoutGravity::Center,
                _ => (),
            }
        }
        if engine.keyboard().is_key_down(KeyCode::Equals) {
            self.text_size += 1.0;
        }
        if engine.keyboard().is_key_down(KeyCode::Minus) {
            self.text_size -= 1.0;
        }
        if engine.keyboard().is_key_down(KeyCode::Num0) {
            self.show_background = !self.show_background;
        }

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::WHITE);

        let graphics_size = engine.graphics().size();

        if self.show_background {
            for x in 0..(graphics_size.width / self.text_size).ceil() as i32 {
                for y in 0..(graphics_size.height / self.text_size).ceil() as i32 {
                    engine.graphics().draw_sprite(
                        TextureRef::None,
                        SpriteDrawParams::default()
                            .region((0.0, 0.0, self.text_size, self.text_size))
                            .color(if (x + y) % 2 == 0 {
                                Color::from_u32(0xffb8b8ff)
                            } else {
                                Color::from_u32(0xa8e6ffff)
                            }),
                        Transform::default()
                            .translate((x as f32 * self.text_size, y as f32 * self.text_size)),
                    );
                }
            }
        }

        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(self.text_size)
                .wrap_width(graphics_size.width)
                .wrap_height(graphics_size.height)
                .horizontal_gravity(self.horizontal_gravity)
                .vertical_gravity(self.vertical_gravity)
                .color(Color::BLACK),
            None,
        );

        Ok(())
    }

    fn event(&mut self, _: &mut Engine, event: Event) -> GameResult<bool> {
        match event {
            Event::ReceiveChar(character) => {
                if !character.is_control() {
                    self.text.push(character);
                }
            }
            Event::KeyboardInput { key, action, .. } => {
                if action == KeyAction::Down {
                    match key {
                        KeyCode::Enter => {
                            self.text.push('\n');
                        }
                        KeyCode::Backspace => {
                            self.text.pop();
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        Ok(false)
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
