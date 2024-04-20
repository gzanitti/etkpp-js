mod utils;

use etk_asm::ingest::Ingest;
use etk_ops::HardFork;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn run(input: &str) {
    let mut output = Vec::new();
    let mut ingest = Ingest::new(&mut output, HardFork::Cancun);
    match ingest.ingest_str(input) {
        Ok(_) => {
            let output_str = output
                .iter()
                .map(|opcode| format!("{:02x}", opcode))
                .collect::<String>();

            alert(&format!("Success! Output: {}", output_str));
        }
        Err(e) => {
            alert(&format!("Error: {}", e));
            return;
        }
    }
}
