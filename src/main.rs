use actix_web::{web, HttpServer, App, HttpResponse, get, post};
use serde::{Deserialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the server on 127.0.0.1:3000...");
    HttpServer::new(|| App::new()
        .service(get_index)
        .service(post_result)
    )
        .bind("127.0.0.1:3000")?
        .run()
        .await
}

#[derive(Deserialize)]
struct GCDParams {
    m: u64,
    n: u64,
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(!(m == 0 && n == 0)); // m and n cannot be both zero
    while m > 0 && n > 0 {
        if m > n {
            m = m - n
        } else {
            n = n - m
        }
    }
    if m == 0 {
        n
    } else {
        m
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(2, 3), 1);
    assert_eq!(gcd(5, 15), 5);
    assert_eq!(gcd(20, 30), 10);
    assert_eq!(gcd(7, 9), 1);
    assert_eq!(gcd(35, 210), 35);
    assert_eq!(gcd(0, 10), 10);
}

#[test]
#[should_panic]
fn test_gcd_with_two_zeros() {
    gcd(0, 0);
}

#[get("/")]
async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r##"
                <title>GCD calculator</title>
                <form action="/result" method="post">
                  <input type="text" name="n"/>
                  <input type="text" name="m"/>
                  <button type="submit">Compute GCD</button>
                </form>
            "##
        )
}


#[post("/result")]
async fn post_result(form: web::Form<GCDParams>) -> HttpResponse {
    if form.m == 0 && form.n == 0 {
        HttpResponse::Ok()
            .content_type("text/html")
            .body("The gcd of 0 and 0 is <b>0</b>. \
            <a target='_blank' href='https://math.stackexchange.com/questions/495119/what-is-gcd0-0'>Here is why</a>")
    } else {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("The gcd of {} and {} is <b>{}</b>", form.m, form.n, gcd(form.m, form.n)))
    }
}