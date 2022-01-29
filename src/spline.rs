use std::fmt::Display;

use crate::args::Arguments;

#[derive(Debug)]
pub struct Spline {
    result: Vec<f64>,
    ordinate: Vec<f64>,
    vector: [f64; 5],
    arguments: Arguments,
}

impl Spline {
    const ABSCISSAS: [f64; 5] = [0.0, 5.0, 10.0, 15.0, 20.0];

    pub fn apply(&mut self) {
        let a = 6.0 * (self.arguments.r10 - 2.0 * self.arguments.r5 + self.arguments.r0) / 50.0;
        let b = 6.0 * (self.arguments.r15 - 2.0 * self.arguments.r10 + self.arguments.r5) / 50.0;
        let c = 6.0 * (self.arguments.r20 - 2.0 * self.arguments.r15 + self.arguments.r10) / 50.0;
        self.vector[2] = (b - (a + c) / 4.0) * 4.0 / 7.0;
        self.vector[1] = a / 2.0 - 0.25 * self.vector[2];
        self.vector[3] = c / 2.0 - 0.25 * self.vector[2];
        for i in 0..self.arguments.n {
            let x = 20.0 / (self.arguments.n - 1) as f64 * i as f64;
            let i = ((x as f64 - 0.01) / 5.0) as usize + 1;
            let x = x as f64;
            let result = -self.vector[i - 1] / 30.0 * (x - Self::ABSCISSAS[i]).powf(3.0)
                + self.vector[i] / 30.0 * (x - Self::ABSCISSAS[i - 1]).powf(3.0)
                - (self.ordinate[i - 1] / 5.0 - 5.0 / 6.0 * self.vector[i - 1])
                    * (x - Self::ABSCISSAS[i])
                + (self.ordinate[i] / 5.0 - 5.0 / 6.0 * self.vector[i])
                    * (x - Self::ABSCISSAS[i - 1]);
            self.result.push(result);
        }
    }
}

impl From<Arguments> for Spline {
    fn from(arguments: Arguments) -> Self {
        Self {
            result: vec![],
            ordinate: vec![
                arguments.r0,
                arguments.r5,
                arguments.r10,
                arguments.r15,
                arguments.r20,
            ],
            vector: [0.0; 5],
            arguments,
        }
    }
}

impl Display for Spline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "vector result: [{:.1}, {:.1}, {:.1}, {:.1}, {:.1}]",
            self.vector[0].better_round(),
            self.vector[1].better_round(),
            self.vector[2].better_round(),
            self.vector[3].better_round(),
            self.vector[4].better_round(),
        )?;
        for i in 0..self.arguments.n as usize {
            writeln!(
                f,
                "abscissa: {:.1} cm\tradius: {:.1} cm",
                20.0 / (self.arguments.n - 1) as f64 * i as f64,
                self.result[i]
            )?;
        }
        Ok(())
    }
}

trait BetterRound {
    fn better_round(self) -> Self;
}

impl BetterRound for f64 {
    fn better_round(self) -> Self {
        match (self * 10.0).round() / 10.0 {
            res if res == 0.0 => 0.0,
            res => res,
        }
    }
}
