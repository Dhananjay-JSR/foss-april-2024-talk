use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Bind Load Balancer to Port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Unable to Bind Port 3000 Port");
    println!("\x1b[93m Load Balancer Listening on Port 3000\x1b[0m");
    // List of IP Address of Servers
    let ip_array = Arc::new(vec![
        "127.0.0.1:3001",
        "127.0.0.1:3002",
        "127.0.0.1:3003",
        "127.0.0.1:3004",
        "127.0.0.1:3005",
    ]);
    // Counter to keep track of the Server to which the next packet should be sent
    let count = Arc::new(tokio::sync::Mutex::new(0));
    loop {
        // Accept Connection from Client
        let (mut soc_stream, _) = listener
            .accept()
            .await
            .expect("Unable to Accept Connection from Bind");
        // Clone the Mutex and IP List to be used in the Tokio Spawn
        let cloned_mutex = Arc::clone(&count);
        let cloned_ip_list = Arc::clone(&ip_array);
        tokio::spawn(async move {
            // Lock the Mutex to get the Server to which the packet should be sent
            let mut mutex_value = cloned_mutex.lock().await;
            // Connect to the Server
            let mut server_conn =
                tokio::net::TcpStream::connect(cloned_ip_list[*mutex_value % cloned_ip_list.len()])
                    .await
                    .expect("Unable to Connect to Server");
            // Increment the Counter
            *mutex_value += 1;
            // Perform Packet Copy
            tokio::io::copy_bidirectional(&mut server_conn, &mut soc_stream)
                .await
                .expect("Unable to Perform packet Copy");
        });
    }
}
