use std::{
    sync::{Arc, Condvar, Mutex},
    thread::{self, JoinHandle},
    time::{self, Duration},
};

extern crate rand;
use rand::prelude::*;

const NUM_ELVES: i32 = 10;
const NUM_REINDEER: i32 = 9;
const ELF_GROUP_MAX_SIZE: i32 = 3;

const RAND_TIME_MIN: u64 = 100;
const RAND_TIME_MAX: u64 = 2000;

/// Initializes the mutexes and condition variables and then starts the threads.
fn main() {
    // An Arc is an atomically reference counted smart pointer. In essence,
    // it allows multiple references to an item across thread boundaries.
    let santa = Arc::new((Mutex::new(false), Condvar::new()));
    let reindeer = Arc::new((Mutex::new(0), Condvar::new()));
    let elves = Arc::new((Mutex::new(0), Condvar::new()));

    let santa_thread = {
        // Cloning an Arc clones the reference to the data and lets the smart
        // pointer know that there is now another reference to the data. That
        // reference count will be decremented when that clone of the Arc is
        // dropped. The data in the Arc will be dropped when the reference count
        // reaches zero.
        let santa = santa.clone();
        let reindeer = reindeer.clone();
        let elves = elves.clone();
        // The threading provided by the standard library uses whatever native
        // threading model is provided by the system. That means that on systems
        // with pthreads, that's what will be used. On Windows, the Windows native
        // threads will be used. The specifics are abstracted over this nice api
        // that spawns a thread and returns a JoinHandle that can then be joined
        // on at a later date. The JoinHandle is generic over the type of the value
        // returned by the function passed to the thread so it would be easy to use
        // threads to "produce" values and "return" them to the parent thread.
        thread::spawn(move || {
            start_santa(santa, reindeer, elves);
        })
    };

    // This could have been done in a for loop rather than mapping an iterator
    // but this way we get the join handles of all the threads in the event
    // that we need them in the future.
    let _: Vec<JoinHandle<()>> = (0..NUM_ELVES)
        .map(|elf_num: i32| {
            let elves = elves.clone();
            let santa = santa.clone();
            thread::spawn(move || {
                start_elf(elf_num, elves, santa);
            })
        })
        .collect();

    let _: Vec<JoinHandle<()>> = (0..NUM_REINDEER)
        .map(|reindeer_num: i32| {
            let reindeer = reindeer.clone();
            let santa = santa.clone();
            thread::spawn(move || {
                start_reindeer(reindeer_num, reindeer, santa);
            })
        })
        .collect();

    santa_thread.join().unwrap();
}

/// Generates a duration of a random time specified by the constants.
fn rand_time() -> Duration {
    let millis = thread_rng().gen_range(RAND_TIME_MIN..RAND_TIME_MAX);
    time::Duration::from_millis(millis)
}

/// Contains the logic for the Santa thread.
fn start_santa(
    santa: Arc<(Mutex<bool>, Condvar)>,
    reindeer: Arc<(Mutex<i32>, Condvar)>,
    elves: Arc<(Mutex<i32>, Condvar)>,
) {
    // Many smart pointers in Rust implement the `Deref` trait which allows structs
    // to be dereferenced as if they were a normal pointer. The following three lines
    // are dereferencing the Arcs to obtain references to the data inside.
    let (santa_mutex, santa_cvar) = &*santa;
    let (warming_shed_mutex, reindeer_cdvar) = &*reindeer;
    let (elf_waiting_mutex, elf_cvar) = &*elves;

    let mut santa_guard = santa_mutex.lock().unwrap();
    println!("Santa created!");
    loop {
        println!("Santa is sleeping");
        santa_guard = santa_cvar.wait(santa_guard).unwrap();
        println!("Santa has been woken up!");

        let mut num_ready_reindeer = warming_shed_mutex.lock().unwrap();

        let reindeer_ready = *num_ready_reindeer == NUM_REINDEER;

        if reindeer_ready {
            let duration = rand_time();
            println!(
                "Santa and his reindeer are off to deliver presents for {} milliseconds.",
                duration.as_millis()
            );

            thread::sleep(duration);
            println!("Santa and his reindeer have returned from delivering presents.");

            *num_ready_reindeer = 0;

            reindeer_cdvar.notify_all();
        }

        let mut num_waiting_elves = elf_waiting_mutex.lock().unwrap();
        let elves_need_help = *num_waiting_elves == ELF_GROUP_MAX_SIZE;
        if elves_need_help {
            let duration = rand_time();
            println!(
                "Santa is helping a group of {} elves for {} milliseconds.",
                *num_waiting_elves,
                duration.as_millis()
            );
            thread::sleep(duration);
            println!("Santa has finished helping a group of elves.");

            *num_waiting_elves = 0;
            elf_cvar.notify_all();
        }
    }
}

/// Contains the logic for an Elf thread.
fn start_elf(elf_num: i32, elves: Arc<(Mutex<i32>, Condvar)>, santa: Arc<(Mutex<bool>, Condvar)>) {
    println!("Elf {} created!", elf_num);
    let (elf_waiting_mutex, elf_cvar) = &*elves;
    let (_, santa_cvar) = &*santa;

    loop {
        println!("Elf {} is making toys.", elf_num);
        thread::sleep(rand_time());
        println!("Elf {} needs help!", elf_num);

        let mut num_waiting_elves = elf_waiting_mutex.lock().unwrap();

        while *num_waiting_elves == ELF_GROUP_MAX_SIZE {
            num_waiting_elves = elf_cvar.wait(num_waiting_elves).unwrap();
        }

        *num_waiting_elves += 1;

        println!(
            "Elf {} is joining the group currently waiting for help. Number of elves in group: {}.",
            elf_num, *num_waiting_elves
        );

        if *num_waiting_elves == ELF_GROUP_MAX_SIZE {
            println!("Elf {} is waking Santa.", elf_num);
            santa_cvar.notify_one();
        }

        // Once the wait is over, we are done with the mutex guard so it can be dropped to unlock the mutex.
        let _ = elf_cvar.wait(num_waiting_elves).unwrap();
        println!("Elf {} has gotten help!", elf_num);
    }
}

/// Contains the logic for a Reindeer thread.
fn start_reindeer(
    reindeer_num: i32,
    reindeer: Arc<(Mutex<i32>, Condvar)>,
    santa: Arc<(Mutex<bool>, Condvar)>,
) {
    println!("Reindeer {} created!", reindeer_num);
    let (warming_shed_mutex, reindeer_cvar) = &*reindeer;
    let (_, santa_cvar) = &*santa;

    loop {
        let duration = rand_time();
        println!(
            "Reindeer {} is going on vacation for {} milliseconds",
            reindeer_num,
            duration.as_millis()
        );

        thread::sleep(duration);

        let mut num_ready_reindeer = warming_shed_mutex.lock().unwrap();

        *num_ready_reindeer += 1;

        println!(
            "Reindeer {} has returned from vacation. Number of reindeer ready: {}.",
            reindeer_num, *num_ready_reindeer
        );

        if *num_ready_reindeer == NUM_REINDEER {
            println!("Reindeer {} is waking Santa.", reindeer_num);
            santa_cvar.notify_one();
        }

        // Once the wait is over, we are done with the mutex guard so it can be dropped to unlock the mutex.
        let _ = reindeer_cvar.wait(num_ready_reindeer);
    }
}
