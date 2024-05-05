// Copyright (c) 2024 bitfl0wer
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ops::{Deref, DerefMut};

use der::Decode;
use serde::de::Visitor;

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

struct Ia5StringVisitor;

impl<'de> Visitor<'de> for Ia5StringVisitor {
    type Value = Ia5String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A concatenation of characters from the IA5 character set")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Ia5String(match der::asn1::Ia5String::new(&v.to_string()) {
            Ok(val) => val,
            Err(e) => return Err(E::custom(e)),
        }))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Ia5String(match der::asn1::Ia5String::new(&v.to_string()) {
            Ok(val) => val,
            Err(e) => return Err(E::custom(e)),
        }))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Ia5String(match der::asn1::Ia5String::new(&v.to_string()) {
            Ok(val) => val,
            Err(e) => return Err(E::custom(e)),
        }))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Ia5String(match der::asn1::Ia5String::new(&v.to_string()) {
            Ok(val) => val,
            Err(e) => return Err(E::custom(e)),
        }))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Ia5String(match der::asn1::Ia5String::new(&v.to_string()) {
            Ok(val) => val,
            Err(e) => return Err(E::custom(e)),
        }))
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

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Ia5String(match der::asn1::Ia5String::from_der(v) {
            Ok(val) => val,
            Err(e) => return Err(E::custom(e)),
        }))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Ia5String(
            der::asn1::Ia5String::new("").expect("Failed to create empty Ia5String"),
        ))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let _ = deserializer;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Option,
            &self,
        ))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Unit,
            &self,
        ))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let _ = deserializer;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::NewtypeStruct,
            &self,
        ))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let _ = seq;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Seq,
            &self,
        ))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let _ = map;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Map,
            &self,
        ))
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        let _ = data;
        Err(serde::de::Error::invalid_type(
            serde::de::Unexpected::Enum,
            &self,
        ))
    }
}
