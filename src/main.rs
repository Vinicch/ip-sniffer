use std::{env, process, sync::mpsc, thread};

use ip_sniffer::{scan, Arguments};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{program} problem parsing arguments: {err}");
            process::exit(0);
        }
    });

    let num_threads = arguments.threads;
    let addr = arguments.ip_addr;
    let (tx, rx) = mpsc::channel();

    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);

    for p in rx {
        out.push(p);
    }

    println!();
    out.sort();

    for v in out {
        println!("{v} is open");
    }
}
