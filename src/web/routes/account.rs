use fehler::throws;
use rocket::http::Status;
use rocket_contrib::templates::Template;
use serde::Serialize;

use crate::db::{get_account_by_name, get_packages_by_repo, get_repos_by_account, models};
use crate::db::models::{Account, Package};
use crate::web::ctx_base::BaseContext;
use crate::web::props::Props;

#[derive(Serialize)]
struct AccountContext {
    base: BaseContext,
    account: String,
    repos: Vec<Repo>
}

#[derive(Serialize)]
struct Repo {
    name: String,
    packages: usize,
    byte_size: usize
}

impl Repo {
    fn new(repo: models::Repo, packages: Vec<Package>) -> Repo {
        let name = repo.name;
        let byte_size = packages.iter().map(|p| p.size as usize).sum();
        let packages = packages.len();
        Repo { name, packages, byte_size }
    }
}

impl AccountContext {
    fn new(props: &Props, account: Account, repos: Vec<Repo>) -> AccountContext {
        let base = BaseContext::new(&props.account);
        let account = account.name;
        AccountContext { base, account, repos }
    }
}

#[get("/<account>")]
#[throws(Status)]
pub fn route_account(props: Props, account: String) -> Template {
    let account = get_account_by_name(&*props.db, &account)
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;
    let repos = get_repos_by_account(&*props.db, account.id)
        .map_err(|_| Status::InternalServerError)?
        .into_iter()
        .map(|repo| {
            let packages = get_packages_by_repo(&*props.db, repo.id)
                .map_err(|_| Status::InternalServerError)?;
            Ok(Repo::new(repo, packages))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let context = AccountContext::new(&props, account, repos);
    Template::render("account", context)
}
