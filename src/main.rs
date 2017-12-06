extern crate rand;
use std::io::{self, BufRead, Write};
use std::{f64, slice};
use std::f64::consts::{PI, FRAC_PI_2};
use rand::random;

fn gen_sample(t: f64, j: usize, c: char) -> f64 {
    let freq = 110.0 * f64::powf(2.0, (j as f64) / 12.0);

    match c {
        '$' => 2.0 * f64::sin(freq * t),
        '*' => f64::abs(2.0 - (t * freq/FRAC_PI_2) % 4.0) - 1.0,
        '+' => ((t * freq/PI) % 2.0 - 1.0) / 4.0,
        '%' => f64::signum(f64::sin(freq * t)) / 4.0,
        '/' => (50.0 / freq) * ((random::<u32>() % 2) as f64),
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
    let frac = 5.0;
    let buf_len = (rate/frac) as usize;

    let stdin = io::stdin();
    let stdout = io::stdout();

    let stdin_h = stdin.lock();
    let mut stdout_h = stdout.lock();

    let mut gt = 0.0;
    let mut buf = vec![0.0 as f32; buf_len];

    for line in stdin_h.lines() {
        /* for each input line, output buf_len samples */
        let line = line.unwrap();
        for i in 0..buf_len {
            let t = gt + (i as f64) * 2.0 * PI/rate;  // arrr!
            buf[i] = (line.chars().enumerate().map(|(j, c)| gen_sample(t, j, c)).sum::<f64>() / 8.0) as f32;
        }
        stdout_h.write_f32_vec(&buf).unwrap();
        gt = gt + 2.0*PI/frac;
    }
}
