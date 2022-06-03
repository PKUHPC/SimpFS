use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RDMAConfig{
    pub addr: String
}
pub const CHUNK_SIZE: u64 = 8;//524288;
pub const DIRENT_BUF_SIZE: u64 = 8 * 1024 * 1024;
pub const CLIENT_CM_IDS: usize = 1;
