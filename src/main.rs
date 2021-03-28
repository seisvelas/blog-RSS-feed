use reqwest::header::USER_AGENT;
use reqwest::header::CONTENT_TYPE;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res: Vec<serde_json::Value> = client
        .get("https://api.github.com/users/seisvelas/gists?page=1&per_page=100")
        .header(USER_AGENT, "Xandre's RSS")
        .header(CONTENT_TYPE, "application/rss+xml")
        .send()
        .await?
        .json::<Vec<serde_json::Value>>()
        .await?;

    println!("<?xml version=\"1.0\" ?>");
    println!("  <rss version=\"2.0\">");
    println!("      <channel>");
    println!("          <title>Xandre's Blog</title>");
    println!("          <link>https://seisvelas.github.io/</link>");
    println!("          <description>Informal musings for curious spirits</description>");

    for gist in &res {
        let url = &gist["url"];
        let desc = &gist["description"];

        for (filename, _) in gist["files"].as_object().unwrap() {
            if filename.contains("blog_") {
                println!("          <item>");
                println!("              <title>{}</title>", desc);
                println!("              <link>{}</link>", url);
                println!("              <description>{}</description>", desc);
                println!("          </item>");
            }
        }
    }
    println!("  </channel>");
    println!("</rss>");

    Ok(())
}
