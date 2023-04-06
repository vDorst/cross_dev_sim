use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::{Baseline, Text},
};

const ITEMS: [&str; 4] = ["Items 1", "Comm >", "printer", "bla"];

#[derive(Default)]
pub struct Menu {
    pos: u8,
    redraw: bool,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            pos: 0,
            redraw: false,
        }
    }

    pub fn up(&mut self) {
        let p = self.pos + 1;
        self.pos = if usize::from(p) == ITEMS.len() { 0 } else { p };
        // defmt::println!("p = {}", p);
        println!("p = {}", p);
        self.redraw = true;
    }

    pub fn down(&mut self) {
        self.pos = if self.pos == 0 {
            u8::try_from(ITEMS.len() - 1).unwrap()
        } else {
            self.pos - 1
        };
        // defmt::println!("p = {}", self.pos);
        println!("p = {}", self.pos);
        self.redraw = true;
    }

    pub fn need_redraw(&self) -> bool {
        self.redraw
    }

    pub fn draw<D>(&mut self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        self.redraw = false;

        let font = ascii::FONT_9X15;
        let text_style_normal = MonoTextStyleBuilder::new()
            .font(&font)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();
        let text_style_selected = MonoTextStyleBuilder::new()
            .font(&font)
            .text_color(BinaryColor::Off)
            .background_color(BinaryColor::On)
            .build();
        let style = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::Off)
            .build();

        let rows = 64 / font.character_size.height;

        Rectangle::new(
            Point::zero(),
            Size::new(64, rows * font.character_size.height),
        )
        .into_styled(style)
        .draw(display)?;

        for p in 0..rows {
            let start = p * font.character_size.height;
            let start_point = Point::new(0, i32::try_from(start).unwrap());

            let str_p = p as usize + usize::from(self.pos);

            if str_p == usize::from(self.pos) {
                Text::with_baseline(
                    ITEMS[str_p],
                    start_point,
                    text_style_selected,
                    Baseline::Top,
                )
                .draw(display)?;
            } else if str_p < ITEMS.len() {
                Text::with_baseline(ITEMS[str_p], start_point, text_style_normal, Baseline::Top)
                    .draw(display)?;
            }
        }

        Ok(())
    }
}
