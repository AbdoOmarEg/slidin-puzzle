use askama::Template;
use axum::extract::Query;
use axum::routing::get;
use axum::Router;
use axum::{extract::State, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

// add tailwindcss
// playable board

mod slayin;

#[tokio::main]
async fn main() {
    // Initialize the database and run migrations
    let pool = initialize_database().await;

    // Set up routes and run the application
    let app = Router::new()
        .route("/", get(handler))
        .route("/first_iteration", get(path_index_whole_board))
        .route("/rest_iterations", get(path_index_iterations))
        .route("/loading", get(loading))
        .with_state(pool);
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "loading.html")]
struct LoadingTemplate {
    title: String,
}

async fn loading() -> impl IntoResponse {
    LoadingTemplate {
        title: String::from("loading..."),
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
    title: String,
}

async fn handler() -> impl IntoResponse {
    let template = HelloTemplate {
        title: String::from("index page"),
    };
    template
}

#[derive(sqlx::FromRow, Serialize, Debug)]
struct PuzzleIteration {
    iteration_index: i32,
    path_json: String,
}

#[derive(Template)]
#[template(path = "iteration2.html")]
struct StepTemplate {
    title: String,
    path: Vec<String>,
    step: i32,
    is_last_step: bool,
}

#[derive(Deserialize, Debug)]
struct Input {
    index: i32,
}

// add query for the index

async fn path_index_iterations(
    State(pool): State<SqlitePool>,
    Query(query): Query<Input>,
) -> impl IntoResponse {
    let index = query.index;
    // println!("rest_iterations initiating with index={}", index);

    let iterations =
        sqlx::query_as::<_, PuzzleIteration>("SELECT * FROM full_path WHERE iteration_index = ?")
            .bind(index)
            .fetch_one(&pool)
            .await;
    // println!("line 84 Query result: {:?}", iterations);

    if let Ok(iteration) = iterations {
        // let path: String = serde_json::from_str(&iteration.path_json).unwrap();
        let path: String = iteration.path_json;

        // println!("path from json ={:?} and needed to be changed to 2x2", path);
        let path: Vec<String> = path
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        // println!("path path_index_z={:?}", path);
        if path.len() == 4 {
            let template = StepTemplate {
                title: String::from("path page"),
                path,
                step: iteration.iteration_index,
                is_last_step: true,
            };
            return template.render().unwrap().into_response();
        }

        let template = StepTemplate {
            title: String::from("path page"),
            path,
            step: iteration.iteration_index + 1,
            is_last_step: false,
        };

        return template.render().unwrap().into_response();
    } else {
        // unreachable else statement
        let iterations = sqlx::query_as::<_, PuzzleIteration>(
            "SELECT * FROM full_path WHERE iteration_index = ?",
        )
        .bind(index - 1)
        .fetch_one(&pool)
        .await
        .unwrap();
        // println!("last board, {:?}", iterations);

        let path: String = iterations.path_json;

        // println!("path from json ={:?} and needed to be changed to 2x2", path);
        let path: Vec<String> = path
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        // println!("path path_index_z={:?}", path);

        let template = StepTemplate {
            title: String::from("path page"),
            path,
            step: iterations.iteration_index,
            is_last_step: true,
        };

        return template.render().unwrap().into_response();
        // axum::response::Html(String::from("<p>Iteration not found</p>")).into_response()
    }
}

// start db

use sqlx::{migrate::MigrateDatabase, Row, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

pub async fn initialize_database() -> SqlitePool {
    // accepting args
    // let args: Vec<String> = env::args().collect();
    //
    // let email = if args.len() > 1 { Some(&args[1]) } else { None };

    // db start
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        // println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let pool = SqlitePool::connect(DB_URL).await.unwrap();

    //migration script
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");

    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&pool)
        .await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    // println!("migration: {:?}", migration_results);

    //start
    let result = sqlx::query(
        "SELECT name
         FROM sqlite_schema
         WHERE type ='table'
         AND name NOT LIKE 'sqlite_%';",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    for (idx, row) in result.iter().enumerate() {
        // println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }

    // // args inserting
    // if let Some(email) = email {
    //     let db = SqlitePool::connect(DB_URL).await.unwrap();
    //
    //     let result = sqlx::query("INSERT INTO fishy_website_com (email) VALUES (?)")
    //         .bind(email)
    //         .execute(&db)
    //         .await;
    //
    //     match result {
    //         Ok(_) => {
    //             println!("Email inserted into the database: {}", email);
    //         }
    //         Err(err) => {
    //             println!(
    //                 "Email already exists in the database: {}, with error: {}",
    //                 email, err
    //             );
    //         }
    //     }
    // }

    // let puzzles =
    //     sqlx::query_as::<_, PathDB>("SELECT id, path_json, full_path FROM puzzle_iterations")
    //         .fetch_all(&pool)
    //         .await
    //         .unwrap();
    //
    // println!("puzzles:");
    // for puzzle in puzzles {
    //     println!(
    //         "[{}] path: {},index: {}",
    //         puzzle.id, &puzzle.path_json, &puzzle.iteration_index
    //     );
    // }

    // // insert
    // let result = sqlx::query("INSERT INTO fishy_website_com (email) VALUES (?)")
    //     .bind("bobby")
    //     .execute(&pool)
    //     .await
    //     .unwrap();
    // println!("Query result: {:?}", result);
    //

    // delete
    // delete any old puzzle at the start of the site
    // let delete_result = sqlx::query("DELETE FROM fishy_website_com  WHERE email=$1")
    //     .bind("bar@foo.com")
    //     .execute(&pool)
    //     .await
    //     .unwrap();
    // println!("Delete result: {:?}", delete_result);

    //end
    pool
}

// end db

#[derive(Template)]
#[template(path = "random.html")]
struct RandomBoard {
    title: String,
    random_board: Vec<i32>,
}

#[derive(Template)]
#[template(path = "iteration.html")]
struct Path {
    title: String,
    path: Vec<Vec<Vec<i32>>>,
}

async fn path_index_whole_board(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let board = slayin::Board::random_board();
    let mut returned_board = slayin::Board::sliding_puzzle_a_star(board).unwrap();
    returned_board.reverse();

    let template = Path {
        title: String::from("path page"),
        path: returned_board
            .iter()
            .map(|row| row.chunks(3).map(|chunk| chunk.to_vec()).collect())
            .collect(),
    };
    // println!("path_index={:?}", template.path);
    // println!("len={:?}", template.path.len());

    // Delete existing data from the database
    let delete_result = sqlx::query("DELETE FROM full_path")
        .execute(&pool)
        .await
        .unwrap();
    // println!("Delete result: {:?}", delete_result);

    let iteration =
        sqlx::query_as::<_, PuzzleIteration>("SELECT * FROM full_path WHERE iteration_index = ?")
            .fetch_optional(&pool)
            .await;
    // println!("Query result: {:?}", iteration);

    for (index, board) in returned_board.iter().enumerate() {
        // println!("returned_board={:?}", returned_board.len());
        // println!("board={:?}", board);
        let mut board_string = String::new();

        for (i, &num) in board.iter().enumerate() {
            let separator = if i == 2 || i == 5 { ", " } else { " " };
            board_string.push_str(&format!("{}{}", num, separator));
        }

        let mut board = board_string.trim_end().to_string();
        if index == returned_board.len() - 1 {
            // println!("a7eeh");
            board.push_str(",atoobees compelete");
        };

        // println!("{}", board);
        let result =
            sqlx::query("INSERT INTO full_path (path_json, iteration_index) VALUES (?, ?)")
                .bind(board) // Serialize board to JSON string
                .bind(index.to_string())
                .execute(&pool)
                .await
                .unwrap();
        // println!("Query result: {:?}", result);
    }

    template
}

// testing
// async fn random_board_index() -> impl IntoResponse {
//     let template = RandomBoard {
//         title: String::from("random page"),
//         random_board: vec![1, 2, 3, 4, 5, 6, 0, 7, 8],
//     };
//     println!("template={:?}", template.random_board);
//     template
// }
