use ical::parser::Component;
use jiff::Timestamp;
use jiff::Zoned;
use jiff::civil::Date;
use scraper::Html;
use std::collections::HashSet;
use std::fs::File;
use std::hash::RandomState;
use std::io::BufReader;
use std::io::Write;
use std::str::FromStr;
use std::time::Duration;

#[derive(serde::Serialize, PartialEq, Eq, Hash)]
struct Event {
    date: String,
    name: String,
    url: String,
}

#[derive(serde::Serialize)]
struct Cal {
    cal: Vec<Event>,
}

async fn download(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("downloading {}", url);
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn download_sync(
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    println!("downloading {}", url);
    let response = client.get(url).send()?;
    let body = response.text()?;
    Ok(body)
}

fn parse_ical(s: String) -> Vec<Event> {
    let reader = BufReader::new(s.as_bytes());
    let parser = ical::IcalParser::new(reader);
    let mut events = Vec::new();
    for line in parser {
        let ical_events = match line {
            Ok(l) => l.events,
            _ => panic!(""),
        };
        for event in ical_events {
            let start_prop = event
                .get_property("DTSTART")
                .expect("failed to get DTSTART property");
            let start_value: &String = start_prop
                .value
                .as_ref()
                .expect("failed to get DTSTART value");
            let date = match Timestamp::from_str(start_value) {
                Ok(t) => {
                    format!("{}", t.strftime("%Y-%m-%d"))
                }
                Err(_) => {
                    let date = Date::from_str(start_value)
                        .expect("failed to parse Timestamp from start property");
                    format!("{}", date.strftime("%Y-%m-%d"))
                }
            };

            let url_prop = event
                .get_property("URL")
                .expect("failed to get URL property");
            let url_value: &String = url_prop.value.as_ref().expect("failed to get URL value");

            let summary_prop = event
                .get_property("SUMMARY")
                .expect("failed to get SUMMARY property");
            let summary_value: &String = summary_prop
                .value
                .as_ref()
                .expect("failed to get SUMMARY value");

            let e = Event {
                date,
                name: summary_value.clone(),
                url: url_value.clone(),
            };
            events.push(e);
        }
    }

    events
}

fn scrape_ical_links(fragment: &Html) -> Vec<&str> {
    let cal_selector = scraper::Selector::parse("div.termine-cal").expect("::parse failed");
    let a_selector = scraper::Selector::parse("a").expect("::parse failed");
    let mut ical_links = HashSet::new();
    for x in fragment.select(&cal_selector) {
        let mut a = x.select(&a_selector);
        let link = a
            .next()
            .unwrap()
            .attr("href")
            .expect("failed to get href attr in a tag");
        ical_links.insert(link);
    }

    ical_links.into_iter().collect()
}

fn get_events_from_ical_link(ical_url: &str) -> Vec<Event> {
    let client = reqwest::blocking::ClientBuilder::new()
        .connect_timeout(Duration::from_secs(30))
        .build()
        .unwrap();
    let ical_content = download_sync(&client, ical_url).unwrap();
    parse_ical(ical_content)
}

fn get_events_from_ical_links(ical_urls: Vec<&str>) -> Vec<Event> {
    let (tx, rx) = std::sync::mpsc::channel();
    for url in ical_urls {
        let url = url.to_string();
        let tx_c = tx.clone();
        std::thread::spawn(move || {
            let events_of_u = get_events_from_ical_link(&url);
            println!("got {} events from {}", events_of_u.len(), url);
            tx_c.send(events_of_u)
                .expect("expected channel send to succeed");
        });
    }
    drop(tx);
    let mut events: Vec<Event> = Vec::new();
    for evs in rx {
        println!("rec {} events", evs.len());
        events.extend(evs);
    }

    events
}

fn filter_events(events: Vec<Event>) -> Vec<Event> {
    let unique_events: HashSet<Event, RandomState> = HashSet::from_iter(events);
    let current_date = Zoned::now().date();
    let mut future_events: Vec<Event> = unique_events
        .into_iter()
        .filter(|event| {
            let date =
                Date::from_str(&event.date).expect("failed to parse Timestamp from start property");
            date >= current_date
        })
        .collect();

    future_events.sort_by_cached_key(|e| Date::from_str(&e.date).unwrap());
    future_events
}

#[tokio::main]
async fn main() {
    let url = "https://www.rheinenergiestadion.de/termine";
    let html = download(url).await.expect("failed to download html");
    let fragment = Html::parse_fragment(&html);
    let ical_links = scrape_ical_links(&fragment);
    let ical_events = get_events_from_ical_links(ical_links);
    let ical_events = filter_events(ical_events);
    ical_events
        .iter()
        .for_each(|e| println!("{} {}", e.date, e.name));
    let cal = Cal { cal: ical_events };
    let json = serde_json::to_string(&cal).expect("failed to serialize");
    let mut file = File::create("cal.json").expect("failed to create file");
    file.write_all(json.as_bytes())
        .expect("failed to write file");
}
