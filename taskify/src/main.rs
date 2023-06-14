use std::io;
use env_logger;
use actix_web::{middleware, web, App,HttpResponse, HttpServer, Responder};
use serde::{Deserialize};
mod db;
use crate::{db::{ Database, Todo}};


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

async fn create_todo(
    
    data: web::Data<AppState>,
    todo: web::Json<CreateTodoReq>,
) -> impl Responder {
    match data.database.insert(Todo::new(&todo.title,&todo.description)) {
        Ok(todo) => HttpResponse::Created().json(todo),
        Err(err) => {
            log::error!("couldn't insert todo: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
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
        Err(err) => HttpResponse::InternalServerError().body(format!("Failed to retrieve tasks: {}", err)),
    }
}

async fn get_by_id(
    data: web::Data<AppState>,
    id: web::Path<u32>,
) -> impl Responder {
    match data.database.get_by_id(id.into_inner()) {
        Ok(todo) => {
            log::info!("Successfully retrieved todo by ID");
            HttpResponse::Ok().json(todo)   
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
    todo: web::Json<UpdateTodoReq>,
) -> impl Responder {
    let todo = Todo {
        id: Some(id.into_inner()),
        title: todo.title.clone(),
        description: todo.description.clone(),
        progress: todo.progress,
    };
    match data.database.update_todo(&todo) {
        Ok(todo) => {
            log::info!("Successfully update todo by ID");
            HttpResponse::Ok().json(todo)   
        }
        Err(err) => {
            log::error!("couldn't update todo: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn delete_todo(
    data: web::Data<AppState>,
    id: web::Path<u32>,
) -> impl Responder {
    match data.database.delete_by_id(id.into_inner()) {
        Ok(_) => {
            log::info!("Successfully Delete todo");
            HttpResponse::Ok().finish()   
        }
        Err(err) => {
            log::error!("couldn't delete todo: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                database: Database::new(String::from("data.db")),
            }))
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(|| HttpResponse::Ok())))
            .service(web::resource("/todo").route(web::post().to(create_todo)))
            .service(web::resource("/todos").route(web::get().to(list_all_todos)))
            .service(web::resource("/todos/{id}").route(web::get().to(get_by_id)))
            .service(web::resource("/todos/update/{id}").route(web::patch().to(update_todo)))
            .service(web::resource("/todos/delete/{id}").route(web::delete().to(delete_todo)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}