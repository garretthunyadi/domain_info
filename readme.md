
# domain_info

An early-stage crate and tool to fetch information about a domain, primarily by looking at the information on the front page of the domain.

From the front page, it gets the load time, language, word count, image/form/script counts and it uses the Wappalizer project rules to identify technilogies used on the front page.

It also does a reverse dns lookup on the host and attempts to determine the host company/platform (E.g GoDaddy, Bluehost, AWS)

Additionally it gets the mail server hosts and whois information about the domain.

```sh
domain_info google.com
```

```json
{
  "domain": "google.com",
  "dns_info": {
    "ip": "172.217.6.238",
    "other_ips": []
  },
  "host_info": {
    "host": "lga25s55-in-f238.1e100.net",
    "host_tld": "1e100.net",
    "platform": "Google"
  },
  "front_page_info": {
    "status_code": "200 OK",
    "load_time": {
      "secs": 0,
      "nanos": 217272393
    },
    "word_count": 100,
    "content_length": 47090,
    "techs": [
      {
        "category": "Web Servers",
        "name": "Google Web Server"
      },
      {
        "category": "JavaScript Frameworks",
        "name": "ExtJS"
      },
      {
        "category": "JavaScript Libraries",
        "name": "List.js"
      }
    ],
    "page_content": "",
    "page_text": "",
    "language": "",
    "iframe_count": 0,
    "image_count": 1,
    "form_count": 1,
    "script_count": 8
  },
  "mx_info": {
    "servers": [
      "alt2.aspmx.l.google.com",
      "alt3.aspmx.l.google.com",
      "aspmx.l.google.com",
      "alt1.aspmx.l.google.com",
      "alt4.aspmx.l.google.com"
    ]
  }
}```

```sh
cat domains.txt | domain_info | jq
```

## Status
An initial implementation is there and it performs adequatily for small lists of domains.  More work and testing needs to be done when running against large lists of domains.


## TODO
- [ ] Better domain validation
- [ ] Executable will silently hang if there is no argument and no piped input.  I need to find a mechanism in Rust to check if there is any input for stdin.
- [ ] Extract Wappalyzer into its own crate
- [ ] Extract the reverse dns host company/brand into its own crate

## Notes

The Wappalyzer-based functionalty is somewhat limited as we are not running the rules from within a headless browser, but only against the initially-returned html from the main page.
