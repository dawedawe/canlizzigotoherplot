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

fn scrape_html(fragment: Html) -> Result<String, Box<dyn std::error::Error>> {
    let event_selector = scraper::Selector::parse("div.events_list_item")?;

    let event_details_selector = scraper::Selector::parse("div.events_list_item_text")?;
    let date_selector = scraper::Selector::parse("b")?;
    let name_selector = scraper::Selector::parse("h1")?;

    let mut cal = Cal { cal: Vec::new() };

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
        let date = Date::strptime("%d/%m/%Y", date)?;
        let date = format!("{}", date.strftime("%Y-%m-%d"));
        let e = Event { date, name };
        cal.cal.push(e);
    }

    let json = serde_json::to_string(&cal)?;
    return Ok(json);
}

fn main() {
    let url = "https://www.rheinenergiestadion.de/termine";
    let html = download_html(url).unwrap();
    let fragment = Html::parse_fragment(&html);
    let json = scrape_html(fragment).expect("failed to scrape the html");
    let mut file = File::create("cal.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
