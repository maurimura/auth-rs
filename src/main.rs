extern crate hyper;
use hyper::rt::Future;
use hyper::service::service_fn;

extern crate futures;
use futures::future;

use hyper::{Body, Request, Response, Server, Method, StatusCode};

type BoxFut = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn router(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {

            if req.headers().contains_key("cookie") {
                // Deserialize
                // Send response back
                *response.body_mut() = Body::from("Try POSTing data to /echo");
            }else {
                *response.body_mut() = Body::from("<b>Unauthorized</b>");
                *response.status_mut() = StatusCode::UNAUTHORIZED;

            }

        
        },
        (&Method::POST, "/echo") => {
            // Pass req body into res body
            *response.body_mut() = req.into_body();
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Box::new(future::ok(response))
}

fn main() {
    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn(router)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    hyper::rt::run(server);
}
