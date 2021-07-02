use docker_api::{api::LogsOpts, conn::TtyChunk, Docker};
use futures::StreamExt;
use std::{env, str};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let docker = Docker::new("tcp://127.0.0.1:80")?;
    let id = env::args()
        .nth(1)
        .expect("You need to specify a service name");

    let mut logs_stream = docker
        .services()
        .get(&id)
        .logs(&LogsOpts::builder().stdout(true).stderr(true).build());

    while let Some(log_result) = logs_stream.next().await {
        match log_result {
            Ok(chunk) => print_chunk(chunk),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}

fn print_chunk(chunk: TtyChunk) {
    match chunk {
        TtyChunk::StdOut(bytes) => {
            println!("Stdout: {}", str::from_utf8(&bytes).unwrap_or_default())
        }
        TtyChunk::StdErr(bytes) => {
            eprintln!("Stdout: {}", str::from_utf8(&bytes).unwrap_or_default())
        }
        TtyChunk::StdIn(_) => unreachable!(),
    }
}
