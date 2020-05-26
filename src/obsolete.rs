use alpm::vercmp;
use itertools::Itertools;

use crate::db::models::Package;

pub fn determine_obsolete(mut packages: Vec<&Package>) -> Vec<&Package> {
    packages.sort_by_key(|p| p.name.clone());
    packages.into_iter()
        .group_by(|p| p.name.clone()).into_iter()
        .map(|(_, g)| determine_obsolete_single(g.collect()))
        .collect::<Vec<Vec<&Package>>>()
        .into_iter().flatten().collect()
}

fn determine_obsolete_single(mut packages: Vec<&Package>) -> Vec<&Package> {
    sort_by_version(&mut packages);
    packages.into_iter()
        .rev()
        .skip_while(|p| !p.active)
        .skip(1)
        .collect()
}

fn sort_by_version(packages: &mut [&Package]) {
    packages.sort_by(|p, q| vercmp(&p.version, &q.version));
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;
    use itertools::Itertools;

    use crate::db::models::{Compression, Package};
    use crate::obsolete::determine_obsolete_single;

    #[test]
    fn test_determine_obsolete_single_no_packages() {
        let obsolete = determine_obsolete_single(vec![]);
        assert!(obsolete.is_empty());
    }

    #[test]
    fn test_determine_obsolete_single_only_newer_packages() {
        let packages = vec![
            make_package(0, "2.3-5", false),
            make_package(1, "2.4-1", false),
            make_package(2, "2.3-4", true),
            make_package(3, "2.4-2", false),
        ];
        let obsolete = determine_obsolete_single(packages.iter().collect());
        assert!(obsolete.is_empty());
    }

    #[test]
    fn test_determine_obsolete_single_only_older_packages() {
        let packages = vec![
            make_package(0, "1.3-1", false),
            make_package(1, "2.3-3", false),
            make_package(2, "2.3-4", true),
            make_package(3, "2.2-2", false),
        ];
        let obsolete = determine_obsolete_single(packages.iter().collect());
        assert_eq!(get_ids(&obsolete), vec![0, 1, 3]);
    }

    #[test]
    fn test_determine_obsolete_single_some_newer_some_older_packages() {
        let packages = vec![
            make_package(0, "1.3-1", false),
            make_package(1, "2.3-3", false),
            make_package(2, "2.3-5", false),
            make_package(3, "2.3-4", true),
            make_package(4, "2.4-1", false),
            make_package(5, "2.2-2", false),
            make_package(6, "2.4-2", false),
        ];
        let obsolete = determine_obsolete_single(packages.iter().collect());
        assert_eq!(get_ids(&obsolete), vec![0, 1, 5]);
    }

    fn make_package(id: i32, version: &str, active: bool) -> Package {
        let name = String::new();
        let version = version.to_owned();
        let arch = String::new();
        let size = 0;
        let archive = String::new();
        let signature = String::new();
        let compression = Compression::Zstd;
        let created = NaiveDate::from_ymd(2016, 7, 8)
            .and_hms(9, 10, 11);
        let deleted = false;
        let repo_id = 0;
        Package { id, name, version, arch, size, archive, signature, compression, created, active, deleted, repo_id }
    }

    fn get_ids(packages: &[&Package]) -> Vec<i32> {
        packages.iter().map(|p| p.id).sorted().collect()
    }
}
