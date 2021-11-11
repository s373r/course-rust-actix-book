// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 1. Getting Started
//    https://actix.rs/book/actix/sec-1-getting-started.html

use actix::prelude::*;

struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

// NOTE: combine types into one place
mod ping_message {
    pub type Result = usize;

    #[derive(actix::Message)]
    #[rtype(result = "Result")]
    pub struct Type(pub Result);
}

impl Handler<ping_message::Type> for MyActor {
    type Result = ping_message::Result;

    fn handle(&mut self, msg: ping_message::Type, _ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;

        self.count
    }
}

#[actix_rt::main]
async fn main() {
    // start new actor
    let addr = MyActor { count: 10 }.start();

    // send message and get future for result
    let res = addr.send(ping_message::Type(10)).await;

    // handle() returns tokio handle
    println!("RESULT: {}", res.unwrap() == 20);

    // stop system and exit
    System::current().stop();
}
