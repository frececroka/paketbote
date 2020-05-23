use fehler::throws;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde::Serialize;

use crate::db::{create_token, delete_token_for_account, get_tokens_for_account};
use crate::db::models::{Account, NewToken, Token};
use crate::web::ctx_base::BaseContext;
use crate::web::Error;
use crate::web::props::Props;
use crate::web::routes::create_random_token;

#[derive(Serialize)]
struct AccessTokensContext {
    base: BaseContext,
    tokens: Vec<Token>
}

impl AccessTokensContext {
    fn new(props: &Props, tokens: Vec<Token>) -> AccessTokensContext {
        let base = BaseContext::new(&props.account);
        AccessTokensContext { base, tokens }
    }
}

#[throws]
#[get("/access-tokens")]
pub fn route_access_tokens(props: Props, account: Account) -> Template {
    let tokens = get_tokens_for_account(&*props.db, account.id)?;
    let context = AccessTokensContext::new(&props, tokens);
    Template::render("access-tokens", context)
}

#[derive(FromForm)]
pub struct CreateAccessToken {
    name: String
}

#[derive(Serialize)]
struct AccessTokenCreatedContext {
    base: BaseContext,
    token: NewToken
}

impl AccessTokenCreatedContext {
    fn new(props: &Props, token: NewToken) -> AccessTokenCreatedContext {
        let base = BaseContext::new(&props.account);
        AccessTokenCreatedContext { base, token }
    }
}

#[throws]
#[post("/access-tokens", data = "<token>")]
pub fn route_access_tokens_create(props: Props, account: Account, token: Form<CreateAccessToken>) -> Template {
    let token = NewToken {
        name: token.name.clone(),
        the_token: create_random_token(),
        account_id: account.id
    };
    create_token(&*props.db, &token)?;
    let context = AccessTokenCreatedContext::new(&props, token);
    Template::render("access-token-created", context)
}

#[throws]
#[delete("/access-tokens/<id>")]
pub fn route_access_tokens_delete(props: Props, account: Account, id: i32) -> Redirect {
    delete_token_for_account(&*props.db, account.id, id)?;
    Redirect::to("/access-tokens")
}
