use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use authenticator;

async fn routes(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    
    match (req.method(), req.uri().path()){
         // Serve some instructions at /
         (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world'`",
        ))),

        // Simply echo the body back to the client.
        (&Method::POST, "/") => Ok(Response::new(req.into_body())),

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }

}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    authenticator::hello_world();
    authenticator::Authenticator::serialize("This should be the user");
    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_,  hyper::Error>(service_fn(routes)) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}