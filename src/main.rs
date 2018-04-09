#[macro_use]
extern crate rouille;

use rouille::Response;

fn main() {
  let index = include_str!("../ui/build/index.html");

  rouille::start_server("0.0.0.0:5000", move |request| {
    let response = router!(request,
        (GET) ["/"] => {
          Response::html(index)
        },
        _ => {
          let asset = rouille::match_assets(&request, "./ui/build/");
          if asset.is_success() {
            return asset;
          }
          
          Response::empty_404()
        }
      );

    response
  });
}
