use crate::{
    reader_writer::{BitReader, BitWriter},
    error::DeErr,
    traits::{De, Ser},
};

impl<T> Ser for Option<T>
where
    T: Ser,
{
    fn ser(&self, writer: &mut BitWriter) {
        if let Some(value) = self {
            writer.write_bit(true);
            value.ser(writer);
        } else {
            writer.write_bit(false);
        }
    }
}

impl<T> De for Option<T>
where
    T: De,
{
    fn de(reader: &mut BitReader) -> Result<Option<T>, DeErr> {
        if reader.read_bit() {
            Ok(Some(De::de(reader)?))
        } else {
            Ok(None)
        }
    }
}

// Tests

#[cfg(test)]
mod tests {
    use crate::{BitReader, BitWriter};

    #[test]
    fn read_write() {
        // Write
        let mut writer = BitWriter::new();

        let in_1 = Some(123);
        let in_2: Option<f32> = None;

        writer.write(&in_1);
        writer.write(&in_2);

        let (buffer_length, buffer) = writer.flush();

        // Read

        let mut reader = BitReader::new(buffer_length, buffer);

        let out_1 = reader.read().unwrap();
        let out_2 = reader.read().unwrap();

        assert_eq!(in_1, out_1);
        assert_eq!(in_2, out_2);
    }
}
