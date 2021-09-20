#[macro_use] extern crate rocket;
mod blockchain;
use crate::blockchain::Blockchain;

// #[get("/")]
// fn index(_: rocket::State<Blockchain>) -> &'static str {
//     "Let's do something awseome!"
// }

#[get("/")]
fn mine_block(chain: rocket::State<Blockchain>) -> &'static str {
    &format!("{}", chain.chain[0].timestamp)
}

#[launch]
fn rocket() -> _ {
    let mut chain = Blockchain::new();
    let genesis_index = chain.create_block(1, String::from("0"));

    rocket::build()
        .manage(chain)
        // .mount("/", routes![index])
        .mount("/mine_block", routes![mine_block])
}
