use rocket::{get, launch, routes, Build, Rocket};
use rocket_dyn_templates::{context, Template};

mod database;
mod models;
mod handlers;

use database::init_db;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Tutorial Rocket + PostgreSQL",
        message: "Bem-vindo ao nosso aplicativo web!"
    })
}

#[get("/login")]
fn login_page() -> Template {
    Template::render("login", context! {
        title: "Login - Rocket + PostgreSQL"
    })
}

#[get("/register")]
fn register_page() -> Template {
    Template::render("register", context! {
        title: "Registro - Rocket + PostgreSQL"
    })
}

#[get("/dashboard")]
fn dashboard_page() -> Template {
    Template::render("dashboard", context! {
        title: "Dashboard - Rocket + PostgreSQL"
    })
}

#[get("/library")]
fn books_page() -> Template {
    Template::render("books", context! {
        title: "Biblioteca Digital - Rocket + PostgreSQL"
    })
}

#[launch]
fn rocket() -> Rocket<Build> {
    // Inicializar banco de dados
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(init_db())
        .expect("Falha ao inicializar banco de dados");

    rocket::build()
        .mount("/", routes![
            index,
            login_page,
            register_page,
            dashboard_page,
            books_page,
            handlers::users::get_users,
            handlers::users::get_user,
            handlers::users::create_user,
            handlers::users::update_user,
            handlers::users::delete_user,
            handlers::auth::login,
            handlers::auth::register,
            handlers::auth::verify_token,
            handlers::books::get_books,
            handlers::books::get_book,
            handlers::books::create_book,
            handlers::books::update_book,
            handlers::books::delete_book,
            handlers::books::get_categories,
            handlers::books::create_category
        ])
        .attach(Template::fairing())
}
