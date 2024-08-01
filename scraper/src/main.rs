use jiff::civil::Date;
use scraper::Html;
use std::fs::File;
use std::io::Write;

#[derive(serde::Serialize)]
struct Event {
    date: String,
    name: String,
}

#[derive(serde::Serialize)]
struct Cal {
    cal: Vec<Event>,
}

fn download_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let body = response.text()?;
    Ok(body)
}

fn main() {
    let url = "https://www.rheinenergiestadion.de/termine";
    let html = download_html(url).unwrap();

    let fragment = Html::parse_fragment(&html);
    let event_selector = scraper::Selector::parse("div.events_list_item").unwrap();

    let event_details_selector = scraper::Selector::parse("div.events_list_item_text").unwrap();
    let date_selector = scraper::Selector::parse("b").unwrap();
    let name_selector = scraper::Selector::parse("h1").unwrap();

    let mut cal = Cal { cal: Vec::new() };

    for event_element in fragment.select(&event_selector) {
        let mut event_details = event_element.select(&event_details_selector);
        let date_element = event_details.next().unwrap();
        let name_element = event_details.next().unwrap();

        let date = date_element
            .select(&date_selector)
            .next()
            .unwrap()
            .text()
            .collect::<String>();
        let name = name_element
            .select(&name_selector)
            .next()
            .unwrap()
            .text()
            .collect::<String>();
        let date = Date::strptime("%d/%m/%Y", date).unwrap();
        let date = format!("{}", date.strftime("%Y-%m-%d"));
        let e = Event { date, name };
        cal.cal.push(e);
    }

    let json = serde_json::to_string(&cal).unwrap();
    let mut file = File::create("cal.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
