use std::io::{BufReader, BufWriter, Read, Write};

use serde::Deserialize;
use serde_json::Deserializer;

use jabcoin::network::Message;

pub struct Connection<W: Write, R: Read>
{
    reader: BufReader<R>,
    writer: BufWriter<W>,
}

impl<W: Write, R: Read> Connection<W, R>
{
    pub fn new(writer: W, reader: R) -> Connection<W, R>
    {
        let reader = BufReader::with_capacity(4096, reader);
        let writer = BufWriter::with_capacity(4096, writer);

        Connection { reader, writer }
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
