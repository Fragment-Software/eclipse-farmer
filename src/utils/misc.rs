use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use term_size::dimensions;

pub fn random_in_range<T>(range: [T; 2]) -> T
where
    T: rand::distributions::uniform::SampleUniform + PartialOrd + Copy,
{
    let start = range[0];
    let end = range[1];

    let inclusive_range = if start <= end { start..=end } else { end..=start };

    rand::thread_rng().gen_range(inclusive_range)
}

pub async fn pretty_sleep(sleep_range: [u32; 2], with_progress_bar: bool) {
    let random_sleep_duration_secs = random_in_range(sleep_range);

    if with_progress_bar {
        let pb = ProgressBar::new(random_sleep_duration_secs as u64);

        let term_width = dimensions().map(|(w, _)| w - 2).unwrap_or(40);
        let bar_width = if term_width > 20 { term_width - 20 } else { 20 };

        pb.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "{{spinner:.green}} [{{elapsed_precise}}] [{{bar:{bar_width}.cyan/blue}}] {{pos}}/{{len}}s"
                ))
                .expect("Invalid progress bar template.")
                .progress_chars("#>-"),
        );

        let step = std::time::Duration::from_secs(1);

        for _ in 0..random_sleep_duration_secs {
            pb.inc(1);
            tokio::time::sleep(step).await;
        }

        pb.finish_with_message("Done!");
    } else {
        tracing::info!("Sleeping for {random_sleep_duration_secs} seconds");
        tokio::time::sleep(Duration::from_secs(random_sleep_duration_secs as u64)).await;
    }
}
