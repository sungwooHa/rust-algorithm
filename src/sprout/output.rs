pub mod problem2557 {
    pub fn problem() {
        println!("Hello World!");
    }
}

pub mod problem10699 {
    use chrono::prelude::*;

    pub fn problem() {
        problemByChrono();
        get_current_date();
    }

    pub fn problemByChrono() {
        let local_time = Local::now();
        println!("{}", local_time.format("%Y-%m-%d").to_string());
    }

    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn get_current_date() {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let in_days = since_the_epoch.as_secs() / 86400;

        let (is_common_era, year) = if in_days >= 719468 {
            (true, (in_days - 719468) / 36524)
        } else {
            // Check if in_days is greater than 719468 before subtracting
            if 719468 > in_days {
                (false, (719468 - in_days) / 36524)
            } else {
                (false, 0)
            }
        };

        // let year : i64 = if is_common_era { (year + 1).try_into().unwrap() } else { (1 - year).try_into().unwrap()};
        // println!("{}-01-01", year);
    }
}

pub mod problem10171 {
    pub fn problem() {
        println!("\\    /\\");
        println!(" )  ( ')");
        println!("(  /  )");
        println!(" \\(__)|");
    }
}

pub mod problem10172 {
    pub fn problem() {
        println!("|\\_/|");
        println!("|q p|   /}}");
        println!("( 0 )\"\"\"\\");
        println!("|\"^\"`    |");
        println!("||_/=\\\\__|");
    }
}

pub mod problem25083 {
    pub fn problem() {
        println!("         ,r'\"7");
        println!("r`-_   ,'  ,/");
        println!(" \\. \". L_r'");
        println!("   `~\\/");
        println!("      |");
        println!("      |");
    }
}
