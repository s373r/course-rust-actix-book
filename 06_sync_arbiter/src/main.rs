// SPDX-License-Identifier: GPL-3.0-or-later

// --- Index ---
// 6. SyncArbiter
//    https://actix.rs/book/actix/sec-6-sync-arbiter.html

// NOTE: an example from
//       https://actix.rs/actix/actix/sync/struct.SyncArbiter.html

use actix::prelude::*;
use futures::future::join_all;

#[derive(Message)]
#[rtype(result = "Result<u64, ()>")]
struct Fibonacci(pub u32);

struct SyncActor;

impl Actor for SyncActor {
    // It's important to note that you use "SyncContext" here instead of "Context".
    type Context = SyncContext<Self>;
}

impl Handler<Fibonacci> for SyncActor {
    type Result = Result<u64, ()>;

    fn handle(&mut self, msg: Fibonacci, _: &mut Self::Context) -> Self::Result {
        if msg.0 == 0 {
            Err(())
        } else if msg.0 == 1 {
            Ok(1)
        } else {
            let mut i = 0;
            let mut sum = 0;
            let mut last = 0;
            let mut curr = 1;

            while i < msg.0 - 1 {
                sum = last + curr;
                last = curr;
                curr = sum;
                i += 1;
            }

            Ok(sum)
        }
    }
}

fn main() {
    System::new("events").block_on(async {
        // Start the SyncArbiter with 2 threads, and receive the address of the Actor pool.
        let addr = SyncArbiter::start(2, || SyncActor);

        let futures: Vec<_> = (5..10).map(|i| addr.send(Fibonacci(i))).collect();
        let numbers = join_all(futures).await;

        println!("{:?}", numbers)
    });
}
