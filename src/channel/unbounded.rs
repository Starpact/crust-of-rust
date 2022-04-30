use std::{
    collections::LinkedList,
    sync::{Arc, Condvar, Mutex},
};

pub struct UnboundedSender<T> {
    shared: Arc<Shared<T>>,
}

pub struct UnboundedReceiver<T> {
    shared: Arc<Shared<T>>,
    buffer: LinkedList<T>,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

struct Inner<T> {
    queue: LinkedList<T>,
    nsenders: usize,
}

pub fn unbounded<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>) {
    let shared = Shared {
        inner: Mutex::new(Inner {
            queue: LinkedList::new(),
            nsenders: 1,
        }),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);

    (
        UnboundedSender {
            shared: shared.clone(),
        },
        UnboundedReceiver {
            shared,
            buffer: LinkedList::new(),
        },
    )
}

impl<T> Clone for UnboundedSender<T> {
    fn clone(&self) -> Self {
        self.shared.inner.lock().unwrap().nsenders += 1;
        UnboundedSender {
            shared: self.shared.clone(),
        }
    }
}

impl<T> Drop for UnboundedSender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.nsenders -= 1;
        if inner.nsenders == 0 {
            drop(inner);
            self.shared.available.notify_one();
        }
    }
}

impl<T> Iterator for UnboundedReceiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

impl<T> UnboundedSender<T> {
    pub fn send(&self, t: T) {
        self.shared.inner.lock().unwrap().queue.push_back(t);
        self.shared.available.notify_one();
    }
}

impl<T> UnboundedReceiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let head @ Some(_) = self.buffer.pop_front() {
            return head;
        }

        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut inner.queue);
                    }
                    return Some(t);
                }
                None if inner.nsenders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (tx, mut rx) = unbounded();
        tx.send(66);
        assert_eq!(rx.recv(), Some(66));
    }

    #[test]
    fn close_tx() {
        let (tx, mut rx) = unbounded::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn iter() {
        let (tx, rx) = unbounded();
        for i in 0..10 {
            tx.send(i);
        }

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(1));
            drop(tx);
        });

        for x in rx {
            println!("{}", x);
        }
    }
}
