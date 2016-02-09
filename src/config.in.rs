#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    file_path: String,
    auth_file: String,
}
