use super::Error;

#[derive(Debug)]
pub struct Mask {
    zeros: u64,
    ones: u64,
}

impl Mask {
    pub fn from_string(mask: &str) -> Result<Self, Error> {
        let zero_bits = mask
            .chars()
            .map(|ch| match ch {
                '0' => Ok(1),
                '1' | 'X' => Ok(0),
                _ => Err("Invalid character in mask"),
            })
            .collect::<Result<Vec<u8>, Error>>()?;

        let one_bits = mask
            .chars()
            .map(|ch| match ch {
                '1' => Ok(1),
                '0' | 'X' => Ok(0),
                _ => Err("Invalid character in mask"),
            })
            .collect::<Result<Vec<u8>, Error>>()?;

        Ok(Self {
            zeros: !Self::to_u64(&zero_bits)?,
            ones: Self::to_u64(&one_bits)?,
        })
    }

    pub fn apply(&self, value: u64) -> u64 {
        (value | self.ones) & self.zeros
    }

    fn to_u64(slice: &[u8]) -> Result<u64, Error> {
        if slice.len() > 64 {
            Err("Cannot convert more than 64 bits to u64")
        } else {
            Ok(slice.iter().fold(0, |acc, &bit| (acc << 1) | bit as u64))
        }
    }
}
