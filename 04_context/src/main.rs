// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 4. Context
//    https://actix.rs/book/actix/sec-4-context.html

use actix::prelude::*;

#[actix_rt::main]
async fn main() {
    #[derive(Debug)]
    struct MyActor {
        count: usize,
    }

    impl Actor for MyActor {
        type Context = Context<Self>;

        fn started(&mut self, ctx: &mut Context<Self>) {
            ctx.set_mailbox_capacity(1);
        }

        fn stopped(&mut self, _ctx: &mut Context<Self>) {
            println!("Actor is stopped");
        }
    }

    #[derive(Message)]
    #[rtype(result = "usize")]
    struct Ping(usize);

    impl Handler<Ping> for MyActor {
        type Result = usize;

        fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
            self.count += msg.0;

            if self.count > 5 {
                println!("Shutting down ping receiver.");
                ctx.stop()
            }

            self.count
        }
    }

    struct WhoAmI;

    impl Message for WhoAmI {
        type Result = Result<actix::Addr<MyActor>, ()>;
    }

    impl Handler<WhoAmI> for MyActor {
        type Result = Result<actix::Addr<MyActor>, ()>;

        fn handle(&mut self, _msg: WhoAmI, ctx: &mut Context<Self>) -> Self::Result {
            Ok(ctx.address())
        }
    }

    let addr = MyActor { count: 10 }.start();
    let who_addr = addr.send(WhoAmI {}).await.unwrap().unwrap();

    println!("{:?}", who_addr);

    // send message and get future for result
    let addr_2 = addr.clone();
    let res = addr.send(Ping(6)).await;

    println!("{:?}", res);

    let res = addr_2.try_send(Ping(6));

    println!("{:?}", res);
}
