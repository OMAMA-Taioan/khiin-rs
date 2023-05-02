#[cfg(windows)]
macro_rules! sep {
    () => {
        r"\"
    };
}

#[cfg(not(windows))]
macro_rules! sep {
    () => {
        r"/"
    };
}

include!(concat!(env!("OUT_DIR"), sep!(), "protos", sep!(), "mod.rs"));

pub mod helpers {
    use std::io::Error;
    use std::io::ErrorKind;
    use std::marker::Unpin;

    use futures::io::BufReader;
    use futures::AsyncReadExt;
    use protobuf::Message;
    use protobuf::Result;

    pub trait WriteDelim {
        fn write_u32_delimited_bytes(&self) -> Result<Vec<u8>>;
    }

    impl<T> WriteDelim for T
    where
        T: Message,
    {
        fn write_u32_delimited_bytes(&self) -> Result<Vec<u8>> {
            let message_bytes = self.write_to_bytes().unwrap();
            let message_len = message_bytes.len() as u32;
            let mut bytes = Vec::<u8>::with_capacity(message_bytes.len() + 4);

            bytes.extend_from_slice(&message_len.to_le_bytes());
            bytes.extend_from_slice(&message_bytes);

            Ok(bytes)
        }
    }

    pub async fn parse_u32_delimited_bytes_async<T, R>(
        reader: &mut BufReader<R>,
    ) -> Result<T>
    where
        T: Message,
        R: AsyncReadExt + Unpin,
    {
        let mut size_buf = [0u8; 4];
        reader.read_exact(&mut size_buf).await?;
        let size = u32::from_le_bytes(size_buf) as usize;
        let mut buf = vec![0u8; size];
        reader.read_exact(&mut buf).await?;
        if let Ok(message) = T::parse_from_bytes(&buf) {
            Ok(message)
        } else {
            Err(Error::new(ErrorKind::Other, "Unable to parse protobuf").into())
        }
    }
}
