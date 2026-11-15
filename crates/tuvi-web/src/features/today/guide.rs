//! The daily guide: what each cung, each Hóa and each hour god means for a
//! day, from `data/daily_guide.json`. The app reads it once at start-up and
//! keeps it in the context.

use std::collections::HashMap;

use serde::Deserialize;
use topcoat::context::{Cx, context};

use crate::i18n::Text;

/// The file, built into the binary.
const JSON: &str = include_str!("../../../../../data/daily_guide.json");

/// The 12 cung roles, as `tuvi_core` writes them.
pub(super) const ROLES: [&str; 12] = [
    "Mệnh",
    "Phụ Mẫu",
    "Phúc Đức",
    "Điền Trạch",
    "Quan Lộc",
    "Nô Bộc",
    "Thiên Di",
    "Tật Ách",
    "Tài Bạch",
    "Tử Tức",
    "Phu Thê",
    "Huynh Đệ",
];

/// The 4 Hóa, as `tuvi_core` writes them.
pub(super) const HOA: [&str; 4] = ["Hóa Lộc", "Hóa Quyền", "Hóa Khoa", "Hóa Kỵ"];

/// The 12 hour gods, as `tuvi_core` writes them, and whether each one is
/// hoàng đạo.
const GODS: [(&str, bool); 12] = [
    ("Thanh Long", true),
    ("Minh Đường", true),
    ("Thiên Hình", false),
    ("Chu Tước", false),
    ("Kim Quỹ", true),
    ("Bảo Quang", true),
    ("Bạch Hổ", false),
    ("Ngọc Đường", true),
    ("Thiên Lao", false),
    ("Huyền Vũ", false),
    ("Tư Mệnh", true),
    ("Câu Trận", false),
];

#[derive(Debug, Deserialize)]
pub struct Guide {
    pub sources: Vec<Source>,
    palaces: HashMap<String, Palace>,
    transformations: HashMap<String, Transformation>,
    hours: HashMap<String, Hour>,
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub title: String,
    pub url: String,
}

/// A cung role: its name and what a day or an hour on it is good for.
#[derive(Debug, Deserialize)]
pub struct Palace {
    pub name: Text,
    pub theme: Text,
    #[serde(rename = "do")]
    pub good: Text,
    pub avoid: Text,
}

/// A Hóa: its name, its meaning, and a line for each cung it can fall in.
#[derive(Debug, Deserialize)]
pub struct Transformation {
    pub name: Text,
    pub meaning: Text,
    palaces: HashMap<String, Text>,
}

impl Transformation {
    /// The line for the Hóa in `role`.
    pub fn in_palace(&self, role: &str) -> Option<&Text> {
        self.palaces.get(role)
    }
}

/// An hour god.
#[derive(Debug, Deserialize)]
pub struct Hour {
    pub name: Text,
    pub auspicious: bool,
    #[serde(rename = "do")]
    pub good: Text,
    pub avoid: Text,
}

impl Guide {
    /// Parses the built-in file and checks that it covers every cung, Hóa and
    /// hour god, so a page never meets a missing text, and that it agrees
    /// with the engine on which hours are hoàng đạo.
    pub fn load() -> Result<Self, String> {
        let guide: Self = serde_json::from_str(JSON).map_err(|error| error.to_string())?;
        for role in ROLES {
            if !guide.palaces.contains_key(role) {
                return Err(format!("palaces: no \"{role}\""));
            }
        }
        for hoa in HOA {
            let Some(transformation) = guide.transformations.get(hoa) else {
                return Err(format!("transformations: no \"{hoa}\""));
            };
            for role in ROLES {
                if transformation.in_palace(role).is_none() {
                    return Err(format!("transformations.{hoa}.palaces: no \"{role}\""));
                }
            }
        }
        for (god, hoang_dao) in GODS {
            match guide.hours.get(god) {
                None => return Err(format!("hours: no \"{god}\"")),
                Some(hour) if hour.auspicious != hoang_dao => {
                    return Err(format!("hours.{god}.auspicious should be {hoang_dao}"));
                }
                Some(_) => {}
            }
        }
        Ok(guide)
    }

    /// The guide in the context.
    pub fn of(cx: &Cx) -> &Self {
        context::<Self>(cx)
    }

    /// `None` only for a name that [`Guide::load`] did not check.
    pub fn palace(&self, role: &str) -> Option<&Palace> {
        self.palaces.get(role)
    }

    pub fn transformation(&self, hoa: &str) -> Option<&Transformation> {
        self.transformations.get(hoa)
    }

    pub fn hour(&self, god: &str) -> Option<&Hour> {
        self.hours.get(god)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_file_covers_everything() {
        let guide = Guide::load().unwrap();
        assert!(!guide.sources.is_empty());
        for role in ROLES {
            let palace = guide.palace(role).unwrap();
            assert!(
                !palace.good.en.is_empty() && !palace.good.vi.is_empty(),
                "{role}"
            );
        }
    }
}
