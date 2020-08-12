#[derive(Debug)]
pub struct StatusCode {
    pub code: u16,
    pub msg: String,
}

impl StatusCode {
    pub fn compile(self) -> String {
        let mut prefix = self.code.to_string();
        prefix.push_str(" ");
        prefix.push_str(&self.msg);
        prefix
    }
}

pub fn get_status_code(code: u16) -> StatusCode {
    match get_status_msg(code) {
        Ok(msg) => {
            return StatusCode {
                code: code,
                msg: msg,
            }
        }
        Err(msg) => {
            return StatusCode {
                code: 500,
                msg: msg,
            }
        }
    }
}

fn get_status_msg(code: u16) -> Result<String, String> {
    match code {
        200 => Ok("OK".to_string()),
        400 => Ok("Bad Request".to_string()),
        404 => Ok("Not Found".to_string()),
        _ => Err("Internal Server Error".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_status_msg_test() {
        let msg1 = unwrap_msg(get_status_msg(200));
        let msg2 = unwrap_msg(get_status_msg(400));
        let msg3 = unwrap_msg(get_status_msg(404));
        let msg4 = unwrap_msg(get_status_msg(600));
        assert_eq!(msg1, "OK");
        assert_eq!(msg2, "Bad Request");
        assert_eq!(msg3, "Not Found");
        assert_eq!(msg4, "Internal Server Error");
    }

    fn unwrap_msg(res: Result<String, String>) -> String {
        match res {
            Ok(msg) => return msg,
            Err(msg) => return msg,
        }
    }
}
