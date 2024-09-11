use ical::parser::Component;
use jiff::civil::Date;
use jiff::Timestamp;
use scraper::Html;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::str::FromStr;

#[derive(serde::Serialize, PartialEq, Eq, Hash)]
struct Event {
    date: String,
    name: String,
}

// impl std::hash::Hash for Event {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.date.hash(state);
//         self.name.hash(state);
//     }
// }

// impl PartialEq for Event {
//     fn eq(&self, other: &Self) -> bool {
//         self.date == other.date && self.name == other.name
//     }
// }

// impl Eq for Event {

// }

#[derive(serde::Serialize)]
struct Cal {
    cal: Vec<Event>,
}

fn download_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let body = response.text()?;
    Ok(body)
}

fn scrape_html(fragment: &Html) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    let event_selector = scraper::Selector::parse("div.events_list_item")?;

    let event_details_selector = scraper::Selector::parse("div.events_list_item_text")?;
    let date_selector = scraper::Selector::parse("b")?;
    let name_selector = scraper::Selector::parse("h1")?;

    let mut events = Vec::new();

    for event_element in fragment.select(&event_selector) {
        let mut event_details = event_element.select(&event_details_selector);
        let date_element = event_details.next().expect("failed to scrape date");
        let name_element = event_details.next().expect("failed to scrape name");

        let date = date_element
            .select(&date_selector)
            .next()
            .expect("failed to scrape date")
            .text()
            .collect::<String>();
        let name = name_element
            .select(&name_selector)
            .next()
            .expect("failed to scrape date")
            .text()
            .collect::<String>();
        let name = name.replace("–", "-");
        let date = Date::strptime("%d/%m/%Y", date)?;
        let date = format!("{}", date.strftime("%Y-%m-%d"));
        let e = Event { date, name };
        events.push(e);
    }

    Ok(events)
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
            let summary_prop = event
                .get_property("SUMMARY")
                .expect("failed to get SUMMARY property");
            let summary: &String = summary_prop
                .value
                .as_ref()
                .expect("failed to get SUMMARY value");
            let start_prop = event
                .get_property("DTSTART")
                .expect("failed to get DTSTART property");
            let start: &String = start_prop
                .value
                .as_ref()
                .expect("failed to get DTSTART value");
            let date = match Timestamp::from_str(start) {
                Ok(t) => {
                    format!("{}", t.strftime("%Y-%m-%d"))
                }
                Err(_) => {
                    let date = Date::from_str(start)
                        .expect("failed to parse Timestamp from start propery");
                    format!("{}", date.strftime("%Y-%m-%d"))
                }
            };

            let e = Event {
                date,
                name: summary.clone(),
            };
            events.push(e);
        }
    }

    events
}

fn scrape_ical(fragment: &Html) -> Result<String, Box<dyn std::error::Error>> {
    let cal_selector = scraper::Selector::parse("div.termine-cal")?;
    let a_selector = scraper::Selector::parse("a")?;
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

    if ical_links.len() > 1 {
        let mut i = 0;
        ical_links.into_iter().for_each(|f| {
            let fname = format!("ical{}.ical", i);
            let mut file = File::create(fname).expect("failed to create file");
            file.write_all(f.as_bytes()).expect("failed to write file");
            i = i+1;
        });

        panic!("differing ical files found"); // yes, they currently provide the same ical file for all events
    }
    let link = ical_links.iter().next().expect("no ical files found");
    let ical_file = download_html(link)?;
    Ok(ical_file)
}

fn merge(events1: Vec<Event>, events2: Vec<Event>) -> Cal {
    let mut events: HashSet<Event> = HashSet::new();
    // let x1 = Event { name: "foo".to_string(), date: "2024-10-05".to_string()};
    // let x2 = Event { name: "foo".to_string(), date: "2024-10-05".to_string()};
    // events.insert(x1);
    // events.insert(x2);
    events1.into_iter().for_each(|e| {
        events.insert(e);
        ()
    });
    println!("after events1 {}", events.len());
    events2.into_iter().for_each(|e| {
        events.insert(e);
        ()
    });
    println!("after events2 {}", events.len());
    let cal_events: Vec<Event> = events.into_iter().collect();
    Cal { cal: cal_events }
}

fn main() {
    let url = "https://www.rheinenergiestadion.de/termine";
    let html = download_html(url).expect("failed to download html");
    let fragment = Html::parse_fragment(&html);
    let ical = scrape_ical(&fragment).unwrap();
    let ical_events = parse_ical(ical);
    println!("ical------");
    ical_events
        .iter()
        .for_each(|e| println!("{} {}", e.name, e.date));
    let website_events = scrape_html(&fragment).expect("failed to scrape html");
    println!("website------");
    website_events
        .iter()
        .for_each(|e| println!("{} {}", e.name, e.date));
    println!("found {} events on website", website_events.len());
    println!("found {} events in ical file", ical_events.len());
    let cal = merge(website_events, ical_events);
    println!("{} merged events", cal.cal.len());
    let json = serde_json::to_string(&cal).expect("failed to serialize");
    let mut file = File::create("cal.json").expect("failed to create file");
    file.write_all(json.as_bytes())
        .expect("failed to write file");
}
