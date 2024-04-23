use async_std::io::WriteExt;
use async_std::net::{Incoming, TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use cheapo_request::cheapo_request;

async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.peek(&mut buffer).await?;

    let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
    stream.write_all(response.as_bytes()).await?;
    stream.shutdown(std::net::Shutdown::Write)?;
    // Проверяем, нужно ли завершить работу сервера

    Ok(())
}

pub async fn run_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running at 127.0.0.1:8080");
    let mut incoming: Incoming<'_> = listener.incoming();
    // Accept connections and process them
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        task::spawn(handle_client(stream));

        return Ok(());
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    task::block_on(async {
        // Запуск тестового сервера
        let server_handle = task::spawn(run_server());

        // Выполнение запроса к серверу
        let response = cheapo_request("localhost", 8080, "/").await?;
        println!("{}", response);

        // Ожидание завершения тестового сервера
        server_handle.await?;

        Ok(())
    })
}
