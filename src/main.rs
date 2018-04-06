#[macro_use]
extern crate rouille;

use rouille::Response;

fn main() {
    let index = include_str!("../ui/build/index.html");
    let bundle = include_str!("../ui/build/static/js/main.ca62a29f.js");
    let styles = include_str!("../ui/build/static/css/main.c17080f1.css");
    let logo = include_str!("../ui/build/static/media/logo.5d5d9eef.svg");
    let sw = include_str!("../ui/build/service-worker.js");

    rouille::start_server("0.0.0.0:5000", move |request| {

        let response = router!(request,
            (GET) ["/"] => {
                Response::html(index)
            },
            (GET) ["/static/js/main.ca62a29f.js"] => {
                Response::text(bundle)
            },
            (GET) ["/static/css/main.c17080f1.css"] => {
                Response::text(styles)
            },
            (GET) ["/static/media/logo.5d5d9eef.svg"] => {
                Response::text(logo)
            },
            (GET) ["/service-worker.js"] => {
                Response::text(sw)
            },
            _ => {
                Response::empty_404()
            }
        );

        response
    });
}
