use url::Url;

fn main() -> std::io::Result<()> {
    let tunnel = ngrok::builder()
        .http()
        .port(8556)
        .executable("./ngrokclinet.exe")
        .run()?;

    let public_url: &Url = tunnel.http()?;

    println!("Tunnel is open at {:?}", public_url.as_str());

    Ok(())
}
