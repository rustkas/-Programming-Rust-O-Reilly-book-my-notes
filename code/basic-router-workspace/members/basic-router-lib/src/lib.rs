use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}

pub struct Response {
    pub code: u32,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}

pub type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

pub struct BasicRouter {
    pub routes: HashMap<String, BoxedCallback>
}

pub fn not_found_response() -> Response {
    Response {
        code: 404,
        headers: HashMap::new(),
        body: b"<h1>Page not found</h1>".to_vec()
    }
}

impl BasicRouter {
    // Create an empty router.
    pub fn new() -> BasicRouter {
        BasicRouter { routes: HashMap::new() }
    }

    // Add a route to the router.
    pub fn add_route<C>(&mut self, url: &str, callback: C)
        where C: Fn(&Request) -> Response + 'static
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    pub fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => not_found_response(),
            Some(callback) => callback(request)
        }
    }
}

pub fn get_form_response() -> Response {
    Response {
        code: 200,
        headers: HashMap::new(),
        body: b"<form>".to_vec()
    }
}

pub fn get_gcd_response(_req: &Request) -> Response {
    Response {
        code: 500,
        headers: HashMap::new(),
        body: b"<h1>Internal server error</h1>".to_vec()
    }
}

pub fn req(url: &str) -> Request {
    Request {
        method: "GET".to_string(),
        url: url.to_string(),
        headers: HashMap::new(),
        body: vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router() {
        let mut router = BasicRouter::new();
        router.add_route("/", |_| get_form_response());
        router.add_route("/gcd", |req| get_gcd_response(req));
    
        assert_eq!(router.handle_request(&req("/piano")).code, 404);
        assert_eq!(router.handle_request(&req("/")).code, 200);
        assert_eq!(router.handle_request(&req("/gcd")).code, 500);
    }
}
