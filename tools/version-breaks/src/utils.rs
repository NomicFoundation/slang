use indicatif::{ProgressBar, ProgressFinish, ProgressStyle};
use rayon::prelude::*;
use reqwest::{blocking::Client, Url};
use std::{io::Cursor, path::PathBuf, time::Duration};

pub fn with_progress<TInput: Send + Sync, TOutput: Send + Sync>(
    title: &str,
    inputs: &Vec<TInput>,
    callback: fn(&TInput) -> TOutput,
) -> Vec<TOutput> {
    let style = ProgressStyle::default_bar()
        .template(&format!(
            "{} {} {}",
            "⌛", title, "[{wide_bar:.cyan/blue}] {pos}/{len} | {elapsed_precise}"
        ))
        .progress_chars("#>-")
        .on_finish(ProgressFinish::AndLeave);

    let bar = ProgressBar::new(inputs.len().try_into().unwrap());
    bar.set_style(style.clone());

    let result: Vec<TOutput> = inputs
        .iter()
        .par_bridge()
        .map(callback)
        .map(|output| {
            bar.inc(1);
            return output;
        })
        .collect();

    bar.set_style(style.template(&format!(
        "{} {} {}",
        "✅", title, "[{wide_bar:.cyan/blue}] {pos}/{len} | {elapsed_precise}"
    )));

    bar.finish();
    return result;
}

pub fn download_file(remote_url: &Url, local_path: &PathBuf) {
    let retries = 5;
    for _ in 0..retries {
        let one_minute = Duration::from_secs(5 * 60);
        let client = Client::builder().timeout(one_minute).build().unwrap();

        let response = match client.get(remote_url.clone()).send() {
            Ok(response) => response,
            Err(error) => {
                if error.is_timeout() {
                    continue;
                } else {
                    panic!("{:?}", error)
                }
            }
        };

        let mut file = std::fs::File::create(local_path).unwrap();
        let mut content = Cursor::new(response.bytes().unwrap());
        std::io::copy(&mut content, &mut file).unwrap();
        return;
    }

    panic!("Timed out {} times downloading: {}", retries, remote_url);
}
