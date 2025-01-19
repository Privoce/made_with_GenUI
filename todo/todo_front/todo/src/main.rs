use gen_macro::plugin;

fn init() -> HttpPublisher {
    let mut http = HttpPublisher::new("cholee-todo-app.fly.dev/api/todos/");
    http.basic.protocol = Protocol::Https;
    http
}

plugin! {
    http1: HttpPublisher => init()
}