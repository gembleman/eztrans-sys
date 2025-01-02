use eztrans_sys::{EzTransError, EzTransLib};

use windows_shared_memory::{Client, RecieveMessage, Server};

fn main() -> Result<(), EzTransError> {
    let ez_trans = EzTransLib::new(None)?;

    // Initialize EzTransLib
    ez_trans.initialize(None, None)?;

    let server = Server::new(None).map_err(|e| EzTransError::SharedMemoryError(e.to_string()))?;
    let _ = server.send_s2c("おはようございます。".as_bytes());
    let client = Client::new(None).map_err(|e| EzTransError::SharedMemoryError(e.to_string()))?;

    loop {
        let recieve_server = client.recv_s2c(true);

        if let RecieveMessage::Message(recv_message) = recieve_server {
            match ez_trans.translate(&recv_message) {
                Ok(translated) => {
                    client
                        .send_c2s(&translated.as_bytes())
                        .map_err(|e| EzTransError::SharedMemoryError(e.to_string()))?;
                }
                Err(error) => {
                    client
                        .send_c2s(&format!("Translation error: {}", &error).as_bytes())
                        .map_err(|e| EzTransError::SharedMemoryError(e.to_string()))?;
                }
            }
        } else if let RecieveMessage::Exit = recieve_server {
            break;
        } else if let RecieveMessage::Error = recieve_server {
            break;
        }
    }

    // Terminate EzTransLib
    ez_trans.terminate()?;

    Ok(())
}

//おはようございます。 example
//こんにちは。 example
//さようなら。 example

//おはようございます。
//こんにちは。
//さようなら。
//exit
//Exiting...
