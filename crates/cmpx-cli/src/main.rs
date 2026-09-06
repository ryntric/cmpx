use cmpx_core::differ::*;
use cmpx_core::loader::SourceLoader;
use cmpx_core::parser::DocumentParser;
use cmpx_core::source::http::{HttpConfig, HttpMethod};
use cmpx_core::source::*;
use futures::future::try_join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sources = vec![
        Source::new(
            "Fake HTTP Data 1",
            SourceFormat::Json,
            SourceConfig::Http(
                HttpConfig::builder("https://dummyjson.com/comments?limit=10&skip=10&select=body,postId")
                    .method(HttpMethod::Get)
                    .build(),
            ),
        ),
        Source::new(
            "Fake HTTP Data 2",
            SourceFormat::Json,
            SourceConfig::Http(
                HttpConfig::builder("https://dummyjson.com/comments?limit=10&skip=10&select=body,postId")
                    .method(HttpMethod::Get)
                    .build(),
            ),
        ),
        Source::new(
            "Fake HTTP Data 3",
            SourceFormat::Json,
            SourceConfig::Http(
                HttpConfig::builder("https://dummyjson.com/comments?limit=10&skip=10&select=body,postId")
                    .method(HttpMethod::Get)
                    .build(),
            ),
        ),
        Source::new(
            "Fake HTTP Data 4",
            SourceFormat::Json,
            SourceConfig::Http(
                HttpConfig::builder("https://dummyjson.com/comments?limit=10&skip=10&select=body,postId")
                    .method(HttpMethod::Get)
                    .build(),
            ),
        ),
        Source::new(
            "Fake HTTP Data 5",
            SourceFormat::Json,
            SourceConfig::Http(
                HttpConfig::builder("https://dummyjson.com/comments?limit=10&skip=10&select=body,postId")
                    .method(HttpMethod::Get)
                    .build(),
            ),
        ),
        Source::new(
            "Fake HTTP Data 6",
            SourceFormat::Json,
            SourceConfig::Http(
                HttpConfig::builder("https://dummyjson.com/comments?limit=10&skip=10&select=body,postId")
                    .method(HttpMethod::Get)
                    .build(),
            ),
        ),
    ];

    let loader = SourceLoader::new();
    if let Ok(sources) = try_join_all(sources.iter().map(|source| loader.load(source))).await {
        Differ::diff(
            &sources
                .iter()
                .map(|source| {
                    return DiffSource::new(
                        source.id(),
                        DocumentParser::parse(source).unwrap(),
                    );
                })
                .collect::<Vec<_>>(),
        )
    }
    Ok(())
}
