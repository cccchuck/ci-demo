use axum::{Router, routing::get};

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello CD!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
