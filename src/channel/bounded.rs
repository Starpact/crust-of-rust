use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Condvar, Mutex,
    },
};

pub struct BoundedSender<T> {
    shared: Arc<Shared<T>>,
}

pub struct BoundedReceiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    nmessages: AtomicUsize,
    send_waker: Condvar,
    recv_waker: Condvar,
}

struct Inner<T> {
    queue: VecDeque<T>,
    nsenders: usize,
}

pub fn bounded<T>(capacity: usize) -> (BoundedSender<T>, BoundedReceiver<T>) {
    let shared = Shared {
        inner: Mutex::new(Inner {
            queue: VecDeque::with_capacity(capacity),
            nsenders: 1,
        }),
        nmessages: AtomicUsize::new(0),
        send_waker: Condvar::new(),
        recv_waker: Condvar::new(),
    };
    let shared = Arc::new(shared);

    (
        BoundedSender {
            shared: shared.clone(),
        },
        BoundedReceiver {
            shared,
            buffer: VecDeque::with_capacity(capacity),
        },
    )
}

impl<T> Clone for BoundedSender<T> {
    fn clone(&self) -> Self {
        self.shared.inner.lock().unwrap().nsenders += 1;
        BoundedSender {
            shared: self.shared.clone(),
        }
    }
}

impl<T> Drop for BoundedSender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.nsenders -= 1;
        if inner.nsenders == 0 {
            drop(inner);
            self.shared.recv_waker.notify_one();
        }
    }
}

impl<T> Iterator for BoundedReceiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

impl<T> BoundedSender<T> {
    pub fn send(&self, v: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        if self.shared.nmessages.load(Ordering::SeqCst) == inner.queue.capacity() {
            inner = self.shared.send_waker.wait(inner).unwrap();
        }

        inner.queue.push_back(v);
        self.shared.nmessages.fetch_add(1, Ordering::SeqCst);
        drop(inner);
        self.shared.recv_waker.notify_one();
    }
}

impl<T> BoundedReceiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let head @ Some(_) = self.buffer.pop_front() {
            self.shared.send_waker.notify_one();
            self.shared.nmessages.fetch_sub(1, Ordering::SeqCst);
            return head;
        }

        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => {
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut inner.queue);
                    }
                    self.shared.nmessages.fetch_sub(1, Ordering::SeqCst);
                    drop(inner);
                    self.shared.send_waker.notify_one();
                    return Some(t);
                }
                None if inner.nsenders == 0 => return None,
                None => {
                    inner = self.shared.recv_waker.wait(inner).unwrap();
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
        let (tx, mut rx) = bounded(1);
        tx.send(66);
        assert_eq!(rx.recv(), Some(66));
    }

    #[test]
    fn close_tx() {
        let (tx, mut rx) = bounded::<()>(1);
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn iter() {
        let (tx, rx) = bounded(3);

        for i in 0..3 {
            let tx = tx.clone();
            std::thread::spawn(move || {
                tx.send(i);
                println!("{} sended", i);
            });
        }
        drop(tx);

        for x in rx {
            println!("{} received", x);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
