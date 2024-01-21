use rppal::spi::{Bus, Mode, SlaveSelect};

pub trait IntoEnum<Target> {
    fn cast(self) -> Target;
}

impl IntoEnum<Bus> for u64 {
    fn cast(self) -> Bus {
        match self {
            0 => Bus::Spi0,
            1 => Bus::Spi1,
            2 => Bus::Spi2,
            3 => Bus::Spi3,
            4 => Bus::Spi4,
            5 => Bus::Spi5,
            6 => Bus::Spi6,
            n => panic!("Invalid value {n}"),
        }
    }
}

impl IntoEnum<SlaveSelect> for u64 {
    fn cast(self) -> SlaveSelect {
        match self {
            0 => SlaveSelect::Ss0,
            1 => SlaveSelect::Ss1,
            2 => SlaveSelect::Ss2,
            3 => SlaveSelect::Ss3,
            4 => SlaveSelect::Ss4,
            5 => SlaveSelect::Ss5,
            6 => SlaveSelect::Ss6,
            7 => SlaveSelect::Ss7,
            8 => SlaveSelect::Ss8,
            9 => SlaveSelect::Ss9,
            10 => SlaveSelect::Ss10,
            11 => SlaveSelect::Ss11,
            12 => SlaveSelect::Ss12,
            13 => SlaveSelect::Ss13,
            14 => SlaveSelect::Ss14,
            15 => SlaveSelect::Ss15,
            n => panic!("Invalid value {n}"),
        }
    }
}

impl IntoEnum<Mode> for u64 {
    fn cast(self) -> Mode {
        match self {
            0 => Mode::Mode0,
            1 => Mode::Mode1,
            2 => Mode::Mode2,
            3 => Mode::Mode3,
            n => panic!("Invalid value {n}"),
        }
    }
}
