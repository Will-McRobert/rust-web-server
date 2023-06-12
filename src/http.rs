pub mod http {
    enum HttpRequestMethod {
        GET,
        HEAD,
        POST,
        PUT,
        DELETE,
        CONNECT,
        OPTIONS,
        TRACE,
        PATCH,
    }

    // enum HttpRequestHeaderMethod {
    //     AIM,
    //     ACCEPT,
    //     ACCEPT_CHARSET,
    //     ACCEPT_DATETIME,
    //     ACCEPT_ENCODING,
    //     ACCEPT_LANGUAGE,
    //     ACCESS_CONTROL_REQUEST_METHOD,
    //     AUTHORIZATION,
    //     CACHE_CONTROL,
    //     CONNECTION,
    //     CONTENT_ENCODING,
    //     CONTENT_LENGTH,
    //     CONTENT_MD5,
    //     CONTENT_TYPE,
    //     COOKIE,
    //     DATE,
    //     DNT,
    //     EXPECT,
    //     FORWARDED,
    //     FROM,
    //     FRONT_END_HTTPS,
    //     HOST,
    //     HTTP2_SETTINGS,
    //     IF_MATCH,
    //     IF_MODIFIED_SINCE,
    //     IF_NONE_MATCH,
    //     IF_RANGE,
    //     IF_UNMODIFIED_SINCE,
    //     MAX_FORWARDS,
    //     ORIGIN,
    //     PRAGMA,
    //     PREFER,
    //     PROXY_AUTHORIZATION,
    //     RANGE,
    //     REFERER,
    //     SAVE_DATA,
    //     SEC_GPC,
    //     TE,
    //     TRAILER,
    //     TRANSFER_ENCODING,
    //     USER_AGENT,
    //     UPGRADE,
    //     UPGRADE_INSECURE_REQUESTS,
    //     VIA,
    //     WARNING,
    //     X_CSRF_TOKEN,
    //     X_FORWARDED_FOR,
    //     X_FORWARDED_HOST,
    //     X_FORWARDED_PROTO,
    //     X_HTTP_METHOD_OVERRIDE,
    //     X_ATT_DEVICE_ID,
    //     X_REQUEST_ID,
    //     X_REQUESTED_WITH,
    //     X_UIDH,
    //     X_WAP_PROFILE
    // }

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
            // Implement error checking for inputs
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
}
