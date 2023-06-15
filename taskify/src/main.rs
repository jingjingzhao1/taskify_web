extern crate tera;

use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use env_logger;
use serde::Deserialize;
use std::io;
mod db;
use crate::db::{Database, Task};
use tera::{Context, Tera};

struct AppState {
    database: Database,
}

#[derive(Debug, Deserialize)]
struct CreateTodoReq {
    title: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct UpdateTodoReq {
    title: String,
    description: String,
    progress: u8,
}

async fn create_todo(data: web::Data<AppState>, todo: web::Form<CreateTodoReq>) -> impl Responder {
    match data
        .database
        .insert(Task::new(&todo.title, &todo.description))
    {
        Ok(_) => HttpResponse::SeeOther()
            .append_header((actix_web::http::header::LOCATION, "/"))
            .finish(),
        Err(err) => {
            log::error!("Couldn't insert todo: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn create_to2(tmpl: web::Data<tera::Tera>) -> impl Responder {
    let ctx = Context::new();
    let s = tmpl.render("add.html", &ctx).unwrap();
    HttpResponse::Ok().body(s)
}

async fn list_all_todos(data: web::Data<AppState>) -> impl Responder {
    match data.database.get_all() {
        Ok(todos) => {
            if todos.is_empty() {
                HttpResponse::NotFound().body("Empty Task List")
            } else {
                HttpResponse::Ok().json(todos)
            }
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to retrieve tasks: {}", err))
        }
    }
}

async fn get_by_id(
    data: web::Data<AppState>,
    id: web::Path<u32>,
    tmpl: web::Data<tera::Tera>,
) -> impl Responder {
    match data.database.get_by_id(id.into_inner()) {
        Ok(todo) => {
            let mut ctx = Context::new();
            ctx.insert("todo", &todo);

            match tmpl.render("update.html", &ctx) {
                Ok(s) => HttpResponse::Ok().body(s),
                Err(err) => {
                    log::error!("Failed to render 'update.html': {}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(err) => {
            log::error!("couldn't get todo by id: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn update_todo(
    data: web::Data<AppState>,
    id: web::Path<u32>,
    todo: web::Form<UpdateTodoReq>,
) -> impl Responder {
    let todo = Task {
        id: Some(id.into_inner()),
        title: todo.title.clone(),
        description: todo.description.clone(),
        progress: todo.progress,
    };
    match data.database.update_todo(&todo) {
        Ok(_) => HttpResponse::SeeOther()
            .append_header((actix_web::http::header::LOCATION, "/"))
            .finish(),
        Err(err) => {
            log::error!("couldn't update todo: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn delete_todo(data: web::Data<AppState>, id: web::Path<u32>) -> impl Responder {
    match data.database.delete_by_id(id.into_inner()) {
        Ok(_) => HttpResponse::SeeOther()
            .append_header((actix_web::http::header::LOCATION, "/"))
            .finish(),
        Err(err) => {
            log::error!("couldn't delete todo: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn index(tmpl: web::Data<tera::Tera>, data: web::Data<AppState>) -> impl Responder {
    match data.database.get_all() {
        Ok(todos) => {
            let mut context = Context::new();
            context.insert("todos", &todos);
            match tmpl.render("index.html", &context) {
                Ok(rendered) => HttpResponse::Ok().body(rendered),
                Err(err) => {
                    log::error!("Failed to render 'index.html': {}", err);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to retrieve tasks: {}", err))
        }
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        let tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        App::new()
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(AppState {
                database: Database::new(String::from("data.db")),
            }))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(
                web::resource("/todo")
                    .route(web::post().to(create_todo))
                    .route(web::get().to(create_to2)),
            )
            .service(web::resource("/todos").route(web::get().to(list_all_todos)))
            .service(
                web::resource("/todos/{id}")
                    .route(web::get().to(get_by_id))
                    .route(web::post().to(update_todo)),
            )
            .service(web::resource("/todos/delete/{id}").route(web::post().to(delete_todo)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
