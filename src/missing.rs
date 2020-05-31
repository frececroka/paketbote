use std::collections::HashSet;

use diesel::PgConnection;
use fehler::throws;

use crate::alpm;
use crate::db::get_depends_by_repo;
use crate::db::get_provides_by_repo;
use crate::error::Error;
use crate::error::Result;
use crate::spec;
use crate::spec::Spec;

#[throws]
pub fn missing_dependencies(db: &PgConnection, repo_id: i32) -> Vec<Spec> {
    // Collect everything that is provided by this repository.
    let provides = get_provides_by_repo(db, repo_id)?.into_iter()
        .map(|(pp, pv)| -> Result<Spec> {
            let version = spec::Version::new_eq(pv);
            let spec: Spec = pp.parse()?;
            let spec = spec.fallback_version(version);
            Ok(spec)
        })
        .collect::<Result<HashSet<_>>>()?;

    // Collect everything that is required by this repository.
    let mut depends = get_depends_by_repo(db, repo_id)?.into_iter()
        .map(|pd| -> Result<Spec> { pd.parse() })
        .collect::<Result<HashSet<_>>>()?;

    let alpm = alpm::create("x86_64")?;

    // Filter out everything that is provided by the official repositories.
    depends.retain(|d| alpm.syncdbs().find_satisfier(&d.to_string()).is_none());

    // Filter out everything that is provided by this repository.
    depends.retain(|d| !provides.iter().any(|p| p.satisfies(d)));

    // Return what's left.
    depends.into_iter().collect()
}

#[cfg(test)]
mod test {
    use diesel::Connection;
    use diesel::PgConnection;
    use fehler::throws;

    use crate::error::Error;

    use super::missing_dependencies;

    #[test]
    #[throws]
    fn test() {
        let db = PgConnection::establish("postgres://alarm:alarm@192.168.0.154:5432/pacman").unwrap();
        let missing = missing_dependencies(&db, 200)?;
        println!("{:?}", missing);
    }
}
