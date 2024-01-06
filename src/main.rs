use std::{env::args, fs, io::Read, sync::mpsc, thread};

fn main() {
    let command: Vec<String> = args().map(String::from).collect();
    let command_size = command.len();
    if command_size < 2 {
        println!("provide atleast a file")
    }

    let file_names: Vec<String> = command[1..].to_vec();

    let iterations: usize = match (command_size - 1) % 4 {
        0 => (command_size - 1) / 4,
        _ => ((command_size - 1) / 4) + 1,
    };

    let (tx, rx) = mpsc::channel();
    file_names.iter().for_each(|file_name| {
        tx.send(fs::File::open(file_name.as_str()).unwrap())
            .expect("failed to send message");
    });

    let mut i = command_size - 1;
    (0..iterations).into_iter().for_each(|_| {
        let thread_iteration = match i {
            x if x >= 4 => 4,
            _ => i,
        };
        let threads: Vec<thread::JoinHandle<()>> =
            (0..thread_iteration)
                .into_iter()
                .fold(vec![], |mut acc, _| {
                    let mut file = rx.recv().unwrap();

                    let t = thread::spawn(move || {
                        let mut buf = String::new();
                        file.read_to_string(&mut buf).unwrap();
                        println!("chars: {}", buf.chars().count());
                    });

                    i -= 1;
                    acc.push(t);
                    acc
                });

        for t in threads {
            t.join().unwrap();
        }
    });
}
