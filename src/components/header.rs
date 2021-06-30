
#[derive(Debug)]
pub struct Header {
    pub    payload: String
        
}

impl Default for Header {

    fn default() -> Self {
        Header {
            payload: "it works!".to_string()
        }
    }
}
