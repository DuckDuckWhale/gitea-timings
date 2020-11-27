use std::collections::HashMap;
use std::fs;

use chrono::{Duration as OldDuration, SecondsFormat, Utc};
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
	let since = Utc::now()
		.checked_sub_signed(OldDuration::hours(24))
		.expect("Overflowed while trying to subtract 24 hours from now.");
	let response = ureq::get("https://git.duckduckwhale.com/api/v1/user/times")
		.query(
			"since",
			since.to_rfc3339_opts(SecondsFormat::Secs, true).as_ref(),
		)
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
	// round up if more then half a minute is left
	let minutes = (seconds + 30) / 60;
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
		minutes / 60,
		minutes % 60,
		times.len(),
		if times.is_empty() { '.' } else { ':' }
	);
	if let Some(max_title_len) = times.iter().map(|(_, title)| title.width()).max() {
		let max_len = max_title_len + 2;
		println!("┌──────────┬{}┐", "─".repeat(max_len));
		for (time, title) in times {
			let line = format!(
				"│ {:0>2}:{:0>2}:{:0>2} │ {}",
				time / 3600,
				time / 60 % 60,
				time % 60,
				title
			);
			println!("{}{}│", line, " ".repeat(max_len + 13 - line.width() - 1));
		}
		println!("└──────────┴{}┘", "─".repeat(max_len));
	}
}
