use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};
use std::str::from_utf8;
use std::time::Instant;

struct State {
    min: f64,
    max: f64,
    sum: f64,
    count: u64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            min: f64::MAX,
            max: f64::MIN,
            sum: 0.0,
            count: 0,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.1}/{:.1}/{:.1}",
            self.min,
            self.sum / (self.count as f64),
            self.max
        )
    }
}

impl State {
    fn update(&mut self, temp: f64) {
        self.min = self.min.min(temp);
        self.max = self.max.max(temp);
        self.sum += temp;
        self.count += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("./measurements.txt")?;

    let mut reader = BufReader::new(file);

    let mut map: BTreeMap<String, State> = BTreeMap::new();
    let mut count: u64 = 0;

    let now = Instant::now();

    loop {
        let mut buffer = String::with_capacity(32);
        let n = reader.read_line(&mut buffer)?;
        if n == 0 {
            break;
        }

        buffer.pop();
        let delim = buffer.rfind(';').unwrap();
        let temp = unsafe { buffer.get_unchecked(delim + 1..).parse::<f64>()? };

        buffer.truncate(delim);
        map.entry(buffer).or_default().update(temp);
        count += 1;
    }

    // map.iter().for_each(|(k, v)| println!("{k}={v}"));
    //
    // let elapsed = now.elapsed().as_secs_f64();
    // println!(
    //     "Processed {:e} entries in {:.3e} seconds ({:.3e} entry/s)",
    //     count,
    //     elapsed,
    //     elapsed / (count as f64)
    // );

    Ok(())
}
