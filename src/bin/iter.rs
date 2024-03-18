use std::collections::HashMap;
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
            Some(buffer)
        }
    }
}

fn main() {
    let file = fs::File::open("./measurements.txt").unwrap();

    let reader = Reader {
        buf: BufReader::new(file),
    };

    let mut map: HashMap<String, State> = HashMap::new();
    // let mut count: u64 = 0;

    // let now = Instant::now();

    reader.for_each(|mut line| {
        let delim = line.rfind(';').unwrap();
        let temp = unsafe {
            line.get_unchecked(delim + 1..line.len() - 1)
                .parse::<f64>()
                .unwrap()
        };
        line.truncate(delim);
        map.entry(line).or_default().update(temp);
        // count += 1;
    });

    let mut result: Vec<_> = map.into_iter().collect();
    result.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    // result.into_iter().for_each(|(k, v)| println!("{k}={v}"));
    //
    // let elapsed = now.elapsed().as_secs_f64();
    // println!(
    //     "Processed {:e} entries in {:.3e} seconds ({:.3e} entry/s)",
    //     count,
    //     elapsed,
    //     elapsed / (count as f64)
    // );
}
