const MAX_HTML_LENGTH: usize = 5_000_000;
// const CONTENT_SAMPLE_LENGTH: usize = 5_000_000;
const TEXT_SAMPLE_LENGTH: usize = 5_000_000;

extern crate reqwest;
extern crate scraper;
extern crate select;
extern crate whatlang;

use super::{DnsInfo, Domain, ScannerResult};
// use select::document::Document;
// use select::predicate::Name;

use super::wappalyzer;
use reqwest::header::HeaderMap;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::io::Read;
use crate::Cookie;
use std::str;
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

impl PageInfo {
  pub async fn from(domain: &Domain, _: &DnsInfo) -> ScannerResult<PageInfo> {
    front_page_scan(domain).await
  }

  // pub fn cookies() -> ? {

  // }
}

// the base data that is used to analyze the page
pub struct RawData {
  pub headers: reqwest::header::HeaderMap,
  pub cookies: Vec<crate::Cookie>,
  pub meta_tags: HashMap<String, String>,
  pub script_tags: Vec<String>,
  // pub parsed_html: Html,
  pub html: String,
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

pub async fn front_page_scan(domain: &Domain) -> ScannerResult<PageInfo> {
  // use std::io::Write;

  let url = format!("http://{}", domain.0);
  let now = Instant::now();

  // let res = reqwest::Client::new().cookie_store(true).get(&url).await?;
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
  // let res = reqwest::get(&url).await?;
  let load_time = now.elapsed();

  let status_code = res.status().to_string();
  if !res.status().is_success() {
    // TODO: failure
  }

  let headers = res.headers().clone();

  // process body
  // let mut buffer = [0; MAX_HTML_LENGTH];
  let html_string = res.text().await?;
  let content_length = html_string.len();
  // let content_length = res.read(&mut buffer)? as usize;
  // let html_string = if content_length < MAX_HTML_LENGTH {
  //   str::from_utf8(&buffer[0..content_length])?
  // } else {
  //   str::from_utf8(&buffer)?
  // };

  let parsed_html = Html::parse_fragment(&html_string);

  let selector = Selector::parse("meta").unwrap();

  let mut script_tags = vec![];
  for js in parsed_html.select(&Selector::parse("script").unwrap()) {
    // eprintln!("\n==============\n{}\n", js.html());
    script_tags.push(js.html());
  }

  // TODO: using a hashmap will not support two meta tags with the same name and different values,
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
    // parsed_html,
    script_tags,
    html: html_string.clone(),
  });

  let techs = wappalyze(raw_data).await;

  let page_content = "".to_string();
  // let page_content = if html_string.len() > CONTENT_SAMPLE_LENGTH {
  //   html_string[0..CONTENT_SAMPLE_LENGTH].to_string()
  // } else {
  //   html_string.to_string()
  // };

  // Headers(/Cookies), HTML(/Meta)

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
  // let fragment = Html::parse_fragment(html);
  // let document = Document::from(html);

  // // use scraper::{Html, Selector};
  let parsed_html = Html::parse_fragment(html);
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
  if let Some(body) = parsed_html.select(&selector).next() {
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

// fn wappalyze(response: &reqwest::Response, body: &str) -> Vec<wappalyzer::Tech> {
//   wappalyzer::Site::new(body).check()
// }
async fn wappalyze(
  raw_data: Arc<RawData>,
  // headers: Arc<HeaderMap>,
  // cookies: Arc<Vec<Cookie>>,
  // meta_tags: Arc<HashMap<String, String>>,
  // parsed_html: Arc<Html>,
  // body: Arc<String>,
) -> Vec<wappalyzer::Tech> {
  // wappalyzer::Site::new(body).check()
  // wappalyzer::check(raw_data, headers, cookies, meta_tags, parsed_html, body).await
  wappalyzer::check(raw_data).await
}

/*
{
  "scan_type": "core",
  "domain": "daybreakzambia.com",
  "load_time": 0,
  "state": "published",
  "content_length": 143,
  "techs": [
    "WebServers/Apache"
  ],
  "scan_time": "2019-09-11T22:03:19.783684-04:00"
}
{
  "scan_type": "core",
  "domain": "whois.domaintools.com",
  "load_time": 0,
  "state": "published",
  "language": "maybe English",
  "word_count": 7,
  "content_length": 71,
  "scan_time": "2019-09-11T22:03:19.898718-04:00"
}
{
  "scan_type": "core",
  "domain": "trunzoautobodyandclassiccars.com",
  "load_time": 0,
  "state": "published",
  "language": "English",
  "word_count": 182,
  "content_length": 24660,
  "image_count": 23,
  "script_count": 8,
  "techs": [
    "WebServers/Apache",
    "FontScripts/FontAwesome",
    "JavaScriptLibraries/jQuery",
    "WebFrameworks/Bootstrap",
    "JavaScriptLibraries/prettyPhoto",
    "JavaScriptLibraries/jQuery",
    "WebFrameworks/animate.css"
  ],
  "meta_description": "Daily Deals for Electronics Computers Home Tools Garden Sport Accessories Kids Shirt Wine & more",
  "meta_keywords": "HOT DEALS COUPONS Discounted Deals ON LINE Shop on line Get the best deals on the internet.",
  "scan_time": "2019-09-11T22:03:19.863292-04:00"
}
{
  "scan_type": "core",
  "domain": "womenshealthcaregj.com",
  "load_time": 0,
  "state": "published",
  "language": "English",
  "word_count": 115,
  "content_length": 6575,
  "image_count": 8,
  "script_count": 3,
  "techs": [
    "Analytics/GoogleAnalytics",
    "WebServers/Apache",
    "JavaScriptLibraries/jQuery",
    "Editors/DreamWeaver"
  ],
  "meta_description": "Women's Health Care of Western Colorado and Mesa Midwives bringing quality obstetrics gynecology and midwifery to Grand Junction and western Colorado.",
  "meta_keywords": "Women's Health Care Mesa Midwives gynecologic gynecology obstetrics obstetric obstetrics grand junction health care colorado midwives pregnancy birth control gynecologic surgery da Vinci Laparoscopy essure midwife grand junction",
  "scan_time": "2019-09-11T22:03:19.98308-04:00"
}
{
  "scan_type": "core",
  "domain": "konama.com",
  "load_time": 0,
  "state": "published",
  "language": "maybe Hausa",
  "word_count": 1,
  "content_length": 842,
  "script_count": 2,
  "techs": [
    "OperatingSystems/CentOS",
    "WebServers/Apache",
    "JavaScriptLibraries/jQuery"
  ],
  "scan_time": "2019-09-11T22:03:20.166315-04:00"
}
{
  "scan_type": "core",
  "domain": "drfangdds.com",
  "load_time": 0,
  "state": "published",
  "language": "maybe English",
  "word_count": 1273,
  "content_length": 92534,
  "image_count": 6,
  "form_count": 1,
  "script_count": 27,
  "techs": [
    "JavaScriptLibraries/jQuery",
    "FontScripts/GoogleFontAPI",
    "CMS/Squarespace",
    "Analytics/GoogleAnalytics"
  ],
  "meta_description": "Adrienne Fang DDS is your premier dental option in Valencia California. Dr. Fang specializes in family dentistry crowns and implants.",
  "scan_time": "2019-09-11T22:03:20.330339-04:00"
}
{
  "scan_type": "core",
  "domain": "ahnsahnghongisgod.com",
  "load_time": 0,
  "state": "published",
  "language": "Mandarin",
  "word_count": 277,
  "content_length": 67436,
  "image_count": 23,
  "form_count": 3,
  "script_count": 20,
  "techs": [
    "JavaScriptLibraries/jQuery",
    "JavaScriptLibraries/jQueryMigrate",
    "JavaScriptLibraries/jQuery",
    "FontScripts/GoogleFontAPI",
    "WebFrameworks/animate.css",
    "WebServers/Nginx",
    "Miscellaneous/Gravatar",
    "Analytics/GoogleAnalytics",
    "CMS/WordPress",
    "ProgrammingLanguages/PHP",
    "Databases/MySQL",
    "FontScripts/FontAwesome"
  ],
  "meta_generator": "WordPress 5.2.3",
  "scan_time": "2019-09-11T22:03:20.690736-04:00"
}
{
  "scan_type": "core",
  "domain": "xtronlabs.com",
  "load_time": 0,
  "state": "published",
  "language": "Mandarin",
  "word_count": 2,
  "content_length": 1769,
  "script_count": 3,
  "techs": [
    "WebFrameworks/MicrosoftASP.NET",
    "JavaScriptLibraries/jQuery",
    "WebServers/IIS",
    "OperatingSystems/WindowsServer"
  ],
  "meta_description": "澳门威尼人网站 专业生产各种型号环链电动葫芦及其相关系列产品 澳门威尼人网站设计和工艺采用日本先进技术 拥有完善的质量管理体系、高素质的人才、先进的技术工艺和生产设备",
  "meta_keywords": "澳门威尼人网站",
  "scan_time": "2019-09-11T22:03:21.140204-04:00"
}
{
  "scan_type": "core",
  "domain": "digi-directory.co.uk",
  "load_time": 0,
  "state": "published",
  "language": "English",
  "word_count": 64,
  "content_length": 54087,
  "image_count": 1,
  "form_count": 1,
  "script_count": 9,
  "techs": [
    "JavaScriptLibraries/jQuery",
    "JavaScriptLibraries/jQueryMigrate",
    "JavaScriptLibraries/jQuery",
    "FontScripts/GoogleFontAPI",
    "CMS/WordPress",
    "ProgrammingLanguages/PHP",
    "Databases/MySQL",
    "WebServers/Apache"
  ],
  "meta_generator": "WordPress 5.2.3",
  "scan_time": "2019-09-11T22:03:20.831106-04:00"
}
{
  "scan_type": "core",
  "domain": "harrispublicrelations.com",
  "load_time": 0,
  "state": "published",
  "language": "maybe English",
  "word_count": 979,
  "content_length": 83408,
  "image_count": 29,
  "script_count": 106,
  "techs": [
    "JavaScriptLibraries/jQuery",
    "JavaScriptLibraries/jQueryMigrate",
    "JavaScriptLibraries/jQuery",
    "JavaScriptLibraries/Modernizr",
    "JavaScriptGraphics/Chart.js",
    "FontScripts/GoogleFontAPI",
    "JavaScriptLibraries/prettyPhoto",
    "JavaScriptLibraries/jQuery",
    "Widgets/OWLCarousel",
    "JavaScriptLibraries/jQuery",
    "WebServers/Nginx",
    "Miscellaneous/Revslider",
    "CMS/WordPress",
    "CMS/WordPress",
    "ProgrammingLanguages/PHP",
    "Databases/MySQL",
    "FontScripts/FontAwesome"
  ],
  "meta_generator": "Powered by Slider Revolution 6.0.9 - responsive Mobile-Friendly Slider Plugin for WordPress with com",
  "scan_time": "2019-09-11T22:03:24.212735-04:00"
}

*/
