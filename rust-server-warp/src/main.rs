use warp::{Filter};

#[tokio::main]
async fn main() {
    let hello = warp::path::end()
        .map(|| format!("Rust-vs-node benchmark<br/>Rust-server-warp benchmark server"));

    let fib = warp::path!("fib" / i64)
        .map(|length| {
            let mut a:i64 = 1;
            let mut b = 0;
            let mut length = length;
            
            while length >= 0 {
                let temp = a;
                a = a + b;
                b = temp;
                length -= 1;
            }
        
            format!("{}", b)
        }); 

    warp::serve(hello.or(fib))
        .run(([0, 0, 0, 0], 3002))
        .await;
}