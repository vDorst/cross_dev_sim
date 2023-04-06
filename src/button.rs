use std::{convert::Infallible, fmt::Debug};
use std::cell::Cell;

//use nrf52840_hal as hal;
use embedded_hal::{
    digital::v2::InputPin,
};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum BtnState {
    NONE,
    // RELEASE,
    PRESS,
    SHORT,
    LONG,
}

pub struct Button<T> {
    btn: T,
    time: usize,
}


#[derive(Debug)]
#[non_exhaustive]
pub struct FakeInput(Cell<bool>);

impl FakeInput {
    pub fn new() -> Self {
        Self(Cell::new(false))
    }

    pub fn set_state(&self, state: bool) {
        self.0.set(state);
    }
}

impl InputPin for &FakeInput {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.0.get())
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.0.get())
    }
}


const KEY_SHORT: usize = 10;

impl<T> Button<T>
where
T: InputPin + Debug,
 {
    pub fn new(btn: T) -> Self {
        Self { btn, time: 0 }
    }

    pub fn sample(&mut self, tick: usize) -> BtnState where <T as embedded_hal::digital::v2::InputPin>::Error: Debug {
        let butten = self.btn.is_low().unwrap();

        if self.time == 0 {
            if butten {
                self.time = tick;
                BtnState::PRESS
            } else {
                BtnState::NONE
            }
        } else {
            let delta = tick - self.time;
            if butten {
                if (delta % self::KEY_SHORT) == 0 {
                    BtnState::LONG
                } else {
                    BtnState::NONE
                }
            } else {
                self.time = 0;
                if delta < self::KEY_SHORT {
                    BtnState::SHORT
                } else {
                    BtnState::NONE
                }
            }
        }
    }
}
