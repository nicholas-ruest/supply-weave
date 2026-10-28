use sha2::{Digest, Sha256};
use std::path::PathBuf;
use sw_errors::{Result, WeaveError};
use sw_types::Plan;
pub struct Repository {
    path: PathBuf,
}
impl Repository {
    pub fn open(dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&dir).map_err(|e| WeaveError::Persistence(e.to_string()))?;
        Ok(Self {
            path: dir.join("plans.rvf"),
        })
    }
    pub fn save(&self, p: &Plan) -> Result<()> {
        let bytes = serde_json::to_vec(p).map_err(|e| WeaveError::Persistence(e.to_string()))?;
        let h = Sha256::digest(p.id.as_bytes());
        let id = u64::from_le_bytes(h[0..8].try_into().expect("slice"));
        sw_rvf::Archive::append(&self.path, id, &bytes)
    }
    pub fn list(&self) -> Result<Vec<Plan>> {
        sw_rvf::Archive::read_all(&self.path)?
            .into_iter()
            .map(|(_, b)| {
                serde_json::from_slice(&b).map_err(|e| WeaveError::Persistence(e.to_string()))
            })
            .collect()
    }
    pub fn get(&self, id: &str) -> Result<Plan> {
        self.list()?
            .into_iter()
            .find(|p| p.id == id)
            .ok_or_else(|| WeaveError::NotFound(id.into()))
    }
    pub fn count(&self) -> Result<usize> {
        Ok(self.list()?.len())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_repository() {
        let d = tempfile::tempdir().unwrap();
        assert_eq!(
            Repository::open(d.path().into()).unwrap().count().unwrap(),
            0
        );
    }
}
