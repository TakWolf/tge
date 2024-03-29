use crate::error::{GameError, GameResult};
use crate::math::Vector;
use crate::event::{Event, KeyAction};
use crate::filesystem::{Filesystem, FilesystemConfig};
use crate::window::{Window, WindowConfig, LogicalPosition, LogicalSize};
use crate::graphics::{Graphics, GraphicsConfig};
use crate::timer::{Timer, TimerConfig};
use crate::keyboard::{Keyboard, KeyboardConfig};
use crate::mouse::{Mouse, MouseConfig};
use crate::touch::{Touch, TouchConfig};
use crate::touchpad::{Touchpad, TouchpadConfig};
use crate::gamepad::{Gamepad, GamepadConfig};
use crate::audio::{Audio, AudioConfig};
use crate::game::Game;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{StartCause, WindowEvent, MouseScrollDelta};
use winit::platform::run_return::EventLoopExtRunReturn;

#[derive(Debug)]
enum State {
    Ready,
    Running,
    Finished,
    Broken(Option<GameError>),
}

pub struct Engine {
    event_loop: Option<EventLoop<()>>,
    filesystem: Filesystem,
    window: Window,
    graphics: Graphics,
    timer: Timer,
    keyboard: Keyboard,
    mouse: Mouse,
    touch: Touch,
    touchpad: Touchpad,
    gamepad: Gamepad,
    audio: Audio,
    state: State,
}

impl Engine {
    pub fn filesystem(&mut self) -> &mut Filesystem {
        &mut self.filesystem
    }

    pub fn window(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn graphics(&mut self) -> &mut Graphics {
        &mut self.graphics
    }

    pub fn timer(&mut self) -> &mut Timer {
        &mut self.timer
    }

    pub fn keyboard(&mut self) -> &mut Keyboard {
        &mut self.keyboard
    }

    pub fn mouse(&mut self) -> &mut Mouse {
        &mut self.mouse
    }

    pub fn touch(&mut self) -> &mut Touch {
        &mut self.touch
    }

    pub fn touchpad(&mut self) -> &mut Touchpad {
        &mut self.touchpad
    }

    pub fn gamepad(&mut self) -> &mut Gamepad {
        &mut self.gamepad
    }

    pub fn audio(&mut self) -> &mut Audio {
        &mut self.audio
    }

    pub fn quit(&mut self) {
        match &self.state {
            State::Finished | State::Broken(_) => (),
            _ => self.state = State::Finished,
        }
    }

    pub fn exit(&mut self, error: GameError) {
        match &self.state {
            State::Finished | State::Broken(_) => (),
            _ => self.state = State::Broken(Some(error)),
        }
    }

    fn handle_event(&mut self, event: winit::event::Event<()>, control_flow: &mut ControlFlow, game: &mut impl Game) -> GameResult {
        match event {
            winit::event::Event::NewEvents(start_cause) => {
                match start_cause {
                    StartCause::Init => self.timer.reset_tick(),
                    _ => (),
                }
            }
            winit::event::Event::WindowEvent { window_id, event } => {
                if window_id == self.window.window().id() {
                    match event {
                        WindowEvent::CloseRequested => {
                            if !game.event(self, Event::WindowClose)? {
                                *control_flow = ControlFlow::Exit;
                                self.quit();
                            }
                        }
                        WindowEvent::Resized(physical_size) => {
                            let scale_factor = self.window.window().scale_factor();
                            let logical_size = physical_size.to_logical(scale_factor);
                            self.graphics.resize(physical_size, scale_factor);
                            game.event(self, Event::WindowResize(LogicalSize::new(logical_size.width, logical_size.height)))?;
                        }
                        WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => {
                            let logical_size = new_inner_size.to_logical(scale_factor);
                            self.graphics.resize(*new_inner_size, scale_factor);
                            game.event(self, Event::WindowResize(LogicalSize::new(logical_size.width, logical_size.height)))?;
                        }
                        WindowEvent::Moved(physical_position) => {
                            let scale_factor = self.window.window().scale_factor();
                            let logical_position = physical_position.to_logical(scale_factor);
                            game.event(self, Event::WindowMove(LogicalPosition::new(logical_position.x, logical_position.y)))?;
                        }
                        WindowEvent::Focused(focused) => {
                            self.window.handle_focus_change_event(focused);
                            game.event(self, Event::WindowFocusChange(focused))?;
                        }
                        WindowEvent::ReceivedCharacter(char) => {
                            game.event(self, Event::ReceiveChar(char))?;
                        }
                        WindowEvent::KeyboardInput { input, .. } => {
                            let key = (input.virtual_keycode, input.scancode).into();
                            let action = input.state.into();
                            let repeated = self.keyboard.handle_input_event(key, action);
                            game.event(self, Event::KeyboardInput { key, action, repeated })?;
                        }
                        WindowEvent::ModifiersChanged(state) => {
                            let state = state.into();
                            self.keyboard.handle_modifiers_state_change(state);
                            game.event(self, Event::ModifiersChange(state))?;
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            let scale_factor = self.window.window().scale_factor();
                            let logical_position = position.to_logical(scale_factor);
                            let position = LogicalPosition::new(logical_position.x, logical_position.y);
                            self.mouse.handle_move_event(position);
                            game.event(self, Event::MouseMove(position))?;
                        }
                        WindowEvent::CursorEntered { .. } => {
                            self.mouse.handle_enter_window_event();
                            game.event(self, Event::MouseEnterWindow)?;
                        }
                        WindowEvent::CursorLeft { .. } => {
                            self.mouse.handle_leave_window_event();
                            game.event(self, Event::MouseLeaveWindow)?;
                        }
                        WindowEvent::MouseWheel { delta, phase, .. } => {
                            match delta {
                                MouseScrollDelta::LineDelta(delta_x, delta_y) => {
                                    let delta = Vector::new(delta_x, delta_y);
                                    self.mouse.handle_wheel_scroll_event(delta);
                                    game.event(self, Event::MouseWheelScroll(delta))?;
                                }
                                MouseScrollDelta::PixelDelta(logical_position) => {
                                    let delta = Vector::new(logical_position.x as f32, logical_position.y as f32);
                                    self.touchpad.handle_scroll_event(delta);
                                    game.event(self, Event::TouchpadScroll { delta, phase: phase.into() })?;
                                }
                            }
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            let button = button.into();
                            let action = state.into();
                            self.mouse.handle_input_event(button, action);
                            game.event(self, Event::MouseInput { button, action })?;
                        }
                        WindowEvent::Touch(touch) => {
                            let id = touch.id;
                            let phase = touch.phase.into();
                            let position = {
                                let scale_factor = self.window.window().scale_factor();
                                let logical_position = touch.location.to_logical(scale_factor);
                                LogicalPosition::new(logical_position.x, logical_position.y)
                            };
                            self.touch.handle_event(id, phase, position);
                            game.event(self, Event::Touch { id, phase, position })?;
                        }
                        WindowEvent::TouchpadPressure { pressure, stage, .. } => {
                            self.touchpad.handle_press_event(pressure, stage);
                            game.event(self, Event::TouchpadPress { pressure, click_stage: stage })?;
                        }
                        WindowEvent::Destroyed => self.quit(),
                        _ => (),
                    }
                }
            }
            winit::event::Event::Suspended => {
                game.event(self, Event::AppSuspend)?;
                self.audio.suspend();
            }
            winit::event::Event::Resumed => {
                self.audio.resume();
                game.event(self, Event::AppResume)?;
            }
            winit::event::Event::MainEventsCleared => {
                let events = self.gamepad.pump_events();
                for event in events {
                    let id = event.id;
                    let event = event.event;
                    match event {
                        gilrs::EventType::Connected => {
                            self.gamepad.handle_connect_event(id);
                            game.event(self, Event::GamepadConnect(id))?;
                        }
                        gilrs::EventType::Disconnected => {
                            self.gamepad.handle_disconnect_event(id);
                            game.event(self, Event::GamepadDisconnect(id))?;
                        }
                        gilrs::EventType::ButtonPressed(button, _) => {
                            let button = button.into();
                            let action = KeyAction::Down;
                            self.gamepad.handle_button_input_event(id, button, action);
                            game.event(self, Event::GamepadButtonInput { id, button, action })?;
                        }
                        gilrs::EventType::ButtonReleased(button, _) => {
                            let button = button.into();
                            let action = KeyAction::Up;
                            self.gamepad.handle_button_input_event(id, button, action);
                            game.event(self, Event::GamepadButtonInput { id, button, action })?;
                        }
                        gilrs::EventType::ButtonChanged(button, value, _) => {
                            let button = button.into();
                            self.gamepad.handle_button_change_event(id, button, value);
                            game.event(self, Event::GamepadButtonChange { id, button, value })?;
                        }
                        gilrs::EventType::AxisChanged(axis, value, _) => {
                            let axis = axis.into();
                            self.gamepad.handle_axis_change_event(id, axis, value);
                            game.event(self, Event::GamepadAxisChange { id, axis, value })?;
                        }
                        _ => (),
                    }
                }
                self.window.window().request_redraw();
            }
            winit::event::Event::RedrawRequested(window_id) => {
                if window_id == self.window.window().id() {
                    if self.timer.tick_and_check() {
                        game.update(self)?;
                        game.render(self)?;
                        self.graphics.present()?;
                        self.keyboard.clear_states();
                        self.mouse.clear_states();
                        self.touch.clear_states();
                        self.touchpad.clear_states();
                        self.gamepad.clear_states();
                    }
                }
            }
            winit::event::Event::LoopDestroyed => {
                self.quit();
                self.graphics.clean();
            }
            _ => (),
        }
        Ok(())
    }

    pub fn run(&mut self, game: &mut impl Game) -> GameResult {
        match &self.state {
            State::Ready => self.state = State::Running,
            _ => return Err(GameError::StateError(format!("engine can not be run on state `{:?}`", self.state).into())),
        }

        let mut event_loop = self.event_loop.take()
            .ok_or_else(|| GameError::RuntimeError("no event_loop instance".into()))?;
        event_loop.run_return(|event, _, control_flow| {
            match &self.state {
                State::Finished | State::Broken(_) => *control_flow = ControlFlow::Exit,
                State::Running => {
                    if let Err(error) = self.handle_event(event, control_flow, game) {
                        self.exit(error);
                    }
                }
                _ => self.exit(GameError::StateError(format!("engine state `{:?}` incorrect on handle event", self.state).into())),
            }
        });
        self.event_loop = Some(event_loop);

        match &mut self.state {
            State::Finished => Ok(()),
            State::Broken(error) => {
                let error = error.take()
                    .unwrap_or_else(|| GameError::RuntimeError("no engine broken error instance".into()));
                Err(error)
            }
            _ => Err(GameError::StateError(format!("engine state `{:?}` incorrect on event loop returned", self.state).into())),
        }
    }

    pub fn run_with<G, F>(&mut self, init: F) -> GameResult
        where
            G: Game,
            F: FnOnce(&mut Self) -> GameResult<G>,
    {
        let mut game = init(self)?;
        self.run(&mut game)
    }
}

#[derive(Debug, Clone)]
pub struct EngineBuilder {
    filesystem_config: Option<FilesystemConfig>,
    window_config: Option<WindowConfig>,
    graphics_config: Option<GraphicsConfig>,
    timer_config: Option<TimerConfig>,
    keyboard_config: Option<KeyboardConfig>,
    mouse_config: Option<MouseConfig>,
    touch_config: Option<TouchConfig>,
    touchpad_config: Option<TouchpadConfig>,
    gamepad_config: Option<GamepadConfig>,
    audio_config: Option<AudioConfig>,
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self {
            filesystem_config: None,
            window_config: None,
            graphics_config: None,
            timer_config: None,
            keyboard_config: None,
            mouse_config: None,
            touch_config: None,
            touchpad_config: None,
            gamepad_config: None,
            audio_config: None,
        }
    }

    pub fn filesystem_config(mut self, filesystem_config: FilesystemConfig) -> Self {
        self.filesystem_config = Some(filesystem_config);
        self
    }

    pub fn window_config(mut self, window_config: WindowConfig) -> Self {
        self.window_config = Some(window_config);
        self
    }

    pub fn graphics_config(mut self, graphics_config: GraphicsConfig) -> Self {
        self.graphics_config = Some(graphics_config);
        self
    }

    pub fn timer_config(mut self, timer_config: TimerConfig) -> Self {
        self.timer_config = Some(timer_config);
        self
    }

    pub fn keyboard_config(mut self, keyboard_config: KeyboardConfig) -> Self {
        self.keyboard_config = Some(keyboard_config);
        self
    }

    pub fn mouse_config(mut self, mouse_config: MouseConfig) -> Self {
        self.mouse_config = Some(mouse_config);
        self
    }

    pub fn touch_config(mut self, touch_config: TouchConfig) -> Self {
        self.touch_config = Some(touch_config);
        self
    }

    pub fn touchpad_config(mut self, touchpad_config: TouchpadConfig) -> Self {
        self.touchpad_config = Some(touchpad_config);
        self
    }

    pub fn gamepad_config(mut self, gamepad_config: GamepadConfig) -> Self {
        self.gamepad_config = Some(gamepad_config);
        self
    }

    pub fn audio_config(mut self, audio_config: AudioConfig) -> Self {
        self.audio_config = Some(audio_config);
        self
    }

    pub fn build(self) -> GameResult<Engine> {
        let filesystem_config = self.filesystem_config.unwrap_or_else(|| FilesystemConfig::new());
        let window_config = self.window_config.unwrap_or_else(|| WindowConfig::new());
        let graphics_config = self.graphics_config.unwrap_or_else(|| GraphicsConfig::new());
        let timer_config = self.timer_config.unwrap_or_else(|| TimerConfig::new());
        let keyboard_config = self.keyboard_config.unwrap_or_else(|| KeyboardConfig::new());
        let mouse_config = self.mouse_config.unwrap_or_else(|| MouseConfig::new());
        let touch_config = self.touch_config.unwrap_or_else(|| TouchConfig::new());
        let touchpad_config = self.touchpad_config.unwrap_or_else(|| TouchpadConfig::new());
        let gamepad_config = self.gamepad_config.unwrap_or_else(|| GamepadConfig::new());
        let audio_config = self.audio_config.unwrap_or_else(|| AudioConfig::new());

        let event_loop = EventLoop::new();

        let filesystem = Filesystem::new(filesystem_config)?;
        let window = Window::new(window_config, &event_loop, &filesystem)?;
        let graphics = Graphics::new(graphics_config, window.context_wrapper(), window.gl())?;
        let timer = Timer::new(timer_config)?;
        let keyboard = Keyboard::new(keyboard_config)?;
        let mouse = Mouse::new(mouse_config, window.context_wrapper())?;
        let touch = Touch::new(touch_config)?;
        let touchpad = Touchpad::new(touchpad_config)?;
        let gamepad = Gamepad::new(gamepad_config)?;
        let audio = Audio::new(audio_config)?;

        Ok(Engine {
            event_loop: Some(event_loop),
            filesystem,
            window,
            graphics,
            timer,
            keyboard,
            mouse,
            touch,
            touchpad,
            gamepad,
            audio,
            state: State::Ready,
        })
    }
}
