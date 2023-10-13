use crate::r#const::MAX_MSG;
use std::io::Error;

const FIRST_PART_SIZE: usize = 4;

pub fn message_builder(msg: String) -> Result<String, Error> {
    let msg_len = msg.len();

    if msg_len > MAX_MSG {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Message is too long",
        ));
    }

    let mut msg_len_string = msg_len.to_string();
    if msg_len_string.len() < FIRST_PART_SIZE {
        let diff = FIRST_PART_SIZE - msg_len_string.len();
        for _ in 0..diff {
            msg_len_string.insert(0, '\0');
        }
    }

    let data = format!("{}{}", msg_len_string, msg);
    return Ok(data);
}

pub async fn message_parser(buf: &[u8]) -> (String, String) {
    let cloned_buf = buf;

    let len_data = cloned_buf[..FIRST_PART_SIZE].to_vec();
    let msg_len = match String::from_utf8(len_data) {
        Ok(msg_len) => msg_len.trim_matches('\0').to_string(),
        Err(e) => panic!("Failed to parse message length: {}", e),
    };
    let msg_len_usize = msg_len.trim().parse::<usize>().unwrap();

    let msg_data = cloned_buf[FIRST_PART_SIZE..(FIRST_PART_SIZE + msg_len_usize + 1)].to_vec();
    let text = match String::from_utf8(msg_data) {
        Ok(text) => text,
        Err(e) => panic!("Failed to parse message: {}", e),
    };

    let pattern: &[_] = &['\r', '\n'];
    let clean_text = text.trim_matches(pattern).to_string();

    return (msg_len, clean_text);
}
