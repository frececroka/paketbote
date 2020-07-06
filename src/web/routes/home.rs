use rocket_contrib::templates::Template;
use serde::Serialize;

use crate::web::ctx_base::BaseContext;
use crate::web::props::Props;

#[derive(Serialize)]
struct Context {
    base: BaseContext
}

#[get("/")]
pub fn home(props: Props) -> Template {
    let context = Context { base: BaseContext::new(&props.account) };
    Template::render("home", context)
}
