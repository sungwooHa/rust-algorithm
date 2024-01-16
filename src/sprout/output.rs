
pub trait Problem {
    fn solve();
}

pub mod problem2557 {
    pub fn problem() {
        println!("Hello World!");
    }

}

pub mod problem10699
{
    use chrono::prelude::*;
    pub fn problem(){
        let local_time = Local::now();
        print!("{}", local_time.format("%Y-%m-%d").to_string())
    }
}