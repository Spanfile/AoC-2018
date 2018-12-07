use reqwest::header;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::str::Lines;

#[derive(Clone)]
pub struct Input {
    input: String,
}

impl Input {
    pub fn lines(&self) -> Lines {
        self.input.lines()
    }

    pub fn get(self) -> String {
        self.input
    }
}

impl FromStr for Input {
    type Err = ();
    fn from_str(s: &str) -> Result<Input, ()> {
        Ok(Input {
            input: String::from(s),
        })
    }
}

pub fn get(day: i32) -> Input {
    let cached_path = String::from("cache/day") + &day.to_string();
    if Path::new(&cached_path).exists() {
        fs::read_to_string(cached_path)
            .expect("error reading cache file")
            .parse()
            .unwrap()
    } else {
        get_live(day).parse().unwrap()
    }
}

fn get_live(day: i32) -> String {
    let url = format!("https://adventofcode.com/2018/day/{}/input", day);
    let client = match reqwest::Client::builder().build() {
        Ok(client) => client,
        Err(err) => panic!("error creating client: {}", err),
    };

    let input = match client
        .get(url.as_str())
        .header(
            header::COOKIE,
            fs::read_to_string("session")
                .expect("error reading session file")
                .trim(),
        )
        .send()
    {
        Ok(mut resp) => match resp.text() {
            Ok(text) => String::from(text.trim()),
            Err(err) => panic!("error reading body: {}", err),
        },
        Err(err) => panic!("error requesting input: {}", err),
    };

    save_to_cache(day, &input);

    input
}

fn save_to_cache(day: i32, input: &str) {
    let cache_path = String::from("cache/day") + &day.to_string();
    fs::write(cache_path, input).expect("error writing to cache");
}
