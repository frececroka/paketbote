use fehler::throws;
use rocket_contrib::templates::Template;
use serde::Serialize;

use crate::db::{get_account, get_packages_by_query, get_repo};
use crate::db::models::{Account, Repo};
use crate::web::ctx_base::BaseContext;
use crate::web::Error;
use crate::web::props::Props;
use crate::web::routes::Package;

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
            let package: Package = package.into();
            let repo = get_repo(&props.db, package.repo_id)?;
            let account = get_account(&props.db, repo.owner_id)?;
            Ok(PackageRepoAccount { package, repo, account })
        })
        .collect::<Result<Vec<_>, diesel::result::Error>>()?;
    let base = BaseContext::new(&props.account);
    let context = SearchResultsContext { base, query, results };
    Template::render("search-results", context)
}
