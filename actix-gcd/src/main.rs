use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[actix_web::main] // Use Actix's runtime attribute
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index)) // Route for the index page
            .route("/gcd", web::post().to(post_gcd)) // Route to handle the GCD calculation
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

// Make the handler async
async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>

            <form action="/gcd" method="post">
                <input type="text" name="n" placeholder="First number"/>
                <input type="text" name="m" placeholder="Second number"/>
                <button type="submit">Compute GCD</button>
            </form>
        "#,
    )
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

// Make the handler async
async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing GCD with zero is boring");
    }

    let response = format!(
        "The greatest common divisor of numbers {} and {} \
             is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}
