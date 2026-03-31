pub fn exercise1() {
    // 1️⃣ Immutable variable
    let server_port: u16 = 8080;
    println!("Server port: {}", server_port);

    // 2️⃣ Mutable variable, increment 3 times
    let mut request_count = 0;
    request_count += 1;
    request_count += 1;
    request_count += 1;
    println!("Request count: {}", request_count);

    // 3️⃣ Tuple representing an HTTP response: (status_code, message, success_flag)
    let http_response: (u16, &str, bool) = (200, "SUCCESS", true);
    let (status_code, message, success) = http_response;
    println!("Status code: {}, Message{}, Success: {}", status_code, message, success);

    // 4️⃣ Array of HTTP methods
    let http_method: [&str; 5] = ["GET", "POST", "PUT", "PATCH", "DELETE"];

    // Print each using indexing
    println!("HTTP Methods:");
    println!("1: {}", http_method[0]);
    println!("2: {}", http_method[1]);
    println!("3: {}", http_method[2]);
    println!("4: {}", http_method[3]);
}
