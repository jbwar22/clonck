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
        println!("{} ({})", now.format("%H:%M:%S %Y年%m月%d日"), day);

        let st = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                     .expect("System time is before the epoch");
        let waitns = 1000000000 - (st.as_nanos() - 
                                      (u128::from(st.as_secs()) * 1000000000)
                                  );
        thread::sleep(Duration::from_nanos(waitns.try_into().unwrap()));
    }
}
