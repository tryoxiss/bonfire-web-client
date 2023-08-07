pub fn send_message(content: &str, content_type: &str)
{
    let post_request = RequestPacket::new(content);
    print!("{:?}", post_request); // This does not work yet, must use some other way to print to terminal or console in browser
}

fn create_header_variables() -> Vec<HeaderVariable>
{
    let mut header_variables: Vec<HeaderVariable> = vec![];

    let header_length: usize = 0;
    for variable in 0..header_length
    {
        header_variables.push(HeaderVariable::new("key", "value"));
    }


    // Use Aes-Gcm-Siv for Client-to-Server (aka not Peer-to-Peer)
    // ---
    // We can use a CSPRNG or Crypographically Secure Psudo-Random Number
    // Generator for encryption values. We will use ChaCha20-poly1305 since
    // - It can produce 1.8gb of randomness every seccond, making it far from
    //   a bottleneck.
    // - initalises fast (but startup times are not very important)
    // - only uses 136 bytes of perpetual memory 
    // - has been [deeply analyised](ChaCha20Analysis) 
    // 
    // [ChaCha20Analysis]: https://datatracker.ietf.org/doc/html/rfc7539#section-1
    // (Same as Above)   : https://www.cryptrec.go.jp/exreport/cryptrec-ex-2601-2016.pdf
    // (Summary)         : https://en.wikipedia.org/wiki/ChaCha20-Poly1305
    // ---

    header_variables.push(HeaderVariable::new("encyption", "aes"));
    header_variables.push(HeaderVariable::new("force_encryption", "t"));
    header_variables.push(HeaderVariable::new("author", "8d1a0cfb13df4ca3bdb0e912be01863b"));
    header_variables.push(HeaderVariable::new("target", "none"));
    header_variables.push(HeaderVariable::new("channel", "20026f0a1c484f95a0063d148c8898f9"));
    header_variables.push(HeaderVariable::new("channel_type", "text_message"));
    header_variables.push(HeaderVariable::new("content_mime_type", "text/plain"));
    header_variables.push(HeaderVariable::new("content_formatting", "none"));
    header_variables.push(HeaderVariable::new("time_sent", "2023-06-25 12:25:22"));

    return header_variables;
}

#[derive(Debug)]
struct HeaderVariable
{
    key: String,
    value: String,
}

impl HeaderVariable
{
    fn new(key: &str, value: &str) -> HeaderVariable
    {
        HeaderVariable
        {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug)]
enum RequestMethod
{
    Get,
    Edit,
    Post,
    Remove,
}

#[derive(Debug)]
struct RequestPacket
{
    version: String,
    method: RequestMethod,
    header_flags: Vec<HeaderVariable>,
    resource: String,

    body: String,
}

impl RequestPacket
{
    fn new(content: &str) -> RequestPacket
    {
        let compliance_vars = create_header_variables();

        return RequestPacket
        {
            version: "1.0".to_string(),
            method: RequestMethod::Post,
            header_flags: compliance_vars,
            resource: "hiss".to_string(),
            body: content.to_string()
        }
    }
}
