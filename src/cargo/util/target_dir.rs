//! Utilities for working with a build target directory, normally `target/`.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use crate::util::errors::CargoResult;

// Check for the existence of a valid `CACHEDIR.TAG` in the
// build target directory, without which it is not a valid
// target directory. See https://bford.info/cachedir/
pub fn check_target_tag(path: &Path) -> CargoResult<()> {
    let mut path = PathBuf::from(path);
    path.push("CACHEDIR.TAG");
    let tag_file = File::open(&path).map_err(|e| {
        anyhow::anyhow!("could not find CACHEDIR.TAG in target directory: {}", e)
    })?;
    let tag_file = BufReader::new(tag_file);
    let tag_line = tag_file
        .lines()
        .next()
        .ok_or(anyhow::anyhow!("invalid (empty) CACHEDIR.TAG in target directory"))?
        .map_err(|_| anyhow::anyhow!("could not read CACHEDIR.TAG in target directory"))?;
    const TAG: &str = "Signature: 8a477f597d28d172789f06886806bc55";
    if tag_line != TAG {
        return Err(anyhow::anyhow!("invalid CACHEDIR.TAG in target directory"))
    }
    Ok(())
}
