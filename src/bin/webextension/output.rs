use byteorder::{NativeEndian, WriteBytesExt};
use serde::Serialize;
use std::io;
use std::io::{StdoutLock, Write};

#[derive(Serialize)]
pub(crate) enum Status {
    Success,
    Error,
}

#[derive(Serialize)]
pub(crate) struct Output {
    pub(crate) status: Status,
    pub(crate) data: String,
}

impl Output {
    pub(crate) fn write_to(&self, output: &mut StdoutLock) -> io::Result<()> {
        let message = serde_json::to_vec(self)?;
        output.write_u32::<NativeEndian>(message.len() as u32)?;
        output.write_all(&message)?;
        output.flush()
    }
}
