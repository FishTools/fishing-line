use chrono::{Local, TimeZone};
use fishing_rod::prelude::*;

fn main() {
    let terminal_path = std::env::var("TERMINAL_PATH").expect("TERMINAL_PATH must be set");

    let runtime = PythonRuntime::new().initialize(&terminal_path);

    if runtime.is_err() {
        panic!("Failed to initialize and connect to MT5 terminal");
    }

    let runtime = runtime.unwrap();

    let copy_rates_from = runtime
        .copy_rates_from("EURUSD", Timeframe::D1, Local::now(), 10)
        .expect("Unable to get symbol information");

    println!("Copy rates from: ");
    for rates in copy_rates_from.iter() {
        println!(
            "time: {}  open: {}\t  high: {}\t  low: {}\t  close:{}\t",
            rates.time, rates.open, rates.high, rates.low, rates.close
        );
    }

    let copy_rates_from_pos = runtime
        .copy_rates_from_pos("EURUSD", Timeframe::D1, 0, 10)
        .expect("Unable to get symbol information");

    println!("Copy rates from pos: ");
    for rates in copy_rates_from_pos.iter() {
        println!(
            "time: {}  open: {}\t  high: {}\t  low: {}\t  close:{}\t",
            rates.time, rates.open, rates.high, rates.low, rates.close
        );
    }

    let copy_rates_range = runtime
        .copy_rates_range(
            "EURUSD",
            Timeframe::D1,
            Local.with_ymd_and_hms(2024, 7, 8, 0, 0, 0).unwrap(),
            Local::now(),
        )
        .expect("Unable to get symbol information");

    println!("Copy rates range: ");
    for rates in copy_rates_range.iter() {
        println!(
            "time: {}  open: {}\t  high: {}\t  low: {}\t  close:{}\t",
            rates.time, rates.open, rates.high, rates.low, rates.close
        );
    }
}
