pub enum Commands {
    Program(Arguments),
    Help,
}

#[derive(Debug)]
pub struct Arguments {
    pub r0: f64,
    pub r5: f64,
    pub r10: f64,
    pub r15: f64,
    pub r20: f64,
    pub n: u64,
}

impl Commands {
    pub fn try_from_args() -> Result<Commands, String> {
        let mut args = std::env::args().skip(1);
        if args.any(|arg| arg == "-h" || arg == "--help") {
            return Ok(Commands::Help);
        }
        let mut args = std::env::args().skip(1);
        let radiuses = {
            let mut radiuses = vec![];
            for _ in 0..5 {
                match args.next().map(|arg| arg.parse::<f64>()) {
                    Some(Ok(n)) if n != 0.0 => radiuses.push(n),
                    Some(Ok(_)) => return Err("Radix can't be zero.".into()),
                    Some(Err(e)) => return Err(e.to_string()),
                    _ => return Err("Invalid argument count.".into()),
                }
            }
            radiuses
        };
        let n = args
            .next()
            .map(|arg| arg.parse::<u64>())
            .ok_or("Invalid argument count.".to_owned())?
            .map_err(|e| e.to_string())?;
        if n == 0 {
            return Err("N can't be zero.".into());
        }

        Ok(Commands::Program(Arguments {
            r0: radiuses[0],
            r5: radiuses[1],
            r10: radiuses[2],
            r15: radiuses[3],
            r20: radiuses[4],
            n,
        }))
    }
}
