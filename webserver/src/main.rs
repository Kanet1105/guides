use webserver::Server;

fn main() {
    let app = Server::new("127.0.0.1:8000");
    app.run();
}
