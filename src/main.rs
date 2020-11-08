use actix_web::{ web, App, HttpRequest, HttpResponse, HttpServer, Responder };
use actix_web::http::header::ToStrError;

fn get_header_value<'a>(header: &'a str, req: &'a HttpRequest) -> Option<&'a str> {
  match req.headers().get(header) {
    Some(header_value) => {
      match header_value.to_str() {
        Ok(event_str) => Some(event_str),
        Err(_err) => None,
      }
    },
    None => None,
  }
}
async fn webhook(req: HttpRequest) -> impl Responder {
  match get_header_value("X-GitHub-Event", &req) {
    Some("push") => {
      //   push 元のブランチ取得、push 元のリポジトリー URL取得
      //   git clone --branch=[push 元のブランチ] [push 元のリポジトリー URL]
      //   クローンしたリポジトリーから .github/vagusci/actions.yml を読み取る
      //   .github/vagusci/actions.yml を GH Actions 形式に書き換える
      //   クローンしたリポジトリーに .github/workflows/actions.yml を作成し git commit --amend && git push origin [push 元のブランチ]
    },
    Some(event) => { panic!("Unknown event type {}", event) },
    None => { panic!("X-GitHub-Event was not found.") }, // TODO return 400 Bad Request
  };

  HttpResponse::Ok().body("This is webhook")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .route("/webhook", web::get().to(webhook))
  }).bind("127.0.0.1:8088")?
  .run()
  .await
}
