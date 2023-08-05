use crate::packet::v5::reason_codes::ReasonCode;

pub struct NetworkConnection<T>
where
    T: Read + Write,
{
    io: T,
}

/// Network connection represents an established TCP connection.
impl<T> NetworkConnection<T>
where
    T: Read + Write,
{
    /// Create a new network handle using the provided IO implementation.
    pub fn new(io: T) -> Self {
        Self { io }
    }

    /// Send the data from `buffer` via TCP connection.
    pub fn send(&mut self, buffer: &[u8]) -> Result<(), ReasonCode> {
        let _ = self
            .io
            .write(buffer)
            .map_err(|_| ReasonCode::NetworkError)?;
        Ok(())
    }

    /// Receive data to the `buffer` from TCP connection.
    pub fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, ReasonCode> {
        self.io.read(buffer).map_err(|_| ReasonCode::NetworkError)
    }
}
