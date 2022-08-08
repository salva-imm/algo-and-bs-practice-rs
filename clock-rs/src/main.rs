#[derive(Debug, PartialEq)]
struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    fn new(hours: i32, minutes: i32) -> Self {
        calc_clock(hours, minutes)
    }

    fn add_minutes(&self, minutes: i32) -> Self {
        calc_clock(self.hours, self.minutes + minutes)
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}

fn calc_clock(hours: i32, minutes: i32) -> Clock {
    let mut extra = minutes / 60;
    if minutes < 0 && minutes % 60 != 0 {
        extra += -1;
    }
    Clock {
        hours: ((hours.rem_euclid(24)) + extra).rem_euclid(24),
        minutes: minutes.rem_euclid(60),
    }
}

fn main() {
    println!(
        "Hello, world!, {}",
        Clock::new(5, 32).add_minutes(-1500).to_string()
    );
}
