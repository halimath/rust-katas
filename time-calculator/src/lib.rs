extern crate regex;

pub mod time {
    pub enum TimeUnit {
        Seconds(i32),
        Minutes(i32),
        Hours(i32),
    }

    impl TimeUnit {
        pub fn parse (input: &str) -> Result<TimeUnit, String> {
            use regex::Regex;

            let regex = Regex::new(r"^([0-9]+)([smh])$").unwrap();
            for group in regex.captures_iter(input) {
                return match &group[1].parse::<i32>() {
                    &Result::Err(_) => Result::Err(format!("Invalid number in time component: {}", input)),
                    &Result::Ok(num) => {
                        match &group[2] {
                            "s" => Result::Ok(TimeUnit::Seconds(num)),
                            "m" => Result::Ok(TimeUnit::Minutes(num)),
                            "h" => Result::Ok(TimeUnit::Hours(num)),
                            &_ => return Result::Err(format!("Invalid time unit: {}", &group[2]))
                        }
                    },
                };
            }

            Result::Err(format!("Invalid time component: {}", input))
        }

        pub fn convert_to_seconds(&self) -> i32 {
            match self {
                &TimeUnit::Seconds(val) => val,
                &TimeUnit::Minutes(val) => val * 60,
                &TimeUnit::Hours(val) => val * 60 * 60,
            }
        }

        pub fn format(&self) -> String {
            match self {
                &TimeUnit::Seconds(val) => format!("{}{}", val, "s"),
                &TimeUnit::Minutes(val) => format!("{}{}", val, "m"),
                &TimeUnit::Hours(val) => format!("{}{}", val, "h"),
            }
        }

        pub fn is_not_empty(&self) -> bool {
            match self {
                &TimeUnit::Seconds(v) => v > 0,
                &TimeUnit::Minutes(v) => v > 0,
                &TimeUnit::Hours(v) => v > 0,
            }
        }
    }

    pub struct TimeInterval {
        seconds: i32,
    }

    impl TimeInterval {
        pub fn new(seconds: i32) -> TimeInterval {
            TimeInterval { seconds: seconds }
        }

        pub fn parse(time_string: &str) -> Result<TimeInterval, String> {
            let mut seconds = 0;

            for component_string in time_string.split(' ') {
                match TimeUnit::parse(component_string) {
                    Result::Err(e) => return Result::Err(format!("Invalid time string: {}: {}", time_string, e)),
                    Result::Ok(component) => seconds += component.convert_to_seconds(),
                }
            }

            Result::Ok(TimeInterval::new(seconds))
        }

        pub fn add(&self, other: &TimeInterval) -> TimeInterval {
            TimeInterval::new(self.seconds + other.seconds)
        }

        pub fn as_string(&self) -> String {
            let mut components: Vec<TimeUnit> = Vec::new();
            components.push(TimeUnit::Seconds(self.seconds % 60));

            let minutes = self.seconds / 60;
            components.push(TimeUnit::Minutes(minutes % 60));

            let hours = minutes / 60;
            components.push(TimeUnit::Hours(hours));

            components.retain(TimeUnit::is_not_empty);

            components.reverse();

            components.iter()
                .map(TimeUnit::format)
                .collect::<Vec<String>>().join(" ")
        }
    }
}