use std::{env::args, fs, io::Read, sync::mpsc, thread};

fn main() {
    let command: Vec<String> = args().map(String::from).collect();
    let command_size = command.len();
    if command_size < 2 {
        println!("provide atleast a file")
    }

    let file_names: Vec<String> = command[1..].to_vec();

    let (tx, rx) = mpsc::channel();
    file_names.iter().for_each(|file_name| {
        tx.send(file_name.clone()).expect("failed to send message");
    });

    let threads = (0..(command_size - 1))
        .into_iter()
        .fold(vec![], |mut acc, _| {
            let file_name = rx.recv().unwrap();
            let t = thread::spawn(move || {
                let mut file = fs::File::open(file_name.as_str()).unwrap();
                let mut buf = String::new();
                file.read_to_string(&mut buf).unwrap();
                println!("chars: {}", buf.chars().count())
            });

            acc.push(t);
            acc
        });

    for t in threads {
        t.join().unwrap();
    }
}
