//! The card code in the URL: the birth moment of a card, so a card needs no
//! database and its link can be shared. For example `1996-04-15-22-nu` is
//! 15/04/1996, 22:00, a woman.

use std::{fmt, str::FromStr};

use tuvi_core::{BirthInput, Gender, SolarYear};

/// A birth moment, written as `yyyy-mm-dd-hh-nam` or `yyyy-mm-dd-hh-nu`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CardCode(pub BirthInput);

impl fmt::Display for CardCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let birth = self.0;
        let gender = match birth.gender {
            Gender::Nam => "nam",
            Gender::Nu => "nu",
        };
        write!(
            f,
            "{:04}-{:02}-{:02}-{:02}-{gender}",
            birth.year.0, birth.month, birth.day, birth.hour
        )
    }
}

/// A code that is not in the `yyyy-mm-dd-hh-nam|nu` form.
#[derive(Debug, PartialEq, Eq)]
pub struct BadCode;

impl fmt::Display for BadCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("not a card code")
    }
}

impl std::error::Error for BadCode {}

impl FromStr for CardCode {
    type Err = BadCode;

    /// Reads the form only. The page checks the date with the engine.
    fn from_str(code: &str) -> Result<Self, BadCode> {
        let mut parts = code.split('-');
        let mut number = |len: usize| {
            parts
                .next()
                .filter(|part| part.len() == len && part.bytes().all(|byte| byte.is_ascii_digit()))
                .ok_or(BadCode)
        };
        let year = SolarYear(number(4)?.parse().map_err(|_| BadCode)?);
        let month = number(2)?.parse().map_err(|_| BadCode)?;
        let day = number(2)?.parse().map_err(|_| BadCode)?;
        let hour = number(2)?.parse().map_err(|_| BadCode)?;
        let gender = match parts.next() {
            Some("nam") => Gender::Nam,
            Some("nu") => Gender::Nu,
            _ => return Err(BadCode),
        };
        if parts.next().is_some() {
            return Err(BadCode);
        }
        Ok(Self(BirthInput {
            year,
            month,
            day,
            hour,
            gender,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LAN: BirthInput = BirthInput {
        year: SolarYear(1996),
        month: 4,
        day: 15,
        hour: 7,
        gender: Gender::Nu,
    };

    #[test]
    fn writes_and_reads_back() {
        let code = CardCode(LAN).to_string();
        assert_eq!(code, "1996-04-15-07-nu");
        assert_eq!(code.parse(), Ok(CardCode(LAN)));
    }

    #[test]
    fn rejects_other_forms() {
        for code in [
            "",
            "1996-04-15-07",
            "1996-4-15-07-nu",
            "1996-04-15-07-nu-x",
            "1996-04-15-07-NU",
            "1996-04-15-+7-nu",
            "abcd-04-15-07-nu",
        ] {
            assert_eq!(code.parse::<CardCode>(), Err(BadCode), "{code}");
        }
    }
}
