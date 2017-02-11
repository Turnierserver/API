use rocket::Response;
use rocket::response::Responder;
use rocket::http::Status;

#[derive(Debug, Clone, PartialEq)]
pub struct Cors<R>(pub R);

impl<'r, R: Responder<'r>> Responder<'r> for Cors<R> {
    fn respond(self) -> Result<Response<'r>, Status> {
        let mut build = Response::build();
        build.merge(self.0.respond()?);

        build
            .raw_header("Access-Control-Allow-Origin", "*")
            .raw_header("Access-Control-Allow-Methods", "GET,HEAD,PUT,PATCH,POST,DELETE")
            .ok()
    }
}