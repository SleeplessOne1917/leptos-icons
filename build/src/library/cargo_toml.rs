use anyhow::Result;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;
use tracing::{error, info, instrument};

use crate::feature::Feature;

const BASE_CARGO_TOML: &str = indoc::indoc!(
    r#"
    # ------------------------------------------------------------------------------------------
    # THIS FILE WAS GENERATED BY THE "BUILD" CRATE.
    # ------------------------------------------------------------------------------------------

    [package]
    name = "leptos-icons"
    version = "0.0.1"
    authors = ["Charles Edward Gagnon"]
    edition = "2021"
    description = "Icons library for the leptos web framework"
    readme = "./README.md"
    repository = "https://github.com/Carlosted/leptos-icons"
    license = "MIT"
    keywords = ["leptos", "icons"]
    categories = ["web-programming"]

    [dependencies]
    leptos = { version = "0.2", default-features = false }

    [features]
"#
);

#[derive(Debug)]
pub(crate) struct CargoToml {
    /// Path to the libraries Cargo.toml file.
    pub path: PathBuf,
}

impl CargoToml {
    #[instrument(level = "info")]
    pub async fn remove(&self) -> Result<()> {
        info!("Removing file.");
        tokio::fs::remove_file(&self.path).await.map_err(Into::into)
    }

    #[instrument(level = "info")]
    async fn create_file(&self) -> Result<tokio::fs::File> {
        info!("Creating file.");
        tokio::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&self.path)
            .await
            .map_err(|err| {
                error!(?err, "Could not create file.");
                err
            })
            .map_err(Into::into)
    }

    #[instrument(level = "info")]
    pub(crate) async fn init(&self) -> Result<()> {
        info!("Writing BASE_CARGO_TOML content.");
        self.create_file()
            .await?
            .write_all(BASE_CARGO_TOML.as_bytes())
            .await
            .map_err(Into::into)
    }

    #[instrument(level = "info", skip_all)]
    async fn append(&self) -> Result<tokio::io::BufWriter<tokio::fs::File>> {
        info!("Creating file.");
        Ok(tokio::io::BufWriter::new(
            tokio::fs::OpenOptions::new()
                .append(true)
                .open(&self.path)
                .await
                .map_err(|err| {
                    error!(?err, "Could not open file to append data.");
                    err
                })?,
        ))
    }

    #[instrument(level = "info", skip(features))]
    pub(crate) async fn append_features(&self, features: Vec<Feature>) -> Result<()> {
        info!(
            num_features = features.len(),
            "Writing features to Cargo.toml."
        );
        let mut cargo_file = self.append().await?;
        for feature in features.iter() {
            cargo_file
                .write_all(format!("{} = []\n", &feature.name).as_bytes())
                .await?;
        }
        cargo_file.flush().await.map_err(|err| {
            error!(?err, "Could not flush Cargo.toml file after writing.");
            err
        })?;

        Ok(())
    }
}
