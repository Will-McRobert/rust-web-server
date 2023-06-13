pub mod http {
    use std::fs::{read_to_string};
    use std::io::prelude::*;
    use std::net::{TcpListener, TcpStream};
    use std::str;
    use std::thread;

    pub struct HttpRequestHeader {
        method: String,
        value: String,
    }

    impl HttpRequestHeader {
        pub fn new(method: &str, value: &str) -> Self {
            HttpRequestHeader {
                method: method.to_string(),
                value: value.to_string(),
            }
        }

        pub fn get_method(&self) -> &str {
            &self.method
        }

        pub fn get_value(&self) -> &str {
            &self.value
        }

        pub fn to_string(&self) -> String {
            format!("{}: {}\r\n", self.method, self.value)
        }
    }

    pub struct HttpResponseHeader {
        method: String,
        value: String,
    }

    impl HttpResponseHeader {
        pub fn new(method: &str, value: &str) -> Self {
            Self {
                method: method.to_string(),
                value: value.to_string(),
            }
        }

        pub fn get_method(&self) -> &str {
            &self.method
        }

        pub fn get_value(&self) -> &str {
            &self.value
        }

        pub fn to_string(&self) -> String {
            format!("{}: {}\r\n", self.method, self.value)
        }
    }

    pub struct HttpRequest {
        method: String,
        path: String,
        query: String,
        headers: Vec<HttpRequestHeader>,
        body: String,
    }

    impl HttpRequest {
        pub fn new(
            method: &str,
            path: &str,
            query: &str,
            headers: Vec<HttpRequestHeader>,
            body: &str,
        ) -> Self {
            Self {
                method: method.to_string(),
                path: path.to_string(),
                query: query.to_string(),
                headers: headers,
                body: body.to_string(),
            }
        }

        pub fn get_method(&self) -> &str {
            &self.method
        }

        pub fn get_path(&self) -> &str {
            &self.path
        }

        pub fn get_headers(&self) -> &Vec<HttpRequestHeader> {
            &self.headers
        }

        pub fn from_string(request: &str) -> Self {
            let mut lines = request.split_terminator("\r\n");

            let mut request_values = lines.next().unwrap().split_whitespace();
            let method = request_values.next().unwrap();
            let path = request_values.next().unwrap();

            let mut request_headers: Vec<HttpRequestHeader> = Vec::new();
            let body = String::new();

            for line in lines {
                if line.trim() == "" {
                    break;
                }

                let mut header_string = line.split(": ");
                let header = header_string.next().unwrap();
                let value = header_string.next().unwrap();
                request_headers.push(HttpRequestHeader::new(header, value))
            }

            Self {
                method: method.to_string(),
                path: path.to_string(),
                query: "".to_string(),
                headers: request_headers,
                body: "".to_string(),
            }
        }

        pub fn to_string(&self) -> String {
            let mut request_string = format!("{} {} HTTP/1.1\r\n", self.method, self.path);

            for header in &self.headers {
                request_string += &header.to_string();
            }

            request_string
        }
    }

    pub struct HttpResponse {
        status_code: String,
        status_message: String,
        headers: Vec<HttpResponseHeader>,
        body: String,
    }

    impl HttpResponse {
        pub fn new(
            status_code: &str,
            status_message: &str,
            headers: Vec<HttpResponseHeader>,
            body: &str,
        ) -> Self {
            Self {
                status_code: status_code.to_string(),
                status_message: status_message.to_string(),
                headers: headers,
                body: body.to_string(),
            }
        }

        pub fn to_string(&self) -> String {
            let mut response_string =
                format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_message);

            for header in &self.headers {
                response_string += &header.to_string();
            }

            response_string += &format!("\r\n{}", &self.body);

            response_string
        }
    }

    pub fn listen(hostname: &str, port: &str) {
        let listener = TcpListener::bind(format!("{}:{}", hostname, port)).unwrap();
        println!("Server is listening on {}:{}", hostname, port);
    
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!(
                        "Client connected from {}",
                        stream.peer_addr().unwrap().to_string()
                    );
                    handle_connection(stream);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
    
    fn handle_connection(mut stream: TcpStream) {
        thread::spawn(move || {
            let mut buffer = [0; 2048];
            stream.read(&mut buffer[..]).unwrap();
    
            let request_string: &str = str::from_utf8(&buffer).unwrap();
    
            let request: HttpRequest = HttpRequest::from_string(request_string);
    
            let mut response: Vec<u8> = vec![];
    
            match request.get_method() {
                "GET" => match request.get_path() {
                    "/" => {
                        let contents = read_to_string("html/index.html").expect("Unable to read file");
                        let headers: Vec<HttpResponseHeader> =
                            vec![HttpResponseHeader::new("Content-Type", "text/html")];
                        let response_struct: HttpResponse =
                            HttpResponse::new("200", "OK", headers, &contents);
    
                        response = response_struct.to_string().into_bytes();
                    },
                    "/index.css" => {
                        let contents = read_to_string("html/index.css").expect("Unable to read file");
                        let headers: Vec<HttpResponseHeader> =
                            vec![HttpResponseHeader::new("Content-Type", "text/css")];
                        let response_struct: HttpResponse =
                            HttpResponse::new("200", "OK", headers, &contents);
    
                        response = response_struct.to_string().into_bytes();
                    },
                    "/register" => {read_to_string("html/index.html").expect("Unable to read file");
                        let contents =
                            read_to_string("html/register.html").expect("Unable to read file");
                        let headers: Vec<HttpResponseHeader> =
                            vec![HttpResponseHeader::new("Content-Type", "text/html")];
                        let response_struct: HttpResponse =
                            HttpResponse::new("200", "OK", headers, &contents);
    
                        response = response_struct.to_string().into_bytes();
                    }
                    &_ => {}
                },
                "POST" => match request.get_path() {
                    "/create/user" => {
                        response = b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\r\n\tOperation: createUser\r\n\tStatus: success\r\n}\r\n".to_vec();
                    }
                    &_ => {}
                },
                &_ => {}
            }
    
            println!("\nReceived request: \n{}", request.to_string().split("\r\n").next().unwrap());
            stream.write_all(&response).unwrap();
            stream.flush().unwrap();
        });
    }
    
    // enum HttpRequestMethod {
    //     GET,
    //     HEAD,
    //     POST,
    //     PUT,
    //     DELETE,
    //     CONNECT,
    //     OPTIONS,
    //     TRACE,
    //     PATCH,
    // }
    
    // enum HttpStatus {
    //     Continue = 100,
    //     SwitchingProtocols = 101,
    //     Processing = 102,
    //     EarlyHints = 103,
    //     Success = 200,
    //     Created = 201,
    //     Accepted = 202,
    //     NonAuthoritativeInformation = 203,
    //     NoContent = 204,
    //     ResetContent = 205,
    //     PartialContent = 206,
    //     MultiStatus = 207,
    //     AlreadyReported = 208,
    //     IMUsed = 226,
    //     MultipleChoices = 300,
    //     MovedPermanently = 301,
    //     Found = 302,
    //     SeeOther = 303,
    //     NotModified = 304,
    //     UseProxy = 305,
    //     Unused = 306,
    //     TemporaryRedirect = 307,
    //     PermanentRedirect = 308,
    //     BadRequest = 400,
    //     Unauthorized = 401,
    //     PaymentRequired = 402,
    //     Forbidden = 403,
    //     NotFound = 404,
    //     MethodNotAllowed = 405,
    //     NotAcceptable = 406,
    //     ProxyAuthenticationRequired = 407,
    //     RequestTimeout = 408,
    //     Conflict = 409,
    //     Gone = 410,
    //     LengthRequired = 411,
    //     PreconditionFailed = 412,
    //     PayloadTooLarge = 413,
    //     URITooLong = 414,
    //     UnsupportedMediaType = 415,
    //     RangeNotSatisfiable = 416,
    //     ExpectationFailed = 417,
    //     ImATeapot = 418,
    //     MisdirectedRequest = 421,
    //     UnprocessableContent = 422,
    //     Locked = 423,
    //     FailedDependency = 424,
    //     TooEarly = 425,
    //     UpgradeRequired = 426,
    //     PreconditionRequired = 428,
    //     TooManyRequests = 429,
    //     RequestHeaderFieldsTooLarge = 431,
    //     UnavailableForLegalReasons = 451,
    //     InternalServerError = 500,
    //     NotImplemented = 501,
    //     BadGateway = 502,
    //     ServiceUnavailable = 503,
    //     GatewayTimeout = 504,
    //     HTTPVersionNotSupported = 505,
    //     VariantAlsoNegotiates = 506,
    //     InsufficientStorage = 507,
    //     LoopDetected = 508,
    //     NotExtended = 510,
    //     NetworkAuthenticationRequired = 511
    // }
    
    // enum HttpStatus {
    //     Continue = 100,
    //     SwitchingProtocols = 101,
    //     Processing = 102,
    //     EarlyHints = 103,
    //     Success = 200,
    //     Created = 201,
    //     Accepted = 202,
    //     NonAuthoritativeInformation = 203,
    //     NoContent = 204,
    //     ResetContent = 205,
    //     PartialContent = 206,
    //     MultiStatus = 207,
    //     AlreadyReported = 208,
    //     IMUsed = 226,
    //     MultipleChoices = 300,
    //     MovedPermanently = 301,
    //     Found = 302,
    //     SeeOther = 303,
    //     NotModified = 304,
    //     UseProxy = 305,
    //     Unused = 306,
    //     TemporaryRedirect = 307,
    //     PermanentRedirect = 308,
    //     BadRequest = 400,
    //     Unauthorized = 401,
    //     PaymentRequired = 402,
    //     Forbidden = 403,
    //     NotFound = 404,
    //     MethodNotAllowed = 405,
    //     NotAcceptable = 406,
    //     ProxyAuthenticationRequired = 407,
    //     RequestTimeout = 408,
    //     Conflict = 409,
    //     Gone = 410,
    //     LengthRequired = 411,
    //     PreconditionFailed = 412,
    //     PayloadTooLarge = 413,
    //     URITooLong = 414,
    //     UnsupportedMediaType = 415,
    //     RangeNotSatisfiable = 416,
    //     ExpectationFailed = 417,
    //     ImATeapot = 418,
    //     MisdirectedRequest = 421,
    //     UnprocessableContent = 422,
    //     Locked = 423,
    //     FailedDependency = 424,
    //     TooEarly = 425,
    //     UpgradeRequired = 426,
    //     PreconditionRequired = 428,
    //     TooManyRequests = 429,
    //     RequestHeaderFieldsTooLarge = 431,
    //     UnavailableForLegalReasons = 451,
    //     InternalServerError = 500,
    //     NotImplemented = 501,
    //     BadGateway = 502,
    //     ServiceUnavailable = 503,
    //     GatewayTimeout = 504,
    //     HTTPVersionNotSupported = 505,
    //     VariantAlsoNegotiates = 506,
    //     InsufficientStorage = 507,
    //     LoopDetected = 508,
    //     NotExtended = 510,
    //     NetworkAuthenticationRequired = 511
    // }
}
