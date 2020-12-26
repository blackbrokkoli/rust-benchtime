use cli_table::{
    print_stdout, Table, WithTitle,
};

pub struct BenchmarkStorage {
    timestamps: Vec<Timestamp>,
}

struct Timestamp {
    name: String,
    timestamp: i64,
}

#[derive(Table)]
struct Period {
    #[table(name = "Start")]
    start: &'static str,
    #[table(name = "End")]
    end: &'static str,
    #[table(name = "Duration in ms")]
    duration: i64,
}


impl BenchmarkStorage {
    pub fn init() -> BenchmarkStorage {
        BenchmarkStorage {timestamps: vec![]}
    }
    pub fn add(&mut self, name: &str) {
        let timestamp = Timestamp {
            name: name.to_string(),
            timestamp: chrono::offset::Local::now().timestamp_millis(),
        };
        self.timestamps.push(timestamp);

    }
    pub fn render(&self) {

        let mut periods : Vec<Period> = vec![];

        for (i, timestamp) in self.timestamps.iter().enumerate() {
            if i < self.timestamps.len() - 1 {
                let start = Box::leak(timestamp.name.clone().into_boxed_str());
                let end = Box::leak(self.timestamps[i+1 as usize].name.clone().into_boxed_str());

                let period = Period {
                    start,
                    end,
                    duration: self.timestamps[i+1 as usize].timestamp - timestamp.timestamp,
                };
                periods.push(period);
            }
        }

        print_stdout(periods.with_title());
    }
}