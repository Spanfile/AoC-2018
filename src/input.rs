use reqwest::header;
use std::fmt::Debug;
use std::fs;
use std::marker::PhantomData;
use std::path::Path;
use std::str::{FromStr, Lines, SplitWhitespace};

#[derive(Clone)]
pub struct Input {
    input: String,
}

pub struct ParsedLines<'a, T: FromStr>
where
    T::Err: Debug,
{
    lines_iter: Lines<'a>,
    item: PhantomData<T>,
}

impl<'a, T: FromStr> Iterator for ParsedLines<'a, T>
where
    T::Err: Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines_iter.next().map(|s| s.parse::<T>().unwrap())
    }
}

pub struct ParsedSplits<'a, T: FromStr>
where
    T::Err: Debug,
{
    splits_iter: SplitWhitespace<'a>,
    item: PhantomData<T>,
}

impl<'a, T: FromStr> Iterator for ParsedSplits<'a, T>
where
    T::Err: Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.splits_iter.next().map(|s| s.parse::<T>().unwrap())
    }
}

impl Input {
    pub fn lines(&self) -> Lines {
        self.input.lines()
    }

    pub fn get(self) -> String {
        self.input
    }

    // pub fn parse<T: FromStr>(&self) -> T
    // where
    //     T::Err: Debug,
    // {
    //     self.input.parse().unwrap()
    // }

    pub fn parse_lines<T: FromStr>(&self) -> ParsedLines<T>
    where
        T::Err: Debug,
    {
        ParsedLines {
            lines_iter: self.lines(),
            item: PhantomData,
        }
    }

    pub fn parse_split<T: FromStr>(&self) -> ParsedSplits<T>
    where
        T::Err: Debug,
    {
        ParsedSplits {
            splits_iter: self.input.split_whitespace(),
            item: PhantomData,
        }
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
