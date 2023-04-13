use encoding_rs::{GBK, UTF_8};

pub struct EncodeTool {}

impl EncodeTool {
    pub fn gbk_to_utf8(gbk_bytes: &[u8]) -> String {
        let (cow, _, had_errors) = GBK.decode(gbk_bytes);
        if had_errors {
            panic!("Invalid GBK string!");
        }
        let (output, _, _) = UTF_8.encode(&cow);
        String::from_utf8_lossy(&output).to_string()
    }
}