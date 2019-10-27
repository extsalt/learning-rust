mod Website;

fn main() {
    let website = Website::Website {
        url: "http://example.com".to_string(),
        hash: "23423sdfa2323".to_string(),
        status: "crawling".to_string()
    };

    println!("{}", website.getUrl());
}
