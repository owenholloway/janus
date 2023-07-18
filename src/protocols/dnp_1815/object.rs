// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

pub struct ObjectHeader {
    pub group: u8,
    pub variation: u8,
    pub qualifier: u8,
    pub range: Vec<u8>,
}
