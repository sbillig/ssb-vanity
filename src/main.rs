use ssb_crypto::{AsBytes, Keypair};
use std::sync::mpsc;
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
    let (tx, rx) = mpsc::channel();
    for _ in 0..args.threads {
        let tx = tx.clone();
        let prefix = prefix.clone();
        std::thread::spawn(move || match_b64_prefix(&prefix, tx));
    }

    loop {
        let (is_exact, keypair) = rx.recv().unwrap();
        println!("{} {}", keypair.id_string(), keypair.secret_string());

        if is_exact {
            return;
        }
    }
}

fn match_b64_prefix(prefix: &str, sender: mpsc::Sender<(bool, Keypair)>) -> Keypair {
    let chars = prefix.len();
    let bytes = (prefix.len() as f64 * 3.0 / 4.0).ceil() as usize;

    let prefix_lower = prefix.to_ascii_lowercase();
    let mut buf = [0; 44];

    loop {
        let key = Keypair::generate();
        base64::encode_config_slice(&key.public.0[..bytes], base64::STANDARD, &mut buf);

        if &buf[..chars] == prefix.as_bytes() {
            sender.send((true, key)).unwrap()
        } else {
            buf[..chars].make_ascii_lowercase();
            if &buf[..chars] == prefix_lower.as_bytes() {
                sender.send((false, key)).unwrap()
            }
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
