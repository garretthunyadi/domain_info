const MAX_HTML_LENGTH: usize = 100_000;
const CONTENT_SAMPLE_LENGTH: usize = 2_000;
const TEXT_SAMPLE_LENGTH: usize = 2_000;

extern crate reqwest;
extern crate scraper;
extern crate select;
extern crate whatlang;

use super::{DnsInfo, Domain, ScannerResult};
use select::document::Document;
use select::predicate::Name;

use std::env;
use std::io::Read;
use std::str;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PageInfo {
    status_code: String,
    load_time: Duration,
    word_count: u64,
    content_length: u64,
    techs: Vec<Tech>,
    page_content: String,
    page_text: String,
    language: String,
}

impl PageInfo {
    pub fn from(domain: &Domain, _: &DnsInfo) -> ScannerResult<PageInfo> {
        front_page_scan(domain)
    }
}

// type ScanResult struct {
// 	Domain          string    `json:"domain"`
// 	LoadTime        int16     `json:"load_time"` // (seconds)
// 	State           string    `json:"state,omitempty"`
// 	SubState        string    `json:"substate,omitempty"`
// 	Language        string    `json:"language,omitempty"`
// 	WordCount       int32     `json:"word_count,omitempty"`
// 	ContentLength   int64     `json:"content_length,omitempty"`
// 	ImageCount      int16     `json:"image_count,omitempty"`
// 	FormCount       int16     `json:"form_count,omitempty"`
// 	IFrameCount     int16     `json:"iframe_count,omitempty"`
// 	ScriptCount     int16     `json:"script_count,omitempty"`
// 	Techs           []string  `json:"techs,omitempty"`
// 	PageContent     string    `json:"page_content,omitempty"`
// 	PageText        string    `json:"page_text,omitempty"`
// 	PageTitle       string    `json:"page_title,omitempty"`
// 	MetaDescription string    `json:"meta_description,omitempty"`
// 	MetaKeywords    string    `json:"meta_keywords,omitempty"`
// 	MetaAuthor      string    `json:"meta_author,omitempty"`
// 	MetaGenerator   string    `json:"meta_generator,omitempty"`
// 	ScanTime        time.Time `json:"scan_time"`
// 	ErrorString     string    `json:"error_string,omitempty"`
// 	ErrorCode       string    `json:"error_code,omitempty"`
// }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tech {
    category: String,
    name: String,
}

fn front_page_scan(domain: &Domain) -> ScannerResult<PageInfo> {
    let url = format!("http://{}", domain.0);
    let now = Instant::now();
    let mut res = reqwest::get(&url)?;
    let load_time = now.elapsed();
    // println!("{}", now.elapsed().as_secs());

    let status_code = res.status().to_string();
    println!("Status: {}", res.status());
    if !res.status().is_success() {
        // TODO: failure
    }
    // println!("Headers:\n{:#?}", res.headers());
    // println!("Body:\n{}", body);

    // process headers
    let _headers: &reqwest::header::HeaderMap = res.headers();
    // for key in headers.keys() {
    //     println!("key/{}", key);
    // }

    // process body
    let mut buffer = [0; MAX_HTML_LENGTH];
    let content_length = res.read(&mut buffer)? as usize;

    let body = if content_length < MAX_HTML_LENGTH {
        str::from_utf8(&buffer[0..content_length])?
    } else {
        str::from_utf8(&buffer)?
    };
    let page_content = if body.len() > CONTENT_SAMPLE_LENGTH {
        body[0..CONTENT_SAMPLE_LENGTH].to_string()
    } else {
        body.to_string()
    };
    // let page_content = if body.len() > CONTENT_SAMPLE_LENGTH {
    //     str::from_utf8(&body[0..CONTENT_SAMPLE_LENGTH])?
    // } else {
    //     str::from_utf8(&body)?
    // };
    
    let mut page_text = body_text(body);
    let language = language_for(&page_text);
    page_text.truncate(TEXT_SAMPLE_LENGTH);

    Ok(PageInfo {
        status_code,
        word_count: 100,
        load_time,
        content_length: content_length as u64,
        techs: vec![],
        page_text,
        page_content,
        language,
    })
}

///
fn body_text(html: &str) -> String {
    use scraper::{Html, Selector};

    let fragment = Html::parse_fragment(html);
    let root = fragment.root_element();
    // let selector = Selector::parse("body").unwrap();

    // let body = fragment.select(&selector).next().unwrap();
    // let h1 = root.next().unwrap();
    root.text().collect::<Vec<_>>().join(" ")

    // assert_eq!(vec!["Hello, ", "world!"], text);
}

fn language_for(text: &str) -> String {
    match whatlang::detect(&text) {
        Some(info) => info.lang.to_code().to_string(),
        None => "".to_string()
    }
}
