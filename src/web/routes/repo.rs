use std::cmp::max;
use std::cmp::min;

use colored::*;
use diesel::PgConnection;
use fehler::throws;
use prettytable::Cell;
use prettytable::format::consts::FORMAT_CLEAN;
use prettytable::Row;
use prettytable::Table;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde::Serialize;

use crate::db::create_repo;
use crate::db::ExpectConflict;
use crate::db::get_all_packages_by_repo;
use crate::db::get_missing_deps;
use crate::db::get_packages_by_repo;
use crate::db::models::Account;
use crate::db::models::NewRepo;
use crate::db::models::Repo;
use crate::db::Paginated;
use crate::db::set_package_deleted;
use crate::jobs::create_repo_action;
use crate::jobs::RepoActionOp;
use crate::obsolete::determine_obsolete;
use crate::web::ctx_base::BaseContext;
use crate::web::db::Db;
use crate::web::Error;
use crate::web::Error::*;
use crate::web::models::augment_package;
use crate::web::models::Package;
use crate::web::props::Props;
use crate::web::routes::load_account;
use crate::web::routes::load_repo;
use crate::web::routes::validate_access;

#[throws]
#[get("/<account>/<repo>", format = "text/plain", rank = 5)]
pub fn route_repo_text(db: Db, account: String, repo: String) -> String {
    let (_, _, packages) = get_packages(&*db, &account, &repo, 0)?;

    let mut table = Table::new();
    table.set_format(*FORMAT_CLEAN);

    for package in packages.items {
        table.add_row(Row::new(vec![
            Cell::new(&package.name),
            Cell::new(&package.version.bright_blue().italic().to_string())]));
    }
    table.to_string()
}

#[derive(Serialize)]
struct RepoContext {
    base: BaseContext,
    account: Account,
    repo: Repo,
    packages: Paginated<Package>,
    missing_deps: Vec<String>,
    pages: Vec<usize>,
    can_edit: bool
}

impl RepoContext {
    fn new(
        props: &Props,
        account: Account,
        repo: Repo,
        packages: Paginated<Package>,
        missing_deps: Vec<String>
    ) -> RepoContext {
        let base = BaseContext::new(&props.account);
        let mut first_page = packages.current_page as isize - 3;
        let mut last_page = packages.current_page as isize + 4;
        if first_page < 0 {
            last_page -= first_page;
            first_page = 0;
        }
        if last_page > packages.total_pages as isize {
            first_page -= last_page - packages.total_pages as isize;
            last_page = packages.total_pages as isize;
        }
        first_page = max(first_page, 0);
        last_page = min(last_page, packages.total_pages as isize);
        let pages = (first_page as usize .. last_page as usize).collect();
        let can_edit = if let Some(active_account) = &props.account {
            active_account.name == account.name
        } else { false };
        RepoContext { base, account, repo, packages, missing_deps, pages, can_edit }
    }
}

#[throws]
#[get("/<account>/<repo>?<p>", format = "text/html", rank = 4)]
pub fn route_repo_html(props: Props, account: String, repo: String, p: Option<usize>) -> Template {
    let (account, repo, packages) = get_packages(&*props.db, &account, &repo, p.unwrap_or(0))?;
    let missing_deps = get_missing_deps(&*props.db, repo.id)?;
    let context = RepoContext::new(&props, account, repo, packages, missing_deps);
    Template::render("repo", context)
}

#[throws]
fn get_packages(db: &PgConnection, account: &str, repo: &str, page: usize) -> (Account, Repo, Paginated<Package>) {
    let account = load_account(db, account)?;
    let repo = load_repo(db, account.id, repo)?;
    let mut packages = get_packages_by_repo(db, repo.id, page)?;
    packages.items.sort_by_key(|p| p.name.clone());
    let packages = packages.try_map(|p| augment_package(db, p))?;
    (account, repo, packages)
}

#[derive(FromForm)]
pub struct CreateRepo {
    name: String
}

#[throws]
#[post("/<account>", data = "<data>")]
pub fn route_repo_create(
    db: Db,
    active_account: Account,
    account: String,
    data: Form<CreateRepo>,
) -> Redirect
{
    let account = validate_access(active_account, account)?;

    if data.name.len() == 0 {
        Err(BadRequest)?
    }

    let repo = NewRepo { name: data.name.clone(), owner_id: account.id };
    let repo = create_repo(&*db, &repo)
        .expect_conflict()?
        .ok_or(Conflict)?;

    Redirect::to(format!("/{}/{}", account.name, repo.name))
}

#[throws]
#[post("/<account>/<repo>/delete-obsolete", rank = 4)]
pub fn route_delete_obsolete(props: Props, account: String, repo: String) -> Redirect {
    let account = load_account(&*props.db, &account)?;
    let repo = load_repo(&*props.db, account.id, &repo)?;
    let packages = get_all_packages_by_repo(&*props.db, repo.id)?;
    let obsoletes = determine_obsolete(packages.iter().collect());

    for obsolete in obsoletes {
        set_package_deleted(&*props.db, obsolete.id, true)?;
        create_repo_action(&*props.db, obsolete.id, RepoActionOp::Remove)?;
    }

    Redirect::to(format!("/{}/{}", account.name, repo.name))
}
