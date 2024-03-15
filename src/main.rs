use std::collections::HashMap;
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

    fn merge(&mut self, other: Self) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.sum += other.sum;
        self.count += other.count;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("./measurements.txt")?;

    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    let mut map: HashMap<String, State> = HashMap::new();
    let mut count: u64 = 0;

    let now = Instant::now();

    loop {
        buffer.clear();
        let n = reader.read_until(b'\n', &mut buffer)?;
        if n == 0 {
            break;
        }

        buffer.pop();
        let string = from_utf8(&buffer)?;
        let (city, temp) = string.split_once(';').unwrap();
        let temp = temp.parse::<f64>()?;

        map.entry(city.to_string()).or_default().update(temp);
        count += 1;
    }

    let mut result: Vec<_> = map.into_iter().collect();
    result.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    result.into_iter().for_each(|(k, v)| println!("{k}={v}"));

    let elapsed = now.elapsed().as_secs_f64();
    println!(
        "Processed {:e} entries in {:.3e} seconds ({:.3e} entry/s)",
        count,
        elapsed,
        elapsed / (count as f64)
    );

    Ok(())
}
