use std::thread;
use std::time::{SystemTime, Duration};
use chrono::{Local, Datelike, Weekday};

fn main() {
    loop {
        let now = Local::now();
        let day = match now.weekday() {
            Weekday::Sun => "日",
            Weekday::Mon => "月",
            Weekday::Tue => "火",
            Weekday::Wed => "水",
            Weekday::Thu => "木",
            Weekday::Fri => "金",
            Weekday::Sat => "土",
        };
        println!("{} ({})", now.format("%H:%M:%S %Y年%m月%d日 %f"), day);
        let ns = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n)  => n.as_nanos(),
            Err(_) => panic!("something went wrong!")
        };
        let sec = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n)  => n.as_secs(),
            Err(_) => panic!("something went wrong!")
        };
        let waitns = 1000000000 - (ns - (u128::from(sec) * 1000000000));
        thread::sleep(Duration::from_nanos(waitns.try_into().unwrap()));
    }
}
