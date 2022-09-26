use clap::{App, Arg};
use std::error::Error;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
    today: NaiveDate,
}

type MyResult<T> = Result<T, Box<dyn Error>>;
