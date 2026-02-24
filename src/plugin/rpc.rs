use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::error::Result;

use super::methods;

pub async fn run_plugin_stdio() -> Result<()> {
    let mut reader = BufReader::new(tokio::io::stdin());

    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).await?;
        if bytes_read == 0 {
            break;
        }

        if let Some(response) = methods::dispatch_line(line.trim_end()).await {
            let mut stdout = tokio::io::stdout();
            stdout.write_all(response.as_bytes()).await?;
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;
        }
    }

    Ok(())
}
