use std::sync::mpsc::{RecvError, SendError};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum CounterAction {
    Increment,
    Decrement,
    Terminate,
}

pub struct CountedChannel<T> {
    sender: mpsc::Sender<T>,
    receiver: mpsc::Receiver<T>,
    counter_tx: mpsc::Sender<CounterAction>,
    size: Arc<Mutex<usize>>,
}

impl<T> CountedChannel<T> {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let (counter_tx, counter_rx) = mpsc::channel();
        let size = Arc::new(Mutex::new(0));

        let size_clone = size.clone();
        thread::spawn(move || {
            while let Ok(action) = counter_rx.recv() {
                match action {
                    CounterAction::Increment => {
                        let mut size = size_clone.lock().unwrap();
                        *size += 1;
                    }
                    CounterAction::Decrement => {
                        let mut size = size_clone.lock().unwrap();
                        *size -= 1;
                    }
                    CounterAction::Terminate => break,
                }
            }
        });

        CountedChannel {
            sender,
            receiver,
            counter_tx,
            size,
        }
    }

    pub fn send(&self, t: T) -> Result<(), SendError<T>> {
        let result = self.sender.send(t);
        if result.is_ok() {
            self.counter_tx.send(CounterAction::Increment).unwrap();
        }
        result
    }

    pub fn recv(&self) -> Result<T, RecvError> {
        let result = self.receiver.recv();
        if result.is_ok() {
            self.counter_tx.send(CounterAction::Decrement).unwrap();
        }
        result
    }

    pub fn len(&self) -> usize {
        *self.size.lock().unwrap()
    }
}

impl<T> Drop for CountedChannel<T> {
    fn drop(&mut self) {
        self.counter_tx.send(CounterAction::Terminate).unwrap();
    }
}
