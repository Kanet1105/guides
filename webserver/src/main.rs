use webserver::Server;

fn main() {
    let mut app = Server::new("127.0.0.1:8000");
    app.register_url("/", "D:\\RustProjects\\guides\\webserver\\src\\static\\index.html");
    app.run().unwrap();
}