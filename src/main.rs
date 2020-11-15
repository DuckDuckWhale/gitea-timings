use std::collections::HashMap;
use std::fs;

use serde::Deserialize;
use unicode_width::UnicodeWidthStr;

#[derive(Deserialize)]
struct TrackedTime {
	time: u64,
	issue: Issue,
}

#[derive(Deserialize)]
struct Issue {
	id: u64,
	title: String,
}

fn main() {
	let token = fs::read_to_string("time-tracker-token")
		.expect("Failed to read token from `time-tracker-token`.");
	let response = ureq::get("https://git.duckduckwhale.com/api/v1/user/times")
		.query("since", "2020-11-12T01:18:22-08:00")
		.set("Authorization", format!("token {}", token).as_ref())
		.timeout_connect(10_000)
		.call();

	if !response.ok() {
		println!(
			"error {}: {}",
			response.status(),
			response
				.into_string()
				.expect("Failed to parse response as UTF-8 string.")
		);
		return;
	}

	let times = response
		.into_json_deserialize::<Vec<TrackedTime>>()
		.expect("Failed to parse response as UTF-8 string.");
	let seconds: u64 = times.iter().map(|tracked| tracked.time).sum();
	let mut map = HashMap::new();
	for tracked in times {
		let (time, _) = map
			.entry(tracked.issue.id)
			.or_insert((0, tracked.issue.title));
		*time += tracked.time;
	}
	let mut times: Vec<(u64, String)> = map.into_iter().map(|(_, value)| value).collect();
	times.sort_unstable_by(|a, b| a.0.cmp(&b.0).reverse().then(a.1.cmp(&b.1)));

	println!(
		"You spent {} hours and {} minutes on {} issues in the last 24 hours{}",
		seconds / 3600,
		(seconds + 30) / 60 % 60,
		times.len(),
		if times.is_empty() { '.' } else { ':' }
	);
	if let Some(max_title_len) = times.iter().map(|(_, title)| title.len()).max() {
		let max_len = max_title_len + 2;
		let separator = format!("+----------+{}+", "-".repeat(max_len));
		println!("{}", separator);
		for (time, title) in times {
			let line = format!(
				"| {:0>2}:{:0>2}:{:0>2} | {}",
				time / 3600,
				time / 60 % 60,
				time % 60,
				title
			);
			println!(
				"{}{}|",
				line,
				" ".repeat(separator.len() - line.width() - 1)
			);
		}
		println!("{}", separator);
	}
}
