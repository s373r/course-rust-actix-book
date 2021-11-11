// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 2. Actor
//    https://actix.rs/book/actix/sec-2-actor.html

use actix::prelude::*;

#[actix_rt::main]
async fn main() {
    {
        /// Define message
        #[derive(Message)]
        #[rtype(result = "Result<bool, std::io::Error>")]
        struct Ping;

        // Define actor
        struct MyActor;

        // Provide Actor implementation for our actor
        impl Actor for MyActor {
            type Context = Context<Self>;

            fn started(&mut self, _ctx: &mut Context<Self>) {
                println!("Actor is alive");
            }

            fn stopped(&mut self, _ctx: &mut Context<Self>) {
                println!("Actor is stopped");
            }
        }

        /// Define handler for `Ping` message
        impl Handler<Ping> for MyActor {
            type Result = Result<bool, std::io::Error>;

            fn handle(&mut self, _msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
                println!("Ping received");

                Ok(true)
            }
        }

        // Start MyActor in current thread
        let addr = MyActor.start();

        // Send Ping message.
        // send() message returns Future object, that resolves to message result
        let result = addr.send(Ping).await;

        match result {
            Ok(res) => println!("Got result: {}", res.unwrap()),
            Err(err) => println!("Got error: {}", err),
        }
    }

    println!("---");

    {
        use actix::dev::{MessageResponse, ResponseChannel};

        #[derive(Message)]
        #[rtype(result = "Responses")]
        enum Messages {
            Ping,
            Pong,
        }

        enum Responses {
            GotPing,
            GotPong,
        }

        impl<A, M> MessageResponse<A, M> for Responses
        where
            A: Actor,
            M: Message<Result = Responses>,
        {
            fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
                if let Some(tx) = tx {
                    tx.send(self);
                }
            }
        }

        // Define actor
        struct MyActor;

        // Provide Actor implementation for our actor
        impl Actor for MyActor {
            type Context = Context<Self>;

            fn started(&mut self, _ctx: &mut Context<Self>) {
                println!("Actor is alive");
            }

            fn stopped(&mut self, _ctx: &mut Context<Self>) {
                println!("Actor is stopped");
            }
        }

        /// Define handler for `Messages` enum
        impl Handler<Messages> for MyActor {
            type Result = Responses;

            fn handle(&mut self, msg: Messages, _ctx: &mut Context<Self>) -> Self::Result {
                match msg {
                    Messages::Ping => Responses::GotPing,
                    Messages::Pong => Responses::GotPong,
                }
            }
        }

        // NOTE: extract a function (just for DRY)
        fn print_message_result(res: Result<Responses, MailboxError>) {
            match res {
                Ok(res) => match res {
                    Responses::GotPing => println!("Ping received"),
                    Responses::GotPong => println!("Pong received"),
                },
                Err(e) => println!("Actor is probably dead: {}", e),
            }
        }

        // Start MyActor in current thread
        let addr = MyActor.start();

        // Send Ping message.
        // send() message returns Future object, that resolves to message result
        let ping_future = addr.send(Messages::Ping).await;

        print_message_result(ping_future);

        let pong_future = addr.send(Messages::Pong).await;

        print_message_result(pong_future)
    }
}
