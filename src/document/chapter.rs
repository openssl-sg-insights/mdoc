use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Chapter {
    pub content: String,
    pub path: Option<PathBuf>,
}

impl Chapter {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            path: None,
        }
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let file = File::open(path).with_context(|| format!("Could not open file {:?}.", path))?;

        let mut content = String::new();
        BufReader::new(&file)
            .read_to_string(&mut content)
            .with_context(|| format!("Could not read {:?} to string.", file))?;

        Ok(Self {
            path: Some(path.to_owned()),
            content,
        })
    }
}
