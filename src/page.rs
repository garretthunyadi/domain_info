const MAX_HTML_LENGTH: usize = 5_000_000;
const CONTENT_SAMPLE_LENGTH: usize = 5_000_000;
const TEXT_SAMPLE_LENGTH: usize = 5_000_000;

extern crate reqwest;
extern crate scraper;
extern crate select;
extern crate whatlang;

use super::{DnsInfo, Domain, ScannerResult};
use select::document::Document;
use select::predicate::Name;

use super::wappalyzer;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::Read;
use std::str;
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
    // headers: reqwest::header::HeaderMap,
}

impl PageInfo {
    pub fn from(domain: &Domain, _: &DnsInfo) -> ScannerResult<PageInfo> {
        front_page_scan(domain)
    }

    // pub fn cookies() -> ? {

    // }
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

fn front_page_scan(domain: &Domain) -> ScannerResult<PageInfo> {
    let url = format!("http://{}", domain.0);
    let now = Instant::now();
    let mut res = reqwest::get(&url)?;
    let load_time = now.elapsed();
    // println!("{}", now.elapsed().as_secs());

    let status_code = res.status().to_string();
    //println!("Status: {}", res.status());
    if !res.status().is_success() {
        // TODO: failure
    }
    // println!("Headers:\n{:#?}", res.headers());
    // println!("Body:\n{}", body);

    // process headers
    // let headers: &reqwest::header::HeaderMap = res.headers();
    // for key in headers.keys() {
    //     println!("key/{}", key);
    // }

    let techs = wappalyze(&res);

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

    // Headers(/Cookies), HTML(/Meta)

    let mut page_text = body_text(body);
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
    })
}

///
fn body_text(html: &str) -> String {
    use scraper::{Html, Selector};

    // let fragment = Html::parse_fragment(html);
    // let document = Document::from(html);

    // // use scraper::{Html, Selector};
    let fragment = Html::parse_fragment(html);
    // let selector = Selector::parse("body").unwrap();

    // let h1 = fragment.select(&selector).next().unwrap();
    // let text = h1.text().collect::<Vec<_>>().join("|||||");

    // println!("{}", text);

    // document.find(predicate: P)

    // for node in document.find(Attr("id", "hmenus").descendant(Name("a"))) {
    //     println!("{} ({:?})", node.text(), node.attr("href").unwrap());
    // }

    // let root = fragment.root_element();
    let selector = Selector::parse("body").unwrap();
    if let Some(body) = fragment.select(&selector).next() {
        body.text().collect::<Vec<_>>().join("|||||")
    } else {
        eprintln!("(no body tag found)");
        "err2".to_string()
    }
    // let h1 = root.next().unwrap();
    // let iter = root.text();

    // assert_eq!(vec!["Hello, ", "world!"], text);
}
// func (s *Selection) Text() string {
// 	var buf bytes.Buffer

// 	// Slightly optimized vs calling Each: no single selection object created
// 	var f func(*html.Node)
// 	f = func(n *html.Node) {
// 		if n.Type == html.TextNode {
// 			// Keep newlines and spaces, like jQuery
// 			buf.WriteString(n.Data)
// 		}
// 		if n.FirstChild != nil {
// 			for c := n.FirstChild; c != nil; c = c.NextSibling {
// 				f(c)
// 			}
// 		}
// 	}
// 	for _, n := range s.Nodes {
// 		f(n)
// 	}

// 	return buf.String()
// }

fn language_for(text: &str) -> String {
    match whatlang::detect(&text) {
        Some(info) => info.lang.to_code().to_string(),
        None => "".to_string(),
    }
}

fn wappalyze(response: &reqwest::Response) -> Vec<wappalyzer::Tech> {
    vec![]
}
