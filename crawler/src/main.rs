use std::boxed::Box;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

fn example() -> usize {
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let _tx = thread::spawn(move || {
        x.store(true, Ordering::Release);
    });
    let _ty = thread::spawn(move || {
        y.store(true, Ordering::Release);
    });
    let t1 = thread::spawn(move || {
        while !x.load(Ordering::Acquire) {}
        if y.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    let t2 = thread::spawn(move || {
        while !y.load(Ordering::Acquire) {}
        if x.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
    let z = z.load(Ordering::SeqCst);
    z
}

fn main() {
    for i in 0..1000 {
        let z = example();
        dbg!(z);
    }
}