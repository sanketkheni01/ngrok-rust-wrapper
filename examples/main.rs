use core::time;
use std::thread;

use url::Url;

fn main() -> Result<(), std::io::Error> {
    let keep_tunnel = true;
    while keep_tunnel {
        start_tunnel();
    }
    Ok(())
}

fn start_tunnel() -> Result<(), std::io::Error> {
    let tunnel = ngrok_wrapper::builder()
        .http()
        .port(8556)
        .executable("./resources/ngrok.exe")
        .run()
        .unwrap();

    let public_url: &Url = tunnel.http().unwrap();
    println!("Tunnel is open at {:?}", public_url.as_str());

    // This will wait for the tunnel to close
    let res = tunnel.wait()?;
    Ok(())
}
