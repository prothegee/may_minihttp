use may_minihttp::{HttpServer, HttpService, Request, Response};
use std::{fs, io};

#[derive(Clone)]
struct StaticServe;

impl HttpService for StaticServe {
    fn call(&mut self, req: Request, resp: &mut Response) -> io::Result<()> {
        /*
        NOTE:
        - you may use mime_guess to adjust the header string format:
            - check the path
            - chose static root to serve
            - do some sanitize
            - check if file from path request is ok
            - then read it to memory
            - check mime type, then pass content-type to it 
        - if it's not file you can pass it to request route
        - some example implementation:
            - https://github.com/prothegee/may_minihttp-example/tree/static_serve
        */

        let path = req.path();
        let file_path = if path == "/" {
            "static/index.html"
        } else {
            &format!("static{}", path)
        };

        match fs::read(file_path) {
            Ok(contents) => {
                resp.header("Content-Type: text/html")
                    .body_vec(contents);
            }
            Err(_) => {
                resp.status_code(404, "Not Found")
                    .header("Content-Type: text/plain")
                    .body("404 Not Found");
            }
        }

        Ok(())
    }
}

fn main() {
    env_logger::init();
    let server = HttpServer(StaticServe).start("127.0.0.1:8080").unwrap();
    server.wait();
}
