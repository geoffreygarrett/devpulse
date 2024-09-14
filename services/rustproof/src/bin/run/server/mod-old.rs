mod http;

use axum::Router;
use rustproof::config::ServerConfig;
use std::net::{SocketAddr, TcpListener};

pub async fn run_server(config: ServerConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut handles = Vec::new();

    if config.enable_http {
        let http_handle = tokio::spawn(retry_on_failure(move || async {
            // HTTP server setup
            let http_addr = SocketAddr::from((
                config.api_host.as_deref().unwrap_or("127.0.0.1").parse::<[u8; 4]>().unwrap(),
                config.http_port
            ));
            let http_listener = TcpListener::bind(http_addr).await?;

            println!("HTTP server running at http://{}", http_addr);

            let app = Router::new(); // Initialize your Axum routes
            axum::Server::from_tcp(http_listener)
                .unwrap()
                .serve(app.into_make_service())
                .await
                .map_err(|e| e.into()) // Convert to Box<dyn std::error::Error>
        }));

        handles.push(http_handle);
    }

    if config.enable_grpc {
        // let grpc_handle = tokio::spawn(retry_on_failure(move || async {
        //     // gRPC server setup
        //     let grpc_addr = SocketAddr::from((
        //         config.api_host.as_deref().unwrap_or("127.0.0.1").parse::<[u8; 4]>().unwrap(),
        //         config.grpc_port
        //     ));
        //     let grpc_listener = TcpListener::bind(grpc_addr).await?;
        //
        //     println!("gRPC server running at {}", grpc_addr);
        //
        //     let grpc_service = YourGrpcService::default(); // Initialize your gRPC service
        //     GrpcServer::builder()
        //         .add_service(your_grpc_service::your_grpc_server::YourGrpcServiceServer::new(grpc_service))
        //         .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(grpc_listener))
        //         .await
        //         .map_err(|e| e.into()) // Convert to Box<dyn std::error::Error>
        // }));
        //
        // handles.push(grpc_handle);
    }

    if !config.enable_http && !config.enable_grpc {
        println!("Both HTTP and gRPC servers are disabled. Exiting...");
        return Ok(());
    }

    // Await all server tasks to complete or return on the first error
    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("Server task error: {}", e);
            return Err(Box::new(e));
        }
    }

    Ok(())
}

async fn retry_on_failure<F, Fut>(mut task: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut() -> Fut + Send + 'static,
    Fut: std::future::Future<Output=Result<(), Box<dyn std::error::Error>>> + Send,
{
    loop {
        match task().await {
            Ok(_) => break Ok(()),
            Err(e) => {
                eprintln!("Server error: {}. Restarting...", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await; // Wait before retrying
            }
        }
    }
}
