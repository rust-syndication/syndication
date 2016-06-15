use rss;

pub struct Image {
    pub url: String,
    pub title: String,
    pub link: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl From<rss::Image> for Image {
    fn from(image: rss::Image) -> Image {
        Image {
            url: image.url,
            title: image.title,
            link: image.link,
            width: image.width,
            height: image.height,
        }
    }
}

impl From<Image> for rss::Image {
    fn from(image: Image) -> rss::Image {
        rss::Image {
            url: image.url,
            title: image.title,
            link: image.link,
            width: image.width,
            height: image.height,
        }
    }
}
