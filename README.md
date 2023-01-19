# rust-syndication

[![Cargo Version](https://img.shields.io/crates/v/syndication.svg)](https://crates.io/crates/syndication)
[![Build Status](https://travis-ci.org/rust-syndication/syndication.svg?branch=master)](https://travis-ci.org/rust-syndication/syndication)

Library for serializing Atom and RSS web feeds. Wraps around [rust-atom](https://github.com/rust-syndication/atom) and [rust-rss](https://github.com/rust-syndication/rss).

## Usage

### Reading
```rust
let atom_str = r#"
<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <id>urn:uuid:b3420f84-6bdf-4f46-a225-f1b9a14703b6</id>
  <title>TechCrunch</title>
  <updated>2019-04-01T07:30:00Z</updated>
  <entry>
    <id>urn:uuid:4ae8550b-2987-49fa-9f8c-54c180c418ac</id>
    <title>Ford hires Elon Musk as CEO</title>
    <updated>2019-04-01T07:30:00Z</updated>
  </entry>
</feed>
"#;

match atom_str.parse::<Feed>().unwrap() {
    Feed::Atom(atom_feed) => println!("Atom feed first entry: {:?}", atom_feed.entries()[0].title()),
    _ => {}
};

let rss_str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>TechCrunch</title>
    <link>http://techcrunch.com</link>
    <description>The latest technology news and information on startups</description>
    <item>
      <title>Ford hires Elon Musk as CEO</title>
      <pubDate>01 Apr 2019 07:30:00 GMT</pubDate>
      <description>In an unprecedented move, Ford hires Elon Musk.</description>
    </item>
  </channel>
</rss>
"#;

match rss_str.parse::<Feed>().unwrap() {
    Feed::RSS(rss_feed) => println!("RSS feed first entry: {:?}",
        rss_feed.items[0].title()),
    _ => {}
};
```

### Writing
Currently not supported.

## Todo
- Parse feeds into common format.
- Support writing feeds.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
