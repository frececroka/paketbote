use std::collections::HashSet;

use alpm::Alpm;
use alpm::SigLevel;
use diesel::PgConnection;
use fehler::throws;

use crate::db::get_all_packages_by_repo;
use crate::db::get_package_depends;
use crate::db::get_package_provides;
use crate::db::models::PackageDepends;
use crate::error::Error;
use crate::error::Result;
use crate::spec;
use crate::spec::Spec;

#[throws]
pub fn missing_dependencies(db: &PgConnection, repo_id: i32) -> Vec<Spec> {
    let packages = get_all_packages_by_repo(db, repo_id)?;

    // Collect everything that is provided by this repository.
    let provides = packages.iter()
        .map(|pkg| -> Result<Vec<Spec>> {
            let provides = get_package_provides(db, pkg.id)?
                .into_iter()
                .map(|p| -> Result<Spec> { p.provides.parse() })
                .collect::<Result<Vec<_>>>()?.into_iter()
                .map(|p| p.fallback_version(spec::Version::new_eq(pkg.version.to_owned())))
                .collect::<Vec<_>>();
            Ok(provides)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter().flatten()
        .collect::<HashSet<_>>();

    // Collect everything that is required by this repository.
    let mut dependencies = packages.iter()
        .map(|p| -> Result<Vec<PackageDepends>> { Ok(get_package_depends(db, p.id)?) })
        .collect::<Result<Vec<_>>>()?.into_iter().flatten()
        .map(|d| -> Result<Spec> {d.depends.parse() })
        .collect::<Result<HashSet<_>>>()?;

    let alpm = create_alpm()?;

    // Filter out everything that is provided by the official repositories.
    dependencies.retain(|d| alpm.syncdbs().find_satisfier(&d.to_string()).is_none());

    // Filter out everything that is provided by this repository.
    dependencies.retain(|d| !provides.iter().any(|p| p.satisfies(d)));

    // Return what's left.
    dependencies.into_iter().collect()
}

#[throws(alpm::Error)]
fn create_alpm() -> Alpm {
    let alpm = Alpm::new("/", "/var/lib/pacman/")?;
    alpm.register_syncdb("core", SigLevel::NONE)?;
    alpm.register_syncdb("community", SigLevel::NONE)?;
    alpm.register_syncdb("extra", SigLevel::NONE)?;
    alpm
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
