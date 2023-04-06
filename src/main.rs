#![deny(clippy::pedantic)]
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, Rectangle, PrimitiveStyle},
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    text::Text,
};

mod button;
mod menu;
use button::Button;

use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder, SimulatorEvent, sdl2::{MouseButton, Mod, Keycode}};

use crate::button::BtnState;
fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));

    let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    Circle::new(Point::new(72, 8), 48)
        .into_styled(line_style)
        .draw(&mut display)?;

    Line::new(Point::new(48, 16), Point::new(8, 16))
        .into_styled(line_style)
        .draw(&mut display)?;

    Line::new(Point::new(48, 16), Point::new(64, 32))
        .into_styled(line_style)
        .draw(&mut display)?;

    Rectangle::new(Point::new(79, 15), Size::new(34, 34))
        .into_styled(line_style)
        .draw(&mut display)?;

    Text::new("Simulator Running!", Point::new(5, 5), text_style).draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut win = Window::new("Hello World", &output_settings);
    

    let gpio_key_left = button::FakeInput::new();
    let gpio_key_right = button::FakeInput::new();

    let mut key_left = Button::new(&gpio_key_left);
    let mut key_right = Button::new(&gpio_key_right);

    let mut tick = 0;

    let mut menu = menu::Menu::new();

    menu.draw(&mut display).unwrap();

    'mainloop: loop {
        win.update(&display);
        let events = win.events();
        for event in events {
            // println!("{event:?}");
            match event {
                SimulatorEvent::Quit => {
                    println!("Window closed, exit simulator.");
                    break 'mainloop;
                }
                SimulatorEvent::MouseButtonDown { mouse_btn, point }  => {
                    if mouse_btn == MouseButton::Left {
                        Pixel(point, BinaryColor::On).draw(&mut display)?;
                    }
                    if mouse_btn == MouseButton::Right {
                        Pixel(point, BinaryColor::Off).draw(&mut display)?;
                    }
                }
                SimulatorEvent::KeyUp { keycode, keymod, repeat: _ } => {
                    if keymod == Mod::NOMOD && keycode == Keycode::Left {
                        gpio_key_left.set_state(true);
                    }
                    if keymod == Mod::NOMOD && keycode == Keycode::Right {
                        gpio_key_right.set_state(true);
                    }
                }
                SimulatorEvent::KeyDown { keycode, keymod, repeat: _ } => {
                    if keymod == Mod::NOMOD && keycode == Keycode::Left {
                        gpio_key_left.set_state(false);
                    }
                    if keymod == Mod::NOMOD && keycode == Keycode::Right {
                        gpio_key_right.set_state(false);
                    }
                }
                _ => (),
            }

        }

        match (key_left.sample(tick), key_right.sample(tick)) {
            (BtnState::SHORT, BtnState::NONE) => menu.down(),
            (BtnState::NONE, BtnState::SHORT) => menu.up(),
            _ => (),
        }

        if menu.need_redraw() {
            menu.draw(&mut display).unwrap();
        }

        tick += 1;
        std::thread::sleep(std::time::Duration::from_millis(25));
    }

    Ok(())
}