// Copyright (c) 2024 bitfl0wer
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use serde::de::Visitor;
use serde::{Deserialize, Serialize};

pub struct Ia5String(der::asn1::Ia5String);

impl Deref for Ia5String {
    type Target = der::asn1::Ia5String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Ia5String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<der::asn1::Ia5String> for Ia5String {
    fn from(s: der::asn1::Ia5String) -> Self {
        Self(s)
    }
}

impl From<Ia5String> for der::asn1::Ia5String {
    fn from(s: Ia5String) -> Self {
        s.0
    }
}

impl<'de> Deserialize<'de> for Ia5String {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(Ia5StringVisitor)
    }
}

struct Ia5StringVisitor;

impl<'de> Visitor<'de> for Ia5StringVisitor {
    type Value = Ia5String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter
            .write_str("A concatenation of characters from the IA5 character set in &str format.")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Ia5String(match der::asn1::Ia5String::new(&v.to_string()) {
            Ok(val) => val,
            Err(e) => return Err(E::custom(e)),
        }))
    }
}

impl Serialize for Ia5String {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.to_string().as_str())
    }
}
