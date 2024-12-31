use crate::validate::{
    SupportedGameVersions, ValidationError, ValidationResult,
};
use std::io::Cursor;
use zip::ZipArchive;
use crate::validate::FileInput;
pub struct PluginYmlValidator;

impl super::Validator for PluginYmlValidator {
    fn get_file_extensions(&self) -> &[&str] {
        &["zip", "jar"]
    }

    fn get_supported_loaders(&self) -> &[&str] {
        &["bukkit", "spigot", "paper", "purpur", "folia"]
    }

    fn get_supported_game_versions(&self) -> SupportedGameVersions {
        SupportedGameVersions::All
    }

    fn validate(
        &self,
        input: FileInput,
    ) -> Result<ValidationResult, ValidationError> {
        let archive = match input {
            FileInput::Archive(archive) => archive,
            _ => return Ok(ValidationResult::Warning("Wrong file type 1")),
        };
        return if archive.file_names().any(|name| name == "plugin.yml" || name == "paper-plugin.yml") {
            Ok(ValidationResult::Pass)
        } else {
            Ok(ValidationResult::Warning(
                "No plugin.yml or paper-plugin.yml present for plugin file.",
            ))
        };
    }
}

pub struct BungeeCordValidator;

impl super::Validator for BungeeCordValidator {
    fn get_file_extensions(&self) -> &[&str] {
        &["zip", "jar"]
    }

    fn get_supported_loaders(&self) -> &[&str] {
        &["bungeecord", "waterfall"]
    }

    fn get_supported_game_versions(&self) -> SupportedGameVersions {
        SupportedGameVersions::All
    }

    fn validate(
        &self,
        input: FileInput,
    ) -> Result<ValidationResult, ValidationError> {
        let archive = match input {
            FileInput::Archive(archive) => archive,
            _ => return Ok(ValidationResult::Warning("Wrong file type 4")),
        };
        return if archive.file_names().any(|name| name == "plugin.yml" || name == "bungee.yml") {
            Ok(ValidationResult::Pass)
        } else {
            Ok(ValidationResult::Warning(
                "No plugin.yml or bungee.yml present for plugin file.",
            ))
        };
    }
}

pub struct VelocityValidator;

impl super::Validator for VelocityValidator {
    fn get_file_extensions(&self) -> &[&str] {
        &["zip", "jar"]
    }

    fn get_supported_loaders(&self) -> &[&str] {
        &["velocity"]
    }

    fn get_supported_game_versions(&self) -> SupportedGameVersions {
        SupportedGameVersions::All
    }

    fn validate(
        &self,
        input: FileInput,
    ) -> Result<ValidationResult, ValidationError> {
        let mut archive = match input {
            FileInput::Archive(archive) => archive,
            _ => return Ok(ValidationResult::Warning("Wrong file type 2")),
        };

        return if archive.by_name("velocity-plugin.json").is_err() {
            Ok(ValidationResult::Warning(
                "No velocity-plugin.json present for plugin file.",
            ))
        } else {
            Ok(ValidationResult::Pass)
        };
    }
}

pub struct SpongeValidator;

impl super::Validator for SpongeValidator {
    fn get_file_extensions(&self) -> &[&str] {
        &["zip", "jar"]
    }

    fn get_supported_loaders(&self) -> &[&str] {
        &["sponge"]
    }

    fn get_supported_game_versions(&self) -> SupportedGameVersions {
        SupportedGameVersions::All
    }

    fn validate(
        &self,
        input: FileInput,
    ) -> Result<ValidationResult, ValidationError> {
        let archive = match input {
            FileInput::Archive(archive) => archive,
            _ => return Ok(ValidationResult::Warning("Wrong file type 3")),
        };

        return if !archive.file_names().any(|name| {
            name == "sponge_plugins.json"
                || name == "mcmod.info"
                || name == "META-INF/sponge_plugins.json"
        }) {
            Ok(ValidationResult::Warning(
                "No sponge_plugins.json or mcmod.info present for Sponge plugin.",
            ))
        } else {
            Ok(ValidationResult::Pass)
        };
    }
}

use obsidian_lib::{ObbyArchive};
/*
// Or from any Read + Seek source
let file = File::open("plugin.obby")?;
let mut archive = ObbyArchive::new(file)?;
let entries = archive.list_entries();
let data = archive.extract_entry("plugin.json")?;
*/

pub struct ObsidianValidator;

impl super::Validator for ObsidianValidator {
    fn get_file_extensions(&self) -> &[&str] {
        &["obby"]
    }

    fn get_supported_loaders(&self) -> &[&str] {
        &["obsidian"]
    }

    fn get_supported_game_versions(&self) -> SupportedGameVersions {
        SupportedGameVersions::All
    }

    fn validate(
        &self,
        input: FileInput,
    ) -> Result<ValidationResult, ValidationError> {
        return match input {
            FileInput::Obby(bytes) => {
                let archive = ObbyArchive::new(bytes.clone())?;
                let entries = archive.list_entries();
                if entries.iter().any(|x| x == "plugin.json") {
                    Ok(ValidationResult::Pass)
                } else {
                    Ok(ValidationResult::Warning("No plugin.json present for Obsidian plugin."))
                }
            }
            _ => Ok(ValidationResult::Warning(
                "Invalid file type: Not an Obby archive",
            )),
        };
    }
}