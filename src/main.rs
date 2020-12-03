use rand::prelude::IteratorRandom;
use std::env;

const HIRA: &str = "あいうえおかきくけこがぎぐげごさしすせそざじずぜぞたちつてとだぢづでどなにぬねのはひふへほばびぶべぼぱぴぷぺぽまみむめもやゆよらりるれろわをん";
const HIRA_GOJUU: &str = "あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわをん";
const KATA: &str = "アイウエオカキクケコガギグゲゴサシスセソザジズゼゾタチツテトダヂヅデドナニヌネノハヒフヘホバビブベボパピプペポマミムメモヤユヨラリルレロワヲン";
const KATA_GOJUU: &str = "アイウエオカキクケコサシスセソタチツテトナニヌネノハヒフヘホマミムメモヤユヨラリルレロワヲン";

fn main() {
	let mut rng = rand::thread_rng();

    let config = Config::new(env::args_os());

	let sel: Vec<char> = config.alpha.chars().choose_multiple(&mut rng, config.amount.into());

    for (i, char) in sel.iter().enumerate() {
        if i != 0 && i % 3 == 0 {
            print!("\n");
        }
        print!("{}　", char)
    }
}

struct Config {
    alpha: String,
    amount: u8,
}

impl Config {
    pub fn new(mut args: env::ArgsOs) -> Config {
        args.next();    // drop first value (path)

        let alpha = match args.next() {
            Some(arg) => list_selection(arg),
            None => String::from(HIRA),
        };

        let amount = match args.next() {
            Some(arg) => arg.into_string().unwrap_or_default().as_str().parse::<u8>().unwrap_or_default(),
            None => 12,
        };

        Config {alpha, amount}
    }
}

fn list_selection(arg: std::ffi::OsString) -> String {
    if arg == "hira50" {
        String::from(HIRA_GOJUU)
    } else if arg == "kata50" {
        String::from(KATA_GOJUU)
    } else if arg == "kata" {
        String::from(KATA)
    } else {
        String::from(HIRA)
    }
}