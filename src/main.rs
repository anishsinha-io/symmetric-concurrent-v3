mod bootstrap;
mod concurrency;
mod shared;
mod storage;
fn main() {
    println!("Hello, world!");
    println!("{}", shared::cwd().unwrap().to_string_lossy());
}
