// WARNING: Do not modify definitions of public types or function names in this
// file – your solution will be tested automatically! Implement all missing parts.

use crossbeam_channel::{unbounded, Receiver, Sender};
use rand::Rng;
use std::thread;
use std::thread::JoinHandle;

type Num = u64;
type Ident = u128;

pub struct FibonacciModule {
    /// Currently hold number from the sequence.
    num: Num,
    /// Index of the required Fibonacci number (the `n`).
    limit: usize,
    /// Identifier of the module.
    id: Ident,
    /// Identifier of the other module.
    other: Option<Ident>,
    /// Queue for outgoing messages.
    queue: Sender<FibonacciSystemMessage>,
}

impl FibonacciModule {
    /// Create the module and register it in the system.
    pub fn create(
        initial_number: Num,
        limit: usize,
        queue: Sender<FibonacciSystemMessage>,
    ) -> Ident {
        // For the sake of simplicity, generate a random number and use it
        // as the module's identifier:
        let id = rand::thread_rng().gen();

        //unimplemented!();
        let tmp = FibonacciModule {
            num: initial_number,
            limit: limit,
            id: id,
            other: None,
            queue: queue.clone(),
        };

        queue.send(FibonacciSystemMessage::RegisterModule(tmp)).unwrap();

        id
    }

    /// Handle the step-message from the other module.
    ///
    /// Here the next number of the Fibonacci sequence is calculated.
    pub fn message(&mut self, idx: usize, num: Num) {
        if idx >= self.limit {
            // The calculation is done.
            //unimplemented!()
            self.queue.send(FibonacciSystemMessage::Done).unwrap()
        }

        //unimplemented!();
        self.num += num; 
        let tmp_idx = idx + 1;
        
        // Put the following `println!()` statement after performing
        // the update of `self.num`:
        println!("Inside {}, value: {}", self.id, self.num);
        
        //unimplemented!()
        match self.other {
            Some(id) => self.queue.send(FibonacciSystemMessage::Message{id: id, idx: tmp_idx, num: self.num}).unwrap(),
            None => std::process::exit(1),
        }
        //self.queue.send(FibonacciSystemMessage::Message{id: self.other, idx: tmp_idx, num: self.num});
        
    }

    /// Handle the init-message.
    ///
    /// The module finishes its initialization and initiates the calculation
    /// if it is the first to go.
    pub fn init(&mut self, other: Ident) {
        //unimplemented!();
        self.other = Some(other);
        if self.num == 0 {
            self.message(2, 1);
        }
    }
}

/// Messages sent to/from the modules.
///
/// The `id` field denotes which module should receive the message.
pub enum FibonacciSystemMessage {
    /// Register the module in the engine.
    ///
    /// Note that this is a struct without named fields: a tuple struct.
    RegisterModule(FibonacciModule),

    /// Finalize module initialization and initiate the calculations.
    ///
    /// `Init` messages should be sent only by the user of the executor system
    /// (in your solution: the `fib()` function).
    Init { id: Ident, other: Ident },

    /// Initiate the next step of the calculations.
    ///
    /// `idx` is the current index in the sequence.
    /// `num` is the current number of the sequence.
    Message { id: Ident, idx: usize, num: Num },

    /// Indicate the end of calculations.
    Done,
}

/// Run the executor.
pub fn run_executor(rx: Receiver<FibonacciSystemMessage>) -> JoinHandle<()> {
    //unimplemented!();
    let mut fib1 = if let FibonacciSystemMessage::RegisterModule(fib1) = rx.recv().unwrap() { fib1 } else { todo!() };
    let mut fib2 = if let FibonacciSystemMessage::RegisterModule(fib2) = rx.recv().unwrap() { fib2 } else { todo!() };
    thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            //unimplemented!()
            match msg {
                FibonacciSystemMessage::Init{id, other} => {
                    if id == fib1.id {
                        fib1.init(other);
                    }
                    else {
                        fib2.init(other);
                    }
                },

                FibonacciSystemMessage::Message{id, idx, num} => {
                    if id == fib1.id {
                        fib1.message(idx, num);
                    }
                    else {
                        fib2.message(idx, num);
                    }
                },
                FibonacciSystemMessage::Done => break,
                
                _ => panic!("ERROR!!"),
            }
        }
    })
}

/// Calculate the `n`-th Fibonacci number.
pub fn fib(n: usize) {
    // Create the queue and two modules:
    let (tx, rx): (
        Sender<FibonacciSystemMessage>,
        Receiver<FibonacciSystemMessage>,
    ) = unbounded();
    let fib1_id = FibonacciModule::create(0, n, tx.clone());
    let fib2_id = FibonacciModule::create(1, n, tx.clone());

    // Tests will be rerun in case the assertion fails:
    assert_ne!(fib1_id, fib2_id);

    // Initialize the modules by sending `Init` messages:
    //unimplemented!()
    tx.send(FibonacciSystemMessage::Init{id: fib1_id, other: fib2_id}).unwrap();
    tx.send(FibonacciSystemMessage::Init{id: fib2_id, other: fib1_id}).unwrap();

    // Run the executor:
    run_executor(rx).join().unwrap();
}
