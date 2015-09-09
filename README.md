# rust-syndication

Library for serializing Atom and RSS web feeds. Wraps around [rust-atom](https://github.com/vtduncan/rust-atom) [rust-rss](https://github.com/frewsxcv/rust-rss).

## Usage

### Reading
```rust
use syndication::Feed;

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

let Feed::Atom(atom_feed) = atom_str.parse::<Feed>().unwrap();

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

let Feed::RSS(rss_feed) = rss_str.parse::<Feed>().unwrap();
```

### Writing
Currently not supported.

## Todo
- Parse feeds into common format.
- Support writing feeds.
