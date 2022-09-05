// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

macro_rules! planet {
    ($planet:ident, $age: expr) => {
        pub struct $planet;
        impl Planet for $planet {
            fn period() -> f64 {
                $age
            }
        }
    };
}

pub const EARTH_YEAR_SECONDS: f64 = 31557600.0;
// 一个星球的年的时间 = seconds / (years_during * earth_year_seconds)
pub trait Planet {
    fn period() -> f64;
    fn years_during(d: &Duration) -> f64 {
        // unimplemented!(
        //     "convert a duration ({:?}) to the number of years on this planet for that duration",
        //     d,
        // );
        d.0 as f64 / EARTH_YEAR_SECONDS / Self::period()
    }
}


planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
