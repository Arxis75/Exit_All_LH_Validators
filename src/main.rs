use std::process::Command;
use std::str;
use std::env;

fn main() {
    let home_env = "HOME";
    match env::var(home_env) {
        Err(e) => println!("couldn't interpret {}: {}", home_env, e),
        Ok(home_path) => {

            let testnet = "medalla";
            let lighthouse_app_path = home_path.to_owned()+&"/lighthouse/target/release/lighthouse";
            let validators_path = home_path.to_owned()+&"/.lighthouse/"+testnet+"/validators/";

            let output = Command::new("ls")
                        .arg(validators_path.to_owned())
                        .output()
                        .expect("failed to list validators");

            let folders = str::from_utf8(&output.stdout).unwrap();

            for f in folders.lines().filter(|f| (f.len() > 2)
                                             && (f[0..2].eq(&"0x".to_string()))) {

                let output = Command::new("ls")
                        .arg(validators_path.to_owned()+f+"/")
                        .output()
                        .expect("failed to list validator keys");

                let key = str::from_utf8(&output.stdout).unwrap();

                for k in key.lines().filter(|k| k[0..22].eq(&"keystore-m_12381_3600_".to_string()) 
                                            && k.ends_with(".json")) {

                    let file_path = String::from(validators_path.to_owned()+f+"/"+k);

                    Command::new(lighthouse_app_path.to_owned())
                        .args(&["account", "validator", "exit"])
                        .args(&["--testnet", testnet])
                        .args(&["--keystore", &file_path])
                        .args(&["--beacon-node", "http://localhost:5052"])
                        .status()
                        .expect(&("failed exit validator ".to_owned()+k));
                }
            }
        }
    }
}
