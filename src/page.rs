const TEXT_SAMPLE_LENGTH: usize = 5_000_000;

extern crate reqwest;
extern crate scraper;
extern crate select;
extern crate whatlang;

use super::wappalyzer;
use super::{Domain, ScannerResult};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PageInfo {
  status_code: String,
  load_time: Duration,
  word_count: u64,
  content_length: u64,
  techs: Vec<wappalyzer::Tech>,
  page_content: String,
  page_text: String,
  language: String,
  iframe_count: usize,
  image_count: usize,
  form_count: usize,
  script_count: usize,
  // headers: reqwest::header::HeaderMap,
}

// impl PageInfo {
//   pub async fn from(domain: &Domain, _: &DnsInfo) -> ScannerResult<PageInfo> {
//     front_page_scan(domain).await
//   }
// }

// the base data that is used to analyze the page
pub struct RawData {
  pub headers: reqwest::header::HeaderMap,
  pub cookies: Vec<crate::Cookie>,
  pub meta_tags: HashMap<String, String>,
  pub script_tags: Vec<String>,
  pub html: String,
}

pub async fn front_page_scan(domain: &Domain) -> ScannerResult<PageInfo> {
  let url = format!("http://{}", domain.0);
  let now = Instant::now();

  let client = reqwest::Client::new();
  let res = client.get(&url).send().await.unwrap();

  let mut cookies = vec![];
  {
    let cs: std::vec::Vec<reqwest::cookie::Cookie<'_>> = res.cookies().collect::<Vec<_>>();
    for c in cs {
      cookies.push(crate::Cookie {
        name: String::from(c.name()),
        value: String::from(c.value()),
      });
    }
  }
  let load_time = now.elapsed();

  let status_code = res.status().to_string();
  if !res.status().is_success() {
    // TODO: failure
  }

  let headers = res.headers().clone();

  let html_string = res.text().await?;
  let content_length = html_string.len();

  let parsed_html = Html::parse_fragment(&html_string);

  let selector = Selector::parse("meta").unwrap();

  let mut script_tags = vec![];
  for js in parsed_html.select(&Selector::parse("script").unwrap()) {
    script_tags.push(js.html());
  }

  // Note: using a hashmap will not support two meta tags with the same name and different values,
  // though I'm not sure if that's legal html.
  let mut meta_tags = HashMap::new();
  for meta in parsed_html.select(&selector) {
    if let (Some(name), Some(content)) = (meta.value().attr("name"), meta.value().attr("content")) {
      eprintln!("META {} -> {}", name, content);
      meta_tags.insert(String::from(name), String::from(content));
    }
  }

  let iframe_count = count_selector(&parsed_html, "iframe");
  let image_count = count_selector(&parsed_html, "img");
  let form_count = count_selector(&parsed_html, "form");
  let script_count = count_selector(&parsed_html, "script");

  let raw_data = Arc::new(RawData {
    headers,
    cookies,
    meta_tags,
    script_tags,
    html: html_string.clone(),
  });

  let techs = wappalyze(raw_data).await;

  let page_content = "".to_string();

  let mut page_text = body_text(&html_string);

  let language = language_for(&page_text);
  page_text.truncate(TEXT_SAMPLE_LENGTH);

  Ok(PageInfo {
    status_code,
    word_count: 100,
    load_time,
    content_length: content_length as u64,
    techs,
    page_text,
    page_content,
    language,
    iframe_count,
    image_count,
    form_count,
    script_count,
  })
}

fn count_selector(parsed_html: &Html, selector: &str) -> usize {
  parsed_html
    .select(&Selector::parse(selector).unwrap())
    .count()
}

///
fn body_text(html: &str) -> String {
  let parsed_html = Html::parse_fragment(html);

  let selector = Selector::parse("body").unwrap();
  if let Some(body) = parsed_html.select(&selector).next() {
    body.text().collect::<Vec<_>>().join("|||||")
  } else {
    eprintln!("(no body tag found)");
    "".to_string()
  }
}

fn language_for(text: &str) -> String {
  match whatlang::detect(&text) {
    Some(info) => info.lang.to_code().to_string(),
    None => "".to_string(),
  }
}

async fn wappalyze(raw_data: Arc<RawData>) -> Vec<wappalyzer::Tech> {
  wappalyzer::check(raw_data).await
}
