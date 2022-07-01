#[allow(dead_code)]
pub struct MusicConfig {
    issue_id: u64,
    enable: bool,
}

#[allow(dead_code)]
pub struct ProgrssConfig {
    issue_id: u64,
    enable: bool,
}

#[allow(dead_code)]
pub struct AllConfig {
    username: String,
    repo_name: String,
    music_config: MusicConfig,
    progress_config: ProgrssConfig,
}