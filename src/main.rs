use actix_cors::Cors;
use actix_web::{get, options, post, App, HttpResponse, HttpServer, Responder};
use rust_chess::Board;
use serde_json::to_string;

#[derive(serde::Deserialize)]
struct MoveJson {
    start: rust_chess::Position,
    end: rust_chess::Position,
    board: Board,
}

#[get("/new-board")]
async fn new_board() -> impl Responder {
    let board = Board::default();
    HttpResponse::Ok().body(to_string(&board).unwrap())
}

#[post("/move-troop")]
async fn move_troop(body: String) -> actix_web::Result<impl Responder> {
    let mut move_json: MoveJson = serde_json::from_str(body.as_str())?;
    move_json.board.move_troop(move_json.start, move_json.end)?;

    Ok(HttpResponse::Ok().body(to_string(&move_json.board).unwrap()))
}

#[post("/display")]
async fn display(body: String) -> actix_web::Result<impl Responder> {
    let board: Board = serde_json::from_str(body.as_str())?;
    Ok(HttpResponse::Ok().body(format!("{}", board)))
}

#[options("/move-troop")]
async fn move_troop_options() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = std::env::var("PORT").unwrap().parse().unwrap();
    println!("Hosting API on port {}", port);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("devinchess*.vercel.app")
            .max_age(3600);

        App::new()
            .service(new_board)
            .service(move_troop)
            .service(display)
            .wrap(cors)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
