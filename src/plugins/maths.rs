use std::time::Duration;

pub struct Interpolation {
    started: Duration,
    duration: Duration,
    endpoints: (f32, f32),
}

impl Interpolation {
    pub fn interp(&self, elapsed: Duration) -> f32 {
        if elapsed < self.started {
            return self.endpoints.0;
        }
        if elapsed > self.started + self.duration {
            return self.endpoints.1;
        }
        let f = (elapsed - self.started).as_secs_f32() / self.duration.as_secs_f32();
        let f = f.powf(2.0);
        debug_assert!(f >= 0.0 && f <= 1.0);
        self.endpoints.1 * f + self.endpoints.0 * (1.0 - f)
    }
}
pub struct TriangleInterpolation {
    fadein: Interpolation,
    fadeout: Interpolation,
}
impl TriangleInterpolation {
    pub fn new(
        started: Duration,
        fadeindur: Duration,
        fadeoutdur: Duration,
        endpoints: (f32, f32),
    ) -> Self {
        let fadein = Interpolation {
            started,
            duration: fadeindur,
            endpoints,
        };
        let fadeout = Interpolation {
            started: started + fadeindur,
            duration: fadeoutdur,
            endpoints: (endpoints.1, endpoints.0),
        };
        Self { fadein, fadeout }
    }
    pub fn interp(&self, elapsed: Duration) -> f32 {
        if elapsed < self.fadein.started && elapsed > self.fadeout.started + self.fadeout.duration {
            return self.fadein.endpoints.0;
        }
        if elapsed > self.fadein.started + self.fadein.duration && elapsed < self.fadeout.started {
            return self.fadein.endpoints.1;
        }
        if elapsed < self.fadein.started + self.fadein.duration {
            return self.fadein.interp(elapsed);
        }
        if elapsed < self.fadeout.started + self.fadeout.duration {
            return self.fadeout.interp(elapsed);
        }
        return self.fadein.endpoints.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_linearinterpolation() {
        for i in 0..5 {
            let intp = Interpolation {
                started: Duration::from_millis(100),
                duration: Duration::from_millis(500),
                endpoints: (0.0, (i as f32) * 1.0),
            };
            let i2 = (i + 4) as f32;
            assert_eq!(intp.interp(Duration::from_millis(150)), (i as f32) * 0.1);
            let intp = Interpolation {
                started: Duration::from_millis(100),
                duration: Duration::from_millis(500),
                endpoints: (i2, i2 + 1.0),
            };
            assert_eq!(
                (intp.interp(Duration::from_millis(150)) - (i2 + 0.1)).abs() <= 0.01,
                true
            );
        }
    }
}
