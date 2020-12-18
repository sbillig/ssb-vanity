use ssb_crypto::{AsBytes, Keypair};
use std::io::{self, Write};
use std::sync::mpsc;
use std::time::Instant;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "ssb-vanity", version = "0.1")]
pub struct Args {
    #[structopt(long, short)]
    prefix: String,

    #[structopt(long, short, default_value = "1")]
    threads: usize,
}

fn main() {
    let args = Args::from_args();
    if args.threads == 0 {
        eprintln!("--threads can't be 0");
        std::process::exit(1);
    }

    let prefix = args.prefix;

    for c in prefix.as_bytes() {
        if !(c.is_ascii_alphanumeric() || *c == b'/' || *c == b'+') {
            eprintln!("Invalid prefix character: {}", *c as char);
            std::process::exit(1);
        }
    }

    let (tx, rx) = mpsc::channel();
    let start = Instant::now();
    let mut last_print = start;

    for _ in 0..args.threads {
        let tx = tx.clone();
        let prefix = prefix.clone();
        std::thread::spawn(move || match_b64_prefix(&prefix, tx));
    }

    let mut count = 0u64;

    loop {
        use Update::*;
        match rx.recv().unwrap() {
            ExactMatch(kp) => {
                println!("\r{} {}", kp.id_string(), kp.secret_string());
                return;
            }
            CloseMatch(kp) => {
                println!("\r{} {}", kp.id_string(), kp.secret_string());
            }
            Stats(n) => {
                count += n;
                if last_print.elapsed().as_secs() >= 1 {
                    let secs = start.elapsed().as_secs();
                    print!(
                        "\relapsed: {}s; keys generated: {}; per second: {}",
                        secs,
                        count,
                        count / secs
                    );
                    io::stdout().flush().unwrap();
                    last_print = Instant::now();
                }
            }
        }
    }
}

enum Update {
    ExactMatch(Keypair),
    CloseMatch(Keypair),
    Stats(u64),
}

fn match_b64_prefix(prefix: &str, sender: mpsc::Sender<Update>) -> Keypair {
    let chars = prefix.len();
    let bytes = (prefix.len() as f64 * 3.0 / 4.0).ceil() as usize;

    let prefix_lower = prefix.to_ascii_lowercase();
    let mut buf = [0; 44];

    let mut count = 0u64;

    loop {
        let key = Keypair::generate();
        base64::encode_config_slice(&key.public.0[..bytes], base64::STANDARD, &mut buf);

        if &buf[..chars] == prefix.as_bytes() {
            sender.send(Update::ExactMatch(key)).unwrap()
        } else {
            buf[..chars].make_ascii_lowercase();
            if &buf[..chars] == prefix_lower.as_bytes() {
                sender.send(Update::CloseMatch(key)).unwrap()
            }
        }
        const N: u64 = 10_000;
        count += 1;
        if count % N == 0 {
            sender.send(Update::Stats(N)).unwrap();
        }
    }
}

trait KeypairExt {
    fn id_string(&self) -> String;
    fn secret_string(&self) -> String;
}

impl KeypairExt for Keypair {
    fn id_string(&self) -> String {
        let mut id = String::with_capacity(53);
        id.push('@');
        base64::encode_config_buf(&self.public.as_bytes(), base64::STANDARD, &mut id);
        id.push_str(".ed25519");
        id
    }

    fn secret_string(&self) -> String {
        let mut s = String::with_capacity(88);
        base64::encode_config_buf(&self.as_bytes(), base64::STANDARD, &mut s);
        s
    }
}
