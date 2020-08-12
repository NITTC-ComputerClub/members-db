mod version_0000_initial;
mod version_0001_uuid;
mod version_0002_contacts_id;
mod version_0003_discord;
mod version_0004_office365;

use serde::Deserialize;
use serde_yaml::Error;

use crate::filesystem;

#[derive(Deserialize)]
struct Outline {
    version: u32,
}

pub fn migrate(path: &str) -> Result<(), Error> {
    let mut yaml = filesystem::open(path);

    loop {
        let outline: Result<Outline, Error> = serde_yaml::from_str(&yaml);
        let version = outline.map_or(0, |o| o.version);

        yaml = match version {
            0 => version_0001_uuid::up(&yaml)?,
            1 => version_0002_contacts_id::up(&yaml)?,
            2 => version_0003_discord::up(&yaml)?,
            3 => version_0004_office365::up(&yaml)?,
            _ => break,
        };

        println!("Migrated from {}", version)
    }

    filesystem::save(path, yaml);

    Ok(())
}
