use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::sync::Mutex;
use sysinfo::{Components, Disks, Networks, System,};


#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: u32,
    name: String,
    status: String,
    stream_url: String
}

#[derive(Serialize, Deserialize, Clone)]
struct SystemData {
    host_name: String,
    uptime: String,
    rx_bytes: u64,
    tx_bytes: u64,
}

type Data = Mutex<Vec<Item>>;

const DATA_FILE: &str = "data.json";
const IP: &str = "0.0.0.0";
const PORT: u16 = 8080;

fn load_data() -> Vec<Item> {
    let mut file = File::open(DATA_FILE).unwrap_or_else(|_| File::create(DATA_FILE).unwrap());
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    if contents.trim().is_empty() {
        vec![]
    } else {
        serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
    }
}

fn save_data(data: &[Item]) {
    let json = serde_json::to_string_pretty(data).unwrap();
    let mut file = File::create(DATA_FILE).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

#[get("/system")]
async fn get_system_info(items: web::Data<Data>) -> impl Responder {
    let host_name = System::host_name().unwrap_or_else(|| "unknown".to_string());

    let uptime = "2:15".to_string();

    let networks = Networks::new_with_refreshed_list();
    let (rx_bytes, tx_bytes) = if let Some(net) = networks.get("enp7s0") {
        (net.total_received(), net.total_transmitted())
    } else {
        (0, 0)
    };

    let data = SystemData {
        host_name,
        uptime,
        rx_bytes,
        tx_bytes,
    };

    HttpResponse::Ok().json(data)
}

#[get("/items")]
async fn get_items(data: web::Data<Data>) -> impl Responder {
    let data = data.lock().unwrap();
    HttpResponse::Ok().json(&*data)
}

#[get("/add")]
async fn add_item(
    query: web::Query<Item>,
    data: web::Data<Data>,
) -> impl Responder {
    let mut data = data.lock().unwrap();
    let item = query.into_inner();

    if data.iter().any(|i| i.id == item.id) {
        return HttpResponse::BadRequest().body("ID already exists");
    }

    data.push(item);
    save_data(&data);
    HttpResponse::Ok().body("Item added")
}

#[get("/update")]
async fn update_item(
    query: web::Query<Item>,
    data: web::Data<Data>,
) -> impl Responder {
    let mut data = data.lock().unwrap();
    let item = query.into_inner();

    if let Some(existing) = data.iter_mut().find(|i| i.id == item.id) {
        existing.name = item.name;
        existing.status = item.status;
        existing.stream_url = item.stream_url;
        save_data(&data);
        HttpResponse::Ok().body("Item updated")
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

#[get("/delete")]
async fn delete_item(
    query: web::Query<std::collections::HashMap<String, String>>,
    data: web::Data<Data>,
) -> impl Responder {
    let id_str = query.get("id");

    if let Some(id_str) = id_str {
        if let Ok(id) = id_str.parse::<u32>() {
            let mut data = data.lock().unwrap();
            let len_before = data.len();
            data.retain(|item| item.id != id);

            if data.len() != len_before {
                save_data(&data);
                return HttpResponse::Ok().body("Item deleted");
            } else {
                return HttpResponse::NotFound().body("Item not found");
            }
        }
    }

    HttpResponse::BadRequest().body("Missing or invalid id")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let initial_data = load_data();
    let shared_data = web::Data::new(Mutex::new(initial_data));

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(get_items)
            .service(add_item)
            .service(update_item)
            .service(delete_item)
            .service(get_system_info)
    })
    .bind((IP, PORT))?
    .run()
    .await


}
