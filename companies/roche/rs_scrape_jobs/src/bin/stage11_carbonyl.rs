use tokio::process::Command;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting carbonyl with https://text.npr.org and simulating mouse clicks on links");

    let mut child = Command::new("/usr/bin/carbonyl")
        .arg("https://text.npr.org")
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    let stdin = child.stdin.as_mut().unwrap();

    // Wait for the page to load
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Simulate left click at column 5, row 15 (example coordinates for a link)
    stdin.write_all(b"\x1b[<0;5;15M\x1b[<0;5;15m").await?;
    println!("Sent mouse click at (5,15)");

    // Wait a bit
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Simulate another click at column 10, row 20
    stdin.write_all(b"\x1b[<0;10;20M\x1b[<0;10;20m").await?;
    println!("Sent mouse click at (10,20)");

    // Keep the process running
    let status = child.wait().await?;
    println!("Carbonyl exited with status: {:?}", status);

    Ok(())
}
