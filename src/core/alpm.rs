use anyhow::Result;
use alpm::{Alpm, PackageReason};
use crate::core::model::Package;

pub struct AlpmContext {
    alpm: Alpm,
}

impl AlpmContext {
    pub fn new() -> Result<Self> {
        Ok(Self { alpm: Alpm::new("/", "/var/lib/pacman/")? })
    }

    /// True orphans: installed as dependencies, required_by list is empty and not explicitly installed.
    pub fn get_orphans(&self) -> Result<Vec<Package>> {
        let pkgs = self.alpm.localdb().pkgs();

        let mut orphans: Vec<Package> = pkgs
            .iter()
            .filter(|p| p.reason() == PackageReason::Depend && p.reason() != PackageReason::Explicit && 
            p.required_by().is_empty() && p.optional_for().is_empty())
            .filter_map(|p| Package::from_alpm_package(p).ok())
            .collect();

        orphans.sort_by(|a, b| b.size.cmp(&a.size));
        Ok(orphans)
    }
}
