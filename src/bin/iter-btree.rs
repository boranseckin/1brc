use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};
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

struct Reader<R> {
    buf: R,
}

impl<R: BufRead> Iterator for Reader<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = String::new();
        let n = self.buf.read_line(&mut buffer).unwrap();
        if n == 0 {
            None
        } else {
            buffer.pop();
            Some(buffer)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("./measurements.txt")?;

    let reader = Reader {
        buf: BufReader::new(file),
    };

    let mut map: BTreeMap<String, State> = BTreeMap::new();
    let mut count: u64 = 0;

    let now = Instant::now();

    reader.into_iter().for_each(|mut line| {
        let delim = line.find(';').unwrap();
        let temp = line.get(delim + 1..).unwrap();
        let temp = temp.parse::<f64>().unwrap();
        line.truncate(delim);

        map.entry(line).or_default().update(temp);
        count += 1;
    });

    map.into_iter().for_each(|(k, v)| println!("{k}={v}"));

    let elapsed = now.elapsed().as_secs_f64();
    println!(
        "Processed {:e} entries in {:.3e} seconds ({:.3e} entry/s)",
        count,
        elapsed,
        elapsed / (count as f64)
    );

    Ok(())
}
