use fehler::throws;
use rocket_contrib::templates::Template;
use serde::Serialize;

use crate::db::get_account;
use crate::db::get_packages_by_query;
use crate::db::get_repo;
use crate::db::models::Account;
use crate::db::models::Repo;
use crate::web::ctx_base::BaseContext;
use crate::web::Error;
use crate::web::models::augment_package;
use crate::web::models::Package;
use crate::web::props::Props;

#[derive(Serialize)]
struct SearchContext {
    base: BaseContext,
}

#[get("/search")]
pub fn route_search(props: Props) -> Template {
    let base = BaseContext::new(&props.account);
    let context = SearchContext { base };
    Template::render("search", context)
}

#[derive(Serialize)]
struct SearchResultsContext {
    base: BaseContext,
    query: String,
    results: Vec<PackageRepoAccount>
}

#[derive(Debug, Serialize)]
struct PackageRepoAccount {
    package: Package,
    repo: Repo,
    account: Account
}

#[throws]
#[get("/search?<query>")]
pub fn route_search_results(props: Props, query: String) -> Template {
    let results = get_packages_by_query(&props.db, &query)?
        .into_iter()
        .map(|package| {
            let package = augment_package(&props.db, package)?;
            let repo = get_repo(&props.db, package.repo_id)?;
            let account = get_account(&props.db, repo.owner_id)?;
            Ok(PackageRepoAccount { package, repo, account })
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let base = BaseContext::new(&props.account);
    let context = SearchResultsContext { base, query, results };
    Template::render("search-results", context)
}
