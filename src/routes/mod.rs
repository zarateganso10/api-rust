use crate::{
    models::users::{UserModel, CreateUserSchema},
    AppState,
};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/users")]
pub async fn users_list(data: web::Data<AppState>) -> impl Responder {
    let users: Vec<UserModel> = sqlx::query_as!(UserModel,"SELECT * FROM users")
        .fetch_all(&data.db)
        .await
        .unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": users.len(),
        "users": users
    });
    HttpResponse::Ok().json(json_response)
}



#[post("/users")]
async fn create_user(
    body: web::Json<CreateUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        UserModel,
        "INSERT into users (name, email, password) values ($1, $2, $3) returning *",
        body.name.to_string(),
        body.email.to_string(),
        body.password.to_string()
    ).fetch_one(&data.db)
    .await;

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "user": user
            })});
            return HttpResponse::Ok().json(user_response);
        }
        Err(e) => {
            if e.to_string().contains("duplicate key value violates unique constraint") {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail", "message": "Duplicate Key"}))
            }
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", e)}));
        }
    }

    // if let Err(err) = query_result {
    //     return HttpResponse::InternalServerError()
    //         .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
    // }

    // let query_result = sqlx::query_as!(NoteModel, r#"SELECT * FROM notes WHERE id = ?"#, user_id)
    //     .fetch_one(&data.db)
    //     .await;

    // match query_result {
    //     Ok(note) => {
    //         let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
    //             "note": filter_db_record(&note)
    //         })});

    //         return HttpResponse::Ok().json(note_response);
    //     }
    //     Err(e) => {
    //         return HttpResponse::InternalServerError()
    //             .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
    //     }
    // }
}


#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, MySQL, and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(users_list)
        .service(create_user);

    conf.service(scope);
}