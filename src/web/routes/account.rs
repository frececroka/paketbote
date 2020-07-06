use fehler::throws;
use rocket::http::Status;
use rocket_contrib::templates::Template;
use serde::Serialize;

use crate::web::ctx_base::BaseContext;
use crate::web::props::Props;
use crate::db::{get_account_by_name, get_repos_by_account};
use crate::db::models::{Account, Repo};

#[derive(Serialize)]
struct AccountContext {
    base: BaseContext,
    account: String,
    repos: Vec<Repo>
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
        .map_err(|_| Status::InternalServerError)?;
    let context = AccountContext::new(&props, account, repos);
    Template::render("account", context)
}
