use crate::{
    config::Config,
    utils::{find_root, SourceType},
    Chapter, Document, CONFIG_FILE,
};

use anyhow::{anyhow, bail, Context, Result};
use ignore::{types::TypesBuilder, WalkBuilder};
use std::path::Path;

pub struct DocumentBuilder {
    source: SourceType,
    config: Option<Config>,
}

impl DocumentBuilder {
    pub fn new() -> Self {
        DocumentBuilder {
            source: SourceType::None,
            config: None,
        }
    }

    #[must_use]
    pub fn source(mut self, path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();

        if path.is_dir() {
            self.source = SourceType::Dir(path.to_owned());
        } else if path.is_file() {
            self.source = SourceType::File(path.to_owned());
        }

        self
    }

    #[must_use]
    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> Result<Document> {
        let (config, chapters, root) = match self.source {
            SourceType::File(ref path) => (
                self.config.unwrap_or_default(),
                vec![Chapter::load(path)?],
                None,
            ),

            SourceType::Dir(ref path) => {
                let config = self
                    .config
                    .unwrap_or(Config::from_file(path.join(CONFIG_FILE))?);
                let chapters = load_chapters(path, &config)?;

                (config, chapters, Some(path.to_owned()))
            }

            SourceType::None => {
                let root = find_root()?;
                let config = self
                    .config
                    .unwrap_or(Config::from_file(root.join(CONFIG_FILE))?);
                let chapters = load_chapters(&root, &config)?;

                (config, chapters, Some(root))
            }
        };

        if chapters.is_empty() {
            bail!("No chapters found.");
        }

        debug!(
            "Building document with {} chapters:\n{:#?}",
            chapters.len(),
            chapters
        );
        debug!("Using config: {:#?}", config);

        Ok(Document {
            chapters,
            config,
            root,
        })
    }
}

impl Default for DocumentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn load_chapters<P: AsRef<Path>>(path: P, config: &Config) -> Result<Vec<Chapter>> {
    let walk = |path: &Path| {
        let md_types = TypesBuilder::new()
            .add_defaults()
            .select("markdown")
            .build()
            .unwrap();

        WalkBuilder::new(path)
            .types(md_types)
            .sort_by_file_path(Path::cmp)
            .build()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .map(|entry| Chapter::load(entry.path()))
            .filter_map(|ch| {
                ch.map_err(|err| warn!("{} Skipping this chapter...", err))
                    .ok()
            })
            .collect()
    };

    match &config.src {
        Some(toml::Value::Array(paths)) => Ok(paths
            .iter()
            .filter_map(|val| val.as_str())
            .map(|p| Chapter::load(Path::new(p)))
            .filter_map(|ch| {
                ch.map_err(|err| warn!("{} Skipping this chapter...", err))
                    .ok()
            })
            .collect()),

        Some(toml::Value::String(path)) => {
            let path = Path::new(&path);
            if !path.exists() {
                bail!("You specified a path {:?}, which does not exist.", path)
            } else if !path.is_dir() {
                Err(anyhow!(
                    "The src field must be a directory or a list of files."
                ))
                .with_context(|| format!("{:?} is not a directory.", path))
            } else {
                Ok(walk(path))
            }
        }

        _ => Ok(walk(path.as_ref())),
    }
}
