use std::path::Path;

use anyhow::{Context, Error};
use globset::{Glob, GlobSet, GlobSetBuilder};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone)]
pub struct BulkCopy {
    include: GlobSet,
    blacklist: GlobSet,
    max_depth: Option<usize>,
}

impl BulkCopy {
    pub fn new<I, S>(include_globs: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Ok(BulkCopy {
            include: compile_globs(include_globs)?,
            blacklist: GlobSet::empty(),
            max_depth: None,
        })
    }

    pub fn with_max_depth(self, depth: impl Into<Option<usize>>) -> Self {
        BulkCopy {
            max_depth: depth.into(),
            ..self
        }
    }

    pub fn with_blacklist<I, S>(self, globs: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Ok(BulkCopy {
            blacklist: compile_globs(globs)?,
            ..self
        })
    }

    pub fn copy<P, Q>(&self, from: P, to: Q) -> Result<(), Error>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        let from = from.as_ref();
        let to = to.as_ref();

        let mut wd = WalkDir::new(from);

        if let Some(max_depth) = self.max_depth {
            wd = wd.max_depth(max_depth);
        }

        for entry in wd.into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            if !self.include.is_match(path) || self.blacklist.is_match(path) {
                continue;
            }

            self.copy_entry(from, to, &entry)?;
        }

        Ok(())
    }

    fn copy_entry(
        &self,
        from: &Path,
        to: &Path,
        entry: &DirEntry,
    ) -> Result<(), Error> {
        let path = entry.path();
        let stripped = path.strip_prefix(from)?;
        let new_name = to.join(stripped);

        if let Some(parent) = new_name.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!(
                    "Unable to create the \"{}\" directory",
                    parent.display()
                )
            })?;
        }

        log::debug!(
            "Copying \"{}\" to \"{}\"",
            path.display(),
            new_name.display()
        );

        std::fs::copy(path, &new_name).with_context(|| {
            format!(
                "Unable to copy \"{}\" to \"{}\"",
                path.display(),
                new_name.display()
            )
        })?;

        Ok(())
    }
}

fn compile_globs<I, S>(globs: I) -> Result<GlobSet, Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut builder = GlobSetBuilder::new();

    for glob in globs {
        let glob = Glob::new(glob.as_ref())?;
        builder.add(glob);
    }

    builder.build().map_err(Error::from)
}
