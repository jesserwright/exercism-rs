use std::fmt;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.get_hrs_mins() == other.get_hrs_mins()
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
    pub fn get_hrs_mins(&self) -> (i32, i32) {
        let mut h = self.hours;
        let mut m = self.minutes;
        h += m / 60;
        m %= 60;

        if m < 0 {
            m = 60 - (-m % 60);
            h -= 1;
        }

        if h < 0 {
            let rem = h % 24;
            if rem != 0 {
                h = 24 + rem;
            } else {
                h = 0;
            }
        } else {
            h %= 24;
        }
        (h, m)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (h, m) = self.get_hrs_mins();
        write!(f, "{:02}:{:02}", h, m)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Clock {{ hours: {}, minutes: {} }}",
            self.hours, self.hours
        )
    }
}
