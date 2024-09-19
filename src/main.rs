use reqwest;
use serde::{Deserialize, Serialize};

struct Confluence {
    base_url: String,
    rest_api_url: String,
}

impl Confluence {
    fn new(base_url: &str, rest_api_url: &str) -> Self {
        Self {
            base_url: base_url.to_owned(),
            rest_api_url: rest_api_url.to_owned(),
        }
    }

    fn api_url(&self) -> String {
        format!("{}/{}", self.base_url, self.rest_api_url)
    }

    /// Fetches content from the provided URL.
    async fn fetch_contents(&self, url: &str) -> ContentResponse {
        api_call::<ContentResponse>(url).await
    }

    /// Fetches attachments for a specific content ID and media type.
    async fn fetch_attachments(
        &self,
        content_id: &str,
        media_type: &str,
        limit: u32,
    ) -> Vec<Attachment> {
        let mut attachment_url = format!(
            "{}/content/{}/child/attachment?limit={}&mediaType={}",
            self.api_url(),
            content_id,
            limit,
            media_type
        );
        let mut all_attachments = Vec::new();

        loop {
            let attachment_response = api_call::<AttachmentResponse>(&attachment_url)
                .await;
            all_attachments.extend(attachment_response.results);

            if let Some(next_attachment_url) = attachment_response._links.next.as_deref() {
                // Update the attachment_url with the next_attachment_url provided by the API.
                attachment_url = format!("{}{}", self.base_url, next_attachment_url);
            } else {
                // No more pages to fetch.
                break;
            }
        }

        all_attachments
    }

    /// Processes all contents and their respective attachments in a content-by-content manner.
    async fn process_all_contents(
        &self,
        media_types: &[&str],
        limit: u32,
    ) {
        let mut content_url = format!(
            "{}/content?type=page&expand=space&limit={}&status=current&start=0",
            self.api_url(),
            limit
        );

        loop {
            let content_response = self.fetch_contents(&content_url).await;

            for content in content_response.results {
                // println!("Processing content: {}", content.title);

                for &media_type in media_types {
                    //println!("Fetching attachments for media type: {}", media_type);

                    let attachments = self
                        .fetch_attachments(&content.id, media_type, limit)
                        .await;

                    for attachment in attachments {
                        println!(
                            "\"{}/{}/{}\" {}{} {}/content/{}/child/attachment/{}/data",
                            content.space.key,
                            content.title,
                            attachment.title,
                            self.base_url,
                            attachment._links.download,
                            self.api_url(),
                            content.id,
                            attachment.id,
                        );
                    }
                }

                //println!("Finished processing content: {}\n", content.title);
            }

            if let Some(next_url) = content_response._links.next.as_deref() {
                // Update the content_url with the next_url provided by the API.
                content_url = format!("{}{}", self.base_url, next_url);
                println!("Fetching next page of contents...\n");
            } else {
                // No more pages to fetch.
                println!("All contents processed.");
                break;
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct WebuiLink {
    webui: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Space {
    id: u32,
    key: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    id: String,
    space: Space,
    title: String,
    _links: WebuiLink,
}

#[derive(Serialize, Deserialize, Debug)]
struct Attachment {
    id: String,
    title: String,
    _links: AttachmentLink,
}

#[derive(Serialize, Deserialize, Debug)]
struct AttachmentLink {
    download: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Link {
    next: Option<String>,
    base: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentResponse {
    results: Vec<Content>,
    _links: Link,
}

#[derive(Serialize, Deserialize, Debug)]
struct AttachmentResponse {
    results: Vec<Attachment>,
    _links: Link,
}

/// Performs an API call and deserializes the response into the specified type.
/// Returns the deserialized type or panics on error.
async fn api_call<T: serde::de::DeserializeOwned>(url: &str) -> T {
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            let api_response: T = response.json().await.unwrap();
            api_response
        }
        other => {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            panic!("Uh oh! Something unexpected happened: {}. Response: {}", other, error_text);
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize Confluence instance
    let wiki = Confluence::new("https://wiki.onap.org", "rest/api");

    // Define media types to fetch
    let media_types = ["video/mp4"];

    // Process contents and attachments incrementally
    wiki.process_all_contents(&media_types, 50).await;
}
