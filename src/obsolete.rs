use std::cmp::Ordering;
use std::panic;
use std::process::Command;

use fehler::throws;
use itertools::Itertools;

use crate::db::models::Package;
use crate::error::Error;

#[throws]
pub fn determine_obsolete(mut packages: Vec<&Package>) -> Vec<&Package> {
    packages.sort_by_key(|p| p.name.clone());
    packages.into_iter()
        .group_by(|p| p.name.clone()).into_iter()
        .map(|(_, g)| determine_obsolete_single(g.collect()))
        .collect::<Result<Vec<Vec<&Package>>, _>>()?
        .into_iter().flatten().collect()
}

#[throws]
fn determine_obsolete_single(packages: Vec<&Package>) -> Vec<&Package> {
    let packages = sort_by_version(packages)?;
    packages.into_iter()
        .skip_while(|p| !p.active)
        .skip(1)
        .collect()
}

#[throws]
fn sort_by_version(mut packages: Vec<&Package>) -> Vec<&Package> {
    panic::catch_unwind(move || {
        packages.sort_by(|p, q|
            package_vercmp(&p.version, &q.version)
                .unwrap().reverse());
        packages
    }).map_err(|_| "Sorting packages by version failed.")?
}

#[throws]
fn package_vercmp(v: &str, w: &str) -> Ordering {
    let output = Command::new("vercmp")
        .arg(v).arg(w)
        .output()?;
    if !output.status.success() {
        Err(format!("Invocation of vercmp failed with exit code {:?}.",
            output.status.code()))?
    }
    let result: i32 = String::from_utf8(output.stdout)?.trim().parse()?;
    if result < 0 {
        Ordering::Less
    } else if result == 0 {
        Ordering::Equal
    } else if result > 0 {
        Ordering::Greater
    } else {
        unreachable!()
    }
}

