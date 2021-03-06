use anyhow::{Context, Result};

use hexalite_common::dirs::get_hexalite_dir_path;

use super::*;

pub async fn webserver(sh: &Shell) -> Result<()> {
    let path = compiled_file!("webserver");
    let path = path
        .to_str()
        .context("Failed to retrieve the compiled webserver path.")?;

    xshell::cmd!(sh, "{path}")
        .run()
        .context("Failed to run the webserver.")
}

pub async fn resource_pack(sh: &Shell) -> Result<()> {
    let path = compiled_file!("resource-pack");
    let path = path
        .to_str()
        .context("Failed to retrieve the compiled resource pack path.")?;
    xshell::cmd!(sh, "{path}")
        .run()
        .context("Failed to run the resource pack generator.")
}

pub async fn minecraft(sh: &Shell) -> Result<()> {
    let hexalite = get_hexalite_dir_path();
    let run = hexalite.join("run");
    sh.change_dir(&run);
    xshell::cmd!(sh, "java -Xmx2G -jar purpur.jar")
        .run()
        .context(" Failed to run the Minecraft server.")
}
