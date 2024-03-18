use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::time::Instant;

use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Debug)]
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

    let mut reader = Reader {
        buf: BufReader::new(file),
    };

    let now = Instant::now();

    let mut buf = String::new();
    reader.buf.read_to_string(&mut buf).unwrap();

    let result = buf
        .lines()
        .par_bridge()
        .map(|line| {
            let (city, temp) = line.split_once(';').unwrap();
            let temp = temp.parse::<f64>().unwrap();
            (city, temp)
        })
        .fold(
            HashMap::new,
            |mut map: HashMap<&str, State>, (city, temp)| {
                map.entry(city).or_default().update(temp);
                map
            },
        )
        .reduce(HashMap::new, |mut acc: HashMap<&str, State>, map| {
            acc.extend(map);
            acc
        });

    let mut result: Vec<_> = result.into_iter().collect();
    result.sort_unstable_by(|a, b| a.0.cmp(b.0));
    result.into_iter().for_each(|(k, v)| println!("{k}={v}"));

    let elapsed = now.elapsed().as_secs_f64();
    println!("Processed entries in {:.3e} seconds", elapsed,);

    Ok(())
}
