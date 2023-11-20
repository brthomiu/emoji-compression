use dialoguer::Confirm;

mod gpt;
mod parser;
mod server;

fn main() {
    // Prompt user if they want to parse the emojis
    let confirm_parse = Confirm::new()
        .with_prompt("Parse emojis.txt?")
        .interact()
        .unwrap();

    // Run parser if confirmed
    if confirm_parse {
        parser::parser();
    }

    // Prompt user if they want to upload the parsed emojis to the GPT AI
    let confirm_upload: bool = Confirm::new()
        .with_prompt("Upload emojis to GPT?")
        .interact()
        .unwrap();

    if confirm_upload {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(gpt::gpt());
    }

    // Prompt user if they want to start the HTTP server
    let confirm_server = Confirm::new()
        .with_prompt("Start server?")
        .interact()
        .unwrap();

    if confirm_server {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(server::start_server());
    }
}
