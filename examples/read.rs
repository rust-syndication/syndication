extern crate syndication;

use syndication::Feed;

fn main() {
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

    println!("Atom feed first entry: {:?}", atom_str.parse::<Feed>().unwrap().entries[0].title);

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

    println!("RSS feed first entry: {:?}", rss_str.parse::<Feed>().unwrap().entries[0].title);
}
