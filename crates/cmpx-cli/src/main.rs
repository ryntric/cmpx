use cmpx_core::differ::*;
use cmpx_core::loader;
use cmpx_core::parser;
use cmpx_core::parser::error::ParserError;
use cmpx_core::source::http::{HttpMethod, HttpSource};
use cmpx_core::source::*;
use futures::future::try_join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let definitions = vec![
        SourceDefinition::http(
            "Fake HTTP Data 1",
            SourceFormat::Json,
            HttpSource::builder("https://jsonplaceholder.typicode.com/todos/1")
                .method(HttpMethod::Get)
                .build(),
        ),
        SourceDefinition::http(
            "Fake HTTP Data 2",
            SourceFormat::Json,
            HttpSource::builder("https://jsonplaceholder.typicode.com/todos/1")
                .method(HttpMethod::Get)
                .build(),
        ),
        SourceDefinition::http(
            "Fake HTTP Data 3",
            SourceFormat::Json,
            HttpSource::builder("https://jsonplaceholder.typicode.com/todos/1")
                .method(HttpMethod::Get)
                .build(),
        ),
        SourceDefinition::http(
            "Fake HTTP Data 4",
            SourceFormat::Json,
            HttpSource::builder("https://jsonplaceholder.typicode.com/todos/1")
                .method(HttpMethod::Get)
                .build(),
        ),
        SourceDefinition::http(
            "Fake HTTP Data 5",
            SourceFormat::Json,
            HttpSource::builder("https://jsonplaceholder.typicode.com/todos/1")
                .method(HttpMethod::Get)
                .build(),
        ),
        SourceDefinition::http(
            "Fake HTTP Data 6",
            SourceFormat::Json,
            HttpSource::builder("https://jsonplaceholder.typicode.com/todos/1")
                .method(HttpMethod::Get)
                .build(),
        ),
    ];

    let loaded = try_join_all(definitions.iter().map(loader::load)).await?;

    let sources = definitions
        .iter()
        .zip(loaded)
        .map(|(definition, source_data)| {
            Ok(DiffSource::new(
                definition.name(),
                parser::parse(source_data)?,
            ))
        })
        .collect::<Result<Vec<_>, ParserError>>()?;

    Differ::diff(&sources);

    Ok(())
}
