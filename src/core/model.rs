use anyhow::Result;
use alpm::Package as AlpmPackage;
use chrono::{DateTime, TimeZone, Utc};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub size: u64,
    pub repo: String,
    pub install_date: DateTime<Utc>,
}

impl Package {
    pub fn from_alpm_package(pkg: &AlpmPackage) -> Result<Self> {
        let secs = pkg.install_date().unwrap_or(0);
        let install_date = Utc.timestamp_opt(secs, 0).single().unwrap_or_else(Utc::now);

        Ok(Self {
            name: pkg.name().to_string(),
            version: pkg.version().to_string(),
            size: pkg.isize() as u64,
            repo: pkg.db().map(|db| db.name()).unwrap_or("unknown").to_string(),
            install_date,
        })
    }

    pub fn size_human(&self) -> String {
        let b = self.size as f64;
        if b >= 1_073_741_824.0 {
            format!("{:.1} GiB", b / 1_073_741_824.0)
        } else if b >= 1_048_576.0 {
            format!("{:.1} MiB", b / 1_048_576.0)
        } else if b >= 1024.0 {
            format!("{:.1} KiB", b / 1024.0)
        } else {
            format!("{} B", b as u64)
        }
    }

    pub fn repo_color(&self) -> &'static str {
        match self.repo.as_str() {
            "core" => "\x1b[31m",
            "extra" => "\x1b[32m",
            "community" | "community-testing" => "\x1b[34m",
            "multilib" => "\x1b[35m",
            _ => "\x1b[33m",
        }
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}\x1b[0m {} ({}) - {} - {}",
            self.repo_color(),
            self.name,
            self.version,
            self.size_human(),
            self.repo,
            self.install_date.format("%Y-%m-%d")
        )
    }
}
