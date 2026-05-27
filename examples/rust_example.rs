// tabbislashcat-cat — rust example implementation
// this is pseudo-rust. the real cat is in your dirt.

use std::net::UdpSocket;
use std::collections::HashMap;

// Assuming we have the Token type from tabbislashcat-infant
// use tabbislashcat_infant::Token;

/// A cat vector — a stream with direction
#[derive(Debug)]
pub struct CatVector {
    pub target: u64,
    pub stream_id: u64,
    pub tokens: Vec<Token>,
}

impl CatVector {
    /// Create a new vector
    pub fn new(target: u64, stream_id: u64, tokens: Vec<Token>) -> Self {
        CatVector { target, stream_id, tokens }
    }
    
    /// Encode vector to bytes (wire format)
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.target.to_be_bytes());
        bytes.extend_from_slice(&self.stream_id.to_be_bytes());
        for token in &self.tokens {
            bytes.extend_from_slice(&token.encode());
        }
        bytes
    }
    
    /// Decode vector from bytes
    /// Returns (vector, consumed_bytes) or None on error
    pub fn decode(bytes: &[u8]) -> Option<(Self, usize)> {
        if bytes.len() < 16 {
            return None;
        }
        
        let target = u64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        let stream_id = u64::from_be_bytes([
            bytes[8], bytes[9], bytes[10], bytes[11],
            bytes[12], bytes[13], bytes[14], bytes[15],
        ]);
        
        let mut consumed = 16;
        let mut tokens = Vec::new();
        
        while consumed < bytes.len() {
            match Token::decode(&bytes[consumed..]) {
                Some((token, n)) => {
                    consumed += n;
                    tokens.push(token);
                }
                None => break, // invalid token, stop here
            }
        }
        
        Some((CatVector { target, stream_id, tokens }, consumed))
    }
    
    /// Send vector over UDP socket
    pub fn send_udp(&self, socket: &UdpSocket, addr: &str) -> std::io::Result<()> {
        let bytes = self.encode();
        socket.send_to(&bytes, addr)?;
        // gone. no wait. no mercy.
        Ok(())
    }
    
    /// Receive vector from UDP socket
    pub fn recv_udp(socket: &UdpSocket) -> std::io::Result<Option<Self>> {
        let mut buf = [0u8; 65535];
        let (len, _) = socket.recv_from(&mut buf)?;
        
        if len < 16 {
            return Ok(None); // too short to be a valid vector
        }
        
        match CatVector::decode(&buf[..len]) {
            Some((vector, _)) => Ok(Some(vector)),
            None => Ok(None), // invalid, drop it
        }
    }
}

/// Cat sender — sends vectors and forgets
pub struct CatSender {
    socket: UdpSocket,
}

impl CatSender {
    pub fn new(target_addr: &str) -> std::io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(target_addr)?;
        Ok(CatSender { socket })
    }
    
    pub fn send(&self, vector: CatVector) -> std::io::Result<()> {
        vector.send_udp(&self.socket, &self.socket.peer_addr()?.to_string())?;
        Ok(())
    }
}

/// Cat receiver — receives vectors and delivers to callback
pub struct CatReceiver<T: Fn(CatVector)> {
    socket: UdpSocket,
    callback: T,
}

impl<T: Fn(CatVector)> CatReceiver<T> {
    pub fn new(addr: &str, callback: T) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(addr)?;
        Ok(CatReceiver { socket, callback })
    }
    
    pub fn recv(&self) -> std::io::Result<()> {
        loop {
            match CatVector::recv_udp(&self.socket)? {
                Some(vector) => (self.callback)(vector),
                None => break, // invalid, continue
            }
        }
        Ok(())
    }
}

// Example usage
fn main() -> std::io::Result<()> {
    // Create a sender
    let sender = CatSender::new("127.0.0.1:9999")?;
    
    // Create a vector with a string token
    let tokens = vec![Token::Str("hello from cat".to_string())];
    let vector = CatVector::new(1, 1, tokens);
    
    // Send it
    sender.send(vector)?;
    
    // Create a receiver
    let receiver = CatReceiver::new("127.0.0.1:9999", |v| {
        println!("Received vector: target={}, stream={}, tokens={:?}", 
                 v.target, v.stream_id, v.tokens);
    })?;
    
    // Receive (in real usage, this would be in a separate thread)
    receiver.recv()?;
    
    Ok(())
}

// Note: This is a simplified example. A real implementation would:
// - Use async I/O for better performance
// - Handle multiple transports (UDP, Unix sockets, shared memory)
// - Support connection multiplexing
// - Have better error handling
// - Support broadcast and multicast
// But cat is the arrow. The dirt is yours.
