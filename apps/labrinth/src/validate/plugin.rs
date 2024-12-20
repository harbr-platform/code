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

pub struct PumpkinValidator;

impl super::Validator for PumpkinValidator {
    fn get_file_extensions(&self) -> &[&str] {
        &["zip", "wasm"]
    }

    fn get_supported_loaders(&self) -> &[&str] {
        &["pumpkin"]
    }

    fn get_supported_game_versions(&self) -> SupportedGameVersions {
        SupportedGameVersions::All
    }

    // validator checks for the existence of a *.wasm by itself or a *.zip with a *.wasm inside
    // any name works, and make sure not to unzip the wasm file no just a pumpkin.wasm
    fn validate(
        &self,
        //archive: &mut ZipArchive<Cursor<bytes::Bytes>>
        input: FileInput,
    ) -> Result<ValidationResult, ValidationError> {
        // Match input and validate accordingly
        return match input {
            FileInput::Archive(archive) => {
                if archive_contains_wasm(&archive) {
                    Ok(ValidationResult::Pass)
                } else {
                    Ok(ValidationResult::Warning("No .wasm present for Pumpkin file."))
                }
            }
            FileInput::Wasm(bytes) => {
                if raw_contains_wasm(&bytes) {
                    Ok(ValidationResult::Pass)
                } else {
                    Ok(ValidationResult::Warning("No .wasm present for Pumpkin file."))
                }
            }
            FileInput::RawFile(bytes) => {
                if raw_contains_wasm(&bytes) {
                    Ok(ValidationResult::Pass)
                } else {
                    Ok(ValidationResult::Warning("No .wasm present for Pumpkin file."))
                }
            }
        }
    }
}

// needs improvement
fn archive_contains_wasm(archive: &ZipArchive<Cursor<bytes::Bytes>>) -> bool {
    archive.file_names().any(|x| x.ends_with(".wasm"))
}

// Helper function to check if raw bytes contain the WebAssembly magic header
fn raw_contains_wasm(bytes: &[u8]) -> bool {
    bytes.windows(4).any(|window| window == [0x00, 0x61, 0x73, 0x6d])
}
