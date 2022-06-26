use serde::Deserialize;
use serde_json::Deserializer;

use std::io::{BufReader, BufWriter, Write};
use std::net::TcpStream;

use jabcoin::network::Message;

pub struct Connection
{
    stream: TcpStream,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl Connection
{
    pub fn new(stream: TcpStream) -> Connection
    {
        let reader = BufReader::new(stream.try_clone().unwrap());
        let writer = BufWriter::new(stream.try_clone().unwrap());

        Connection {
            stream,
            reader,
            writer,
        }
    }

    pub fn get_stream(&self) -> &TcpStream
    {
        &self.stream
    }

    pub fn write_msg(&mut self, msg: Message) -> serde_json::Result<()>
    {
        serde_json::to_writer(&mut self.writer, &msg)?;
        self.writer.flush().unwrap();
        Ok(())
    }

    pub fn read_msg(&mut self) -> serde_json::Result<Message>
    {
        let mut de = Deserializer::from_reader(&mut self.reader);
        let msg = Message::deserialize(&mut de)?;

        Ok(msg)
    }
}
