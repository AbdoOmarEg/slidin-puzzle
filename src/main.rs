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
        .route(
            "/first_iteration",
            get(path_index_whole_board), /* .post(slide) */
        )
        .route("/rest_iterations", get(path_index_iterations))
        .route("/loading", get(loading))
        .route("/slide", get(slide))
        .route("/solve", get(solve))
        .route("/loadz", get(loadz))
        .with_state(pool);
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "loadz.html")]
struct LoadzTemplate {
    title: String,
}

async fn loadz() -> impl IntoResponse {
    LoadzTemplate {
        title: String::from("loadzzzing..."),
    }
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

// #[derive(Template, Debug)]
// #[template(path = "iteration2.html")]
// struct StepTemplate {
//     title: String,
//     path: Vec<String>,
//     step: i32,
//     is_last_step: bool,
// }

#[derive(Deserialize, Debug)]
struct Input {
    index: i32,
}

// add query for the index

// put in the db
async fn path_index_iterations(
    State(pool): State<SqlitePool>,
    Query(query): Query<Input>,
) -> impl IntoResponse {
    let index = query.index;
    // println!("index={:?}", index);
    // println!("rest_iterations initiating with index={}", index);

    let iterations =
        sqlx::query_as::<_, PuzzleIteration>("SELECT * FROM full_path WHERE iteration_index = ?")
            .bind(index)
            .fetch_one(&pool)
            .await
            .unwrap();
    // println!("line 84 Query result: {:?}", iterations);
    let path = iterations.path_json;
    let path: Vec<i32> = path
        .split(", ") // split by comma and space
        .flat_map(|s| s.split_whitespace()) // split each substring by whitespace
        .map(|s| s.parse().unwrap()) // parse each number
        .collect::<Vec<i32>>();
    // println!("path={:?}", path);

    let mut returned_board = slayin::Board::sliding_puzzle_a_star(Board::new(path)).unwrap();
    returned_board.reverse();
    // println!("returned_board={:?}", returned_board);

    // if let Ok(iteration) = iterations {
    // let path: String = serde_json::from_str(&iteration.path_json).unwrap();
    // let path: String = iteration.path_json;
    //
    // // println!("path from json ={:?} and needed to be changed to 2x2", path);
    // let path: Vec<String> = path
    //     .split(",")
    //     .map(|s| s.to_string())
    //     .collect::<Vec<String>>();
    // println!("path path_index_z={:?}", path);
    // if path.len() == 4 {
    //     let template = StepTemplate {
    //         title: String::from("path page"),
    //         path,
    //         step: iteration.iteration_index,
    //         is_last_step: true,
    //     };
    //     return template.render().unwrap().into_response();
    // }

    for (idx, state) in returned_board.iter().enumerate() {
        let mut path: String = state
            .chunks(3)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|&x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join(", ");

        // println!("{}", path);
        // if path.len() == 4 {
        //     let template = StepTemplate {
        //         title: String::from("path page"),
        //         path,
        //         step: iteration.iteration_index,
        //         is_last_step: true,
        //     };
        //     }
        if idx == returned_board.len() - 1 {
            // println!("a7eeh");
            path.push_str(",atoobees compelete");
        };

        let result =
            sqlx::query("INSERT INTO full_path (path_json, iteration_index) VALUES (?, ?)")
                .bind(path) // Serialize board to JSON string
                .bind(idx as i32)
                .execute(&pool)
                .await
                .unwrap();
        // println!("result={:?}", result);
    }

    // let path: Vec<String> = returned_board[0]
    //     .chunks(3)
    //     .map(|chunk| {
    //         chunk
    //             .iter()
    //             .map(|&num| num.to_string())
    //             .collect::<Vec<String>>()
    //             .join(" ")
    //     })
    //     .collect();
    // println!("path path_index_z={:?}", path);

    let path: Vec<Vec<i32>> = returned_board[0]
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect();
    // println!("path path_index_z={:?}", path);

    // let template = StepTemplate {
    //     // title: String::from("ya 7lal ya 3leem ya rzak ya kareem");
    //     title: String::from("an 7elw w anta 7elw"),
    //     path,
    //     step: query.index + 1,
    //     is_last_step: false,
    // };
    let template = PathTany {
        title: String::from("an 7elw w anta 7elw"),
        path,
        step: query.index + 1,
        is_last_step: false,
    };
    // println!("template={:?}", template);

    return template.render().unwrap().into_response();

    // } else {
    //     // unreachable else statement
    //     let iterations = sqlx::query_as::<_, PuzzleIteration>(
    //         "SELECT * FROM full_path WHERE iteration_index = ?",
    //     )
    //     .bind(index - 1)
    //     .fetch_one(&pool)
    //     .await
    //     .unwrap();
    //     // println!("last board, {:?}", iterations);
    //
    //     let path: String = iterations.path_json;
    //
    //     // println!("path from json ={:?} and needed to be changed to 2x2", path);
    //     let path: Vec<String> = path
    //         .split(",")
    //         .map(|s| s.to_string())
    //         .collect::<Vec<String>>();
    //     // println!("path path_index_z={:?}", path);
    //
    //     let template = StepTemplate {
    //         title: String::from("path page"),
    //         path,
    //         step: iterations.iteration_index,
    //         is_last_step: true,
    //     };
    //
    //     return template.render().unwrap().into_response();
    //     // axum::response::Html(String::from("<p>Iteration not found</p>")).into_response()
    // }
}

// start db

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

use crate::slayin::Board;

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
    // let result = sqlx::query(
    //     "SELECT name
    //      FROM sqlite_schema
    //      WHERE type ='table'
    //      AND name NOT LIKE 'sqlite_%';",
    // )
    // .fetch_all(&pool)
    // .await
    // .unwrap();
    // for (idx, row) in result.iter().enumerate() {
    // println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    // }

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

#[derive(Template, Debug)]
#[template(path = "iteration.html")]
struct Path {
    title: String,
    path: Vec<Vec<(usize, i32)>>,
}

#[derive(Template, Debug)]
#[template(path = "iteration2.html")]
struct PathTany {
    title: String,
    path: Vec<Vec<i32>>,
    step: i32,
    is_last_step: bool,
}

#[derive(Template)]
#[template(path = "slide.html")]
struct SlideTemplate {
    title: String,
    path: Vec<Vec<(usize, i32)>>,
}

#[derive(Deserialize, Debug)]
struct Edxd {
    idx: i32,
}

// hx-get="/solve?index={{step}}
async fn solve(State(pool): State<SqlitePool>, Query(query): Query<Input>) -> impl IntoResponse {
    let iteration =
        sqlx::query_as::<_, PuzzleIteration>("SELECT * FROM full_path WHERE iteration_index = ?")
            .bind(query.index)
            .fetch_one(&pool)
            .await
            .unwrap();
    // println!("Query result: {:?}", iteration);
    // println!("iteration={:?}", iteration);
    let path: String = iteration.path_json;

    //     let mut board_string = String::new();
    //
    // for (i, &num) in path.iter().enumerate() {
    //     let separator = if i == 2 || i == 5 { ", " } else { " " };
    //     board_string.push_str(&format!("{}{}", num, separator));
    // }
    //
    // let board = board_string.trim_end().to_string();
    // println!("board={:?}", board);

    let mut new = Vec::new();
    let mut ok = Vec::new();
    'outer: for (i, e) in path.split_whitespace().enumerate() {
        for e in e.split(",") {
            // println!("e={:?}", e);
            if e.is_empty() {
                continue;
            }
            if e == "atoobees" {
                break 'outer;
            }

            ok.push(e.parse().unwrap());
            if i == 2 || i == 5 || i == 8 {
                new.push(ok.clone());
                ok.clear()
            }
        }
    }
    // println!("new in slide={:?}", new);
    // let template = StepTemplate {
    //     title: String::from("ya 7lal ya 3leem ya rzak ya kareem"),
    //     // title: String::from("an 7elw w anta 7elw"),
    //     path,
    //     step: query.index + 1,
    //     is_last_step: false,
    // };
    if path.len() >= 24 {
        return PathTany {
            title: String::from("الف مبرو9و9و9وك"),
            path: new,
            step: query.index,
            is_last_step: true,
        }
        .render()
        .unwrap()
        .into_response();
    }
    let template = PathTany {
        title: String::from("ya 7lal ya 3leem ya rzak ya kareem"),
        path: new,
        step: query.index + 1,
        is_last_step: false,
    };
    // println!("template={:?}", template);

    return template.render().unwrap().into_response();
}

async fn slide(State(pool): State<SqlitePool>, Query(edxd): Query<Edxd>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, PuzzleIteration>("SELECT * FROM full_path")
        .fetch_all(&pool)
        .await
        .unwrap();
    // println!("Query result ALL: {:?}\n\n", result);

    let idx = edxd.idx;
    // println!("edxd={:?}", edxd);
    let (i, j) = (idx / 3, idx % 3);
    let iteration =
        sqlx::query_as::<_, PuzzleIteration>("SELECT * FROM full_path WHERE iteration_index = ?")
            .bind(0)
            .fetch_one(&pool)
            .await
            .unwrap();
    // println!("Query result: {:?}", iteration);
    // println!("iteration={:?}", iteration);

    let path: String = iteration.path_json;

    // println!("path from json ={:?} and needed to be changed to 2x2", path);
    // 2d one iteration
    // let path: Vec<String> = path
    //     .split(",")
    //     .map(|s| s.to_string())
    //     .collect::<Vec<String>>();
    // println!("path={:?}", path);

    // let path: Vec<Vec<i32>> = path
    //     .split(",")
    //     .map(|s| {
    //         s.trim() // Remove leading/trailing whitespace
    //             .split_whitespace() // Split by whitespace
    //             .map(|num| num.parse::<i32>().unwrap()) // Parse each number
    //             .collect::<Vec<i32>>() // Collect into a vector
    //     })
    //     .collect::<Vec<Vec<i32>>>(); // Collect into a vector of vectors

    // let path = path.split(",").collect();
    let mut path: Vec<i32> = path
        .split(",")
        .flat_map(|s| s.split_whitespace()) // Split each substring by whitespace
        .map(|s| s.parse::<i32>().unwrap()) // Parse each substring as i32
        .collect(); // Collect into a vector
                    // println!("path={:?}", path);

    for (di, dj) in &[(0, 1), (1, 0), (-1, 0), (0, -1)] {
        let ni = di + i as i32;
        let nj = dj + j as i32;
        // if ni, nj valid and is zero then swap and insert to db
        let nidx = ni * 3 + nj;
        // println!("new new nidx={:?}", nidx);
        if nidx >= 0 && nidx < 9 && path[nidx as usize] == 0 {
            path.swap(nidx as usize, idx as usize);
            // println!("swap happened");
            // println!("idx={:?}", idx);
            // println!("nidx={:?}", nidx);
            // println!("path={:?}", path);

            let mut board_string = String::new();

            for (i, &num) in path.iter().enumerate() {
                let separator = if i == 2 || i == 5 { ", " } else { " " };
                board_string.push_str(&format!("{}{}", num, separator));
            }

            let board = board_string.trim_end().to_string();
            // println!("board={:?}", board);
            // if index == returned_board.len() - 1 {
            //     // println!("a7eeh");
            //     board.push_str(",atoobees compelete");
            // };

            let mut new = Vec::new();
            let mut ok = Vec::new();
            for (i, &e) in path.iter().enumerate() {
                ok.push((i, e));
                if i == 2 || i == 5 || i == 8 {
                    new.push(ok.clone());
                    ok.clear()
                }
            }
            // println!("new in slide={:?}", new);

            let result = sqlx::query_as::<_, PuzzleIteration>("SELECT * FROM full_path")
                .fetch_all(&pool)
                .await
                .unwrap();
            // println!("Query result ALL: {:?}\n\n", result);

            let result = sqlx::query(
                "UPDATE full_path SET path_json = ?, iteration_index = ? WHERE iteration_index = 0",
            )
            .bind(board.clone()) // Serialize board to JSON string
            .bind(0)
            // Add your condition here to specify which row(s) to update
            .execute(&pool)
            .await
            .unwrap();
            // println!("Query result: {:?}", result);
            // println!("Query result: {:?}", result);
            // println!("board={:?}", board);

            let result = sqlx::query_as::<_, PuzzleIteration>("SELECT * FROM full_path")
                .fetch_all(&pool)
                .await
                .unwrap();
            // println!("Query result after after insert: {:?}\n\n", result);

            return SlideTemplate {
                title: String::from("I like to move it move it"),
                path: new,
            };
        }
    }

    todo!()
}

// make it take just the first vec and adjust the html of it too
async fn path_index_whole_board(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let board = slayin::Board::random_board();
    // println!("board={:?}", board);
    // let mut returned_board = slayin::Board::sliding_puzzle_a_star(board.clone()).unwrap();
    // returned_board.reverse();
    // println!("returned_board={:?}", returned_board);
    let mut new = Vec::new();
    let mut ok = Vec::new();
    for (i, &e) in board.cells.iter().enumerate() {
        ok.push((i, e));
        if i == 2 || i == 5 || i == 8 {
            new.push(ok.clone());
            ok.clear()
        }
    }
    // println!("new={:?}", new);
    // let f: Vec<Vec<i32>> = returned_board
    //     .iter()
    //     .take(1)
    //     .map(|row| {
    //         row.chunks(3)
    //             .map(|chunk| chunk.to_vec())
    //             .collect::<Vec<_>>()
    //     })
    //     .flatten()
    //     .collect();
    // println!("f={:?}", f);
    // let f: Vec<Vec<i32>> = board.cells.chunks(3).map(|chunk| chunk.to_vec()).collect();
    // println!("f={:?}", f);
    let template = Path {
        title: String::from("path page"),
        path: new.clone(),
    };
    // println!("template={:?}", template);
    // println!("path_index={:?}", template.path);
    // println!("len={:?}", template.path.len());

    // Delete existing data from the database
    let delete_result = sqlx::query("DELETE FROM full_path")
        .execute(&pool)
        .await
        .unwrap();
    // println!("delete_result={:?}", delete_result);

    let mut board_string = String::new();
    // println!("new={:?}", new);

    for (i, &(_, x)) in new.iter().flatten().enumerate() {
        let separator = if i == 2 || i == 5 { ", " } else { " " };
        board_string.push_str(&format!("{}{}", x, separator));
        // println!("board_string={:?}", board_string);
    }

    let board = board_string.trim_end().to_string();
    // println!("board_string={:?}", board_string);
    // if index == returned_board.len() - 1 {
    //     // println!("a7eeh");
    //     board.push_str(",atoobees compelete");
    // };

    // println!("board to insert ={:?}", board);
    let result = sqlx::query("INSERT INTO full_path (path_json, iteration_index) VALUES (?, ?)")
        .bind(board.clone()) // Serialize board to JSON string
        .bind(0)
        .execute(&pool)
        .await
        .unwrap();
    // println!("result={:?}", result);

    // #[derive(Template)]
    // #[template(path = "iteration2.html")]
    // struct StepTemplate {
    //     title: String,
    //     path: Vec<String>,
    //     step: i32,
    //     is_last_step: bool,
    // }
    //
    // let path: Vec<String> = board
    //     .split(",")
    //     .map(|s| s.to_string())
    //     .collect::<Vec<String>>();
    // println!("path path_index_z={:?}", path);

    // let template = StepTemplate {
    //     // title: String::from("ya 7lal ya 3leem ya rzak ya kareem");
    //     title: String::from("an 7elw w anta 7elw"),
    //     path,
    //     step: 0,
    //     is_last_step: todo!(),
    // };
    // println!("template={:?}", template);

    template

    // println!("Delete result: {:?}", delete_result);

    // let iteration =
    //     sqlx::query_as::<_, PuzzleIteration>("SELECT * FROM full_path WHERE iteration_index = ?")
    //         .fetch_optional(&pool)
    //         .await;
    // println!("Query result: {:?}", iteration);

    // for (index, board) in returned_board.iter().enumerate() {
    //     // println!("returned_board={:?}", returned_board.len());
    //     // println!("board={:?}", board);
    //     let mut board_string = String::new();
    //
    //     for (i, &num) in board.iter().enumerate() {
    //         let separator = if i == 2 || i == 5 { ", " } else { " " };
    //         board_string.push_str(&format!("{}{}", num, separator));
    //     }
    //
    //     let mut board = board_string.trim_end().to_string();
    //     if index == returned_board.len() - 1 {
    //         // println!("a7eeh");
    //         board.push_str(",atoobees compelete");
    //     };
    //
    //     // println!("{}", board);
    //     let result =
    //         sqlx::query("INSERT INTO full_path (path_json, iteration_index) VALUES (?, ?)")
    //             .bind(board) // Serialize board to JSON string
    //             .bind(index.to_string())
    //             .execute(&pool)
    //             .await
    //             .unwrap();
    //     // println!("Query result: {:?}", result);
    // }

    // template
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
