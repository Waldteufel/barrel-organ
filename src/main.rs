extern crate clap;
extern crate rand;
use std::io::{self, BufRead, Write};
use std::{f64, slice};
use std::f64::consts::{PI, FRAC_PI_2};
use clap::{Arg, App};

fn gen_sample(t: f64, j: usize, c: char) -> f64 {
    let freq = 110.0 * f64::powf(2.0, (j as f64) / 12.0);

    match c {
        '$' => 2.0 * f64::sin(freq * t),
        '*' => f64::abs(2.0 - (t * freq/FRAC_PI_2) % 4.0) - 1.0,
        '+' => ((t * freq/PI) % 2.0 - 1.0) / 4.0,
        '%' => f64::signum(f64::sin(freq * t)) / 4.0,
        '/' => (50.0 / freq) * ((rand::random::<u32>() % 2) as f64),
        _ => 0.0
    }
}

trait WriteFloats {
    fn write_f32_vec(&mut self, vec: &Vec<f32>) -> std::io::Result<()>;
}

impl<T> WriteFloats for T where T: Write {
    fn write_f32_vec(&mut self, vec: &Vec<f32>) -> std::io::Result<()> {
        self.write_all(unsafe{ slice::from_raw_parts(vec.as_ptr() as *const u8, 4 * vec.len()) })
    }
}

fn main() {
    let rate = 11025.0;
    let default_bpm = 150.0;
    let beat: f64 = 1.0/4.0;
    let quantization: f64 = 1.0/8.0;
    let quarter_subdivisions = (quantization.recip().log2() - beat.recip().log2()) as i32;

    let matches = App::new("barrel-organ")
        .arg(Arg::with_name("bpm")
            .short("b")
            .long("bpm")
            .value_name("tempo")
            .help("Specifies the tempo of the song, in beats per minute (bpm)")
            .takes_value(true))
        .get_matches();

    let mut bpm = matches
        .value_of("bpm")
        .unwrap_or(&default_bpm.to_string())
        .parse::<f64>()
        .unwrap();

    let stdin = io::stdin();
    let stdout = io::stdout();

    let stdin_h = stdin.lock();
    let mut stdout_h = stdout.lock();

    let bpm_override = bpm;
    let mut composer_bpm = f64::NAN;
    let mut gt = 0.0;

    for line in stdin_h.lines() {
        /* for each input line, output buf_len samples */
        let buf_len = (rate/bpm*60.0/f64::powi(2.0, quarter_subdivisions)) as usize;
        let frac = rate/(buf_len as f64);
        let mut buf = vec![0.0 as f32; buf_len];

        let line = line.unwrap();
        if line.starts_with("#") {
            continue;
        }
        else if line.starts_with("!") {
            let command = &line[1..];
            let parts = command
                .split(" ")
                .filter(|&x| !x.is_empty())
                .collect::<Vec<&str>>();
            match parts.first() {
                Some(&name) => {
                    match name {
                        "bpm" => {
                            let args = &parts[1..];
                            if args.is_empty() {
                                eprintln!("Missing tempo.");
                            }
                            else {
                                let new_bpm = args[0].parse::<f64>().unwrap();
                                if composer_bpm.is_nan() {
                                    composer_bpm = new_bpm;
                                    if bpm_override == default_bpm {
                                        bpm = new_bpm;
                                    }
                                }
                                else {
                                    let change_rate = new_bpm / composer_bpm;
                                    bpm = bpm_override * change_rate;
                                }
                            }
                        },
                        _ => eprintln!("Ignoring unknown command: \"{}\"", &name)
                    }
                },
                _ => eprintln!("Ignoring empty command")
            };
            continue;
        }
        else if composer_bpm.is_nan() {
            /* this covers cases where the initial tempo is missing, and only later changes in tempo exist */
            composer_bpm = default_bpm;
        }
        for i in 0..buf_len {
            let t = gt + (i as f64) * 2.0 * PI/rate;  // arrr!
            buf[i] = (line.chars().enumerate().map(|(j, c)| gen_sample(t, j, c)).sum::<f64>() / 8.0) as f32;
        }
        stdout_h.write_f32_vec(&buf).unwrap();
        gt = gt + 2.0*PI/frac;
    }
}
