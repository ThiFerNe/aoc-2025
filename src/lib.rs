#[macro_export]
macro_rules! aoc {
    ($input_str:ident, $func_ident:ident) => {
        #[cfg(feature = "benchmark")]
        {
            const MINIMUM_DURATION: f64 = 3.0;
            const MINIMUM_COUNT: usize = 10;
            let mut overall_duration: f64 = 0.0;
            let mut durations = Vec::with_capacity(10_000);
            while overall_duration < MINIMUM_DURATION || durations.len() < MINIMUM_COUNT {
                let start = std::time::Instant::now();
                let _answer = $func_ident($input_str);
                let end = std::time::Instant::now();
                let duration = end - start;
                durations.push(duration);
                overall_duration += duration.as_secs_f64();
            }
            let count = durations.len();
            let mean = durations
                .into_iter()
                .map(|duration| duration.as_secs_f64())
                .sum::<f64>()
                / count as f64;
            println!("Mean Duration ({count} runs): {mean} seconds");
        }
        #[cfg(not(feature = "benchmark"))]
        {
            #[cfg(feature = "internal_timings")]
            let start = std::time::Instant::now();
            let answer = $func_ident($input_str);
            println!("The answer is: {answer}");
            #[cfg(feature = "internal_timings")]
            {
                let end = std::time::Instant::now();
                println!(
                    "Duration: {} seconds",
                    end.duration_since(start).as_secs_f64()
                );
            }
        }
    };
    ($input_str:ident, $func_1_ident:ident, $func_2_ident: ident) => {
        #[cfg(feature = "benchmark")]
        {
            const MINIMUM_DURATION: f64 = 3.0;
            const MINIMUM_COUNT: usize = 10;
            let mut overall_duration: f64 = 0.0;
            let mut durations = Vec::with_capacity(10_000);
            while overall_duration < MINIMUM_DURATION || durations.len() < MINIMUM_COUNT {
                let start = std::time::Instant::now();
                #[cfg(feature = "part1")]
                {
                    let _answer = $func_1_ident($input_str);
                }
                #[cfg(feature = "part2")]
                {
                    let _answer = $func_2_ident($input_str);
                }
                let end = std::time::Instant::now();
                let duration = end - start;
                durations.push(duration);
                overall_duration += duration.as_secs_f64();
            }
            let count = durations.len();
            let mean = durations
                .into_iter()
                .map(|duration| duration.as_secs_f64())
                .sum::<f64>()
                / count as f64;
            println!("Mean Duration ({count} runs): {mean} seconds");
        }
        #[cfg(not(feature = "benchmark"))]
        {
            #[cfg(feature = "internal_timings")]
            let start = std::time::Instant::now();
            #[cfg(feature = "part1")]
            {
                let answer = $func_1_ident($input_str);
                println!("The answer to part 1 is: {answer}");
            }
            #[cfg(feature = "part2")]
            {
                let answer = $func_2_ident($input_str);
                println!("The answer to part 2 is: {answer}");
            }
            #[cfg(feature = "internal_timings")]
            {
                let end = std::time::Instant::now();
                println!(
                    "Duration: {} seconds",
                    end.duration_since(start).as_secs_f64()
                );
            }
        }
    };
}
