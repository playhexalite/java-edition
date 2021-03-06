use std::path::PathBuf;

use anyhow::{Context, Result};
use tokio::fs;

use hexalite_common::dirs::get_hexalite_dir_path;

use crate::internal::{handle_dir_error, use_handling_auto};

use super::use_handling;

lazy_static::lazy_static! {
    static ref FILES: Vec<&'static str> = vec!["run", ".env"];
}

pub async fn init(src_path: PathBuf) -> Result<()> {
    let hexalite = get_hexalite_dir_path();
    if let Err(err) = fs::create_dir_all(&hexalite).await {
        handle_dir_error(&hexalite, &hexalite, err);
    }

    let src_path = fs::canonicalize(src_path)
        .await
        .context("Failed to get the canonical source path.")?;

    use_handling(&src_path, &hexalite.join("dev"), |src, dest| {
        symlink::symlink_dir(src, dest)
    });
    use_handling(&src_path.join("resource-pack/out"), &hexalite.join("resource-pack"), |src, dest| {
        symlink::symlink_dir(src, dest)
    });

    for file in &*FILES {
        use_handling_auto(&src_path, file, |src, dest| {
            symlink::symlink_auto(src, dest)
        });
    }
    Ok(())
}
