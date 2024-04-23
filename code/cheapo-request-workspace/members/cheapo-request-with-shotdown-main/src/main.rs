use async_std::io::WriteExt;
use async_std::net::{Incoming, TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use cheapo_request::cheapo_request;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

//TODO: server shutting down is not work. Please, find a way to implement shutting down correctly.
static mut IS_STOP: bool = false;

async fn handle_client(mut stream: TcpStream, should_stop: Arc<AtomicBool>) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.peek(&mut buffer).await?;

    let response = if buffer.starts_with(b"GET /shutdown") {
        // Если получен запрос на /shutdown, устанавливаем флаг should_stop
        should_stop.store(true, Ordering::Relaxed);

        unsafe {
            IS_STOP = true;
        }

        "HTTP/1.1 200 OK\r\n\r\nServer is shutting down"
    } else {
        "HTTP/1.1 200 OK\r\n\r\nHello, world!"
    };

    stream.write_all(response.as_bytes()).await?;
    stream.shutdown(std::net::Shutdown::Write)?;

    Ok(())
}

pub async fn run_server(should_stop: Arc<AtomicBool>) -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running at 127.0.0.1:8080");
    let mut incoming: Incoming<'_> = listener.incoming();

    // Accept connections and process them
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        let should_stop = should_stop.clone();
        let should_stop2 = should_stop.clone();
        task::spawn(handle_client(stream, should_stop));

        unsafe {
            if IS_STOP {
                break;
            }
        }

        // Проверяем, нужно ли завершить работу сервера
        if should_stop2.load(Ordering::Relaxed) {
            break;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    task::block_on(async {
        // Флаг для управления работой сервера
        let should_stop = Arc::new(AtomicBool::new(false));

        // Запуск тестового сервера
        let server_handle = task::spawn(run_server(should_stop.clone()));

        // Выполнение запроса к серверу
        let response = cheapo_request("localhost", 8080, "/").await?;
        println!("{}", response);

        // Отправка запроса на завершение работы сервера
        let response = cheapo_request("localhost", 8080, "/shutdown").await?;
        println!("{}", response);

        // Ожидание завершения тестового сервера
        server_handle.await?;

        Ok(())
    })
}
