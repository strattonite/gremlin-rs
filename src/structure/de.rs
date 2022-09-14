use std::num::*;
use std::str::{from_utf8, FromStr, Utf8Error};

use serde::de::{
    self, Deserialize, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
    VariantAccess,
};
use thiserror::Error;

// pub fn from_ws<T: DeserializeOwned>(m: Message) -> GResult<T> {
//     match m {
//         Message::Binary(bin) => {
//             let mut d = Deserializer::from_vec(&bin);
//             T::deserialize(&mut d)
//         }
//         _ => return Err(InvalidDataType),
//     }
// }

// pub fn from_slice<'de, T: Deserialize<'de>>(s: &'de [u8]) -> GResult<T> {
//     let mut d = Deserializer::from_slice(s);
//     T::deserialize(&mut d)
// }

pub fn from_str<'de, T: Deserialize<'de>>(s: &'de str) -> GResult<T> {
    let mut d = Deserializer::from_slice(s.as_bytes());
    T::deserialize(&mut d)
}

pub fn from_vec<'de, T: Deserialize<'de>>(v: &'de Vec<u8>) -> GResult<T> {
    let mut d = Deserializer::from_vec(v);
    T::deserialize(&mut d)
}

#[derive(Error, Debug)]
pub enum GsonError {
    #[error(transparent)]
    FloatError(#[from] ParseFloatError),
    #[error(transparent)]
    IntError(#[from] ParseIntError),
    #[error("end of file encountered")]
    Eof,
    #[error(transparent)]
    InvalidUTF8(#[from] Utf8Error),
    #[error("error deserializing: {0}")]
    CustomError(String),
    #[error("expected separator: {0}")]
    ExpectedSeparator(char),
    #[error("invalid syntax: expected {0} - got {1}")]
    InvalidSyntax(&'static str, String),
    #[error("invalid data type: expected binary")]
    InvalidDataType,
}

impl GsonError {
    pub fn invalid_char(e: &'static str, g: u8) -> Self {
        InvalidSyntax(e, String::from_utf8_lossy(&[g]).to_string())
    }
    pub fn invalid_str(e: &'static str, g: &[u8]) -> Self {
        InvalidSyntax(e, String::from_utf8_lossy(g).to_string())
    }
    pub fn invalid_string(e: &'static str, g: &str) -> Self {
        InvalidSyntax(e, g.to_string())
    }
}

impl de::Error for GsonError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::CustomError(format!("{}", msg))
    }
}

use GsonError::*;

pub type GResult<T> = Result<T, GsonError>;

pub struct Deserializer<'de> {
    input: &'de [u8],
    data_depth: usize,
}

impl<'de> Deserializer<'de> {
    pub fn from_vec(v: &'de Vec<u8>) -> Self {
        Self {
            input: v.as_slice(),
            data_depth: 0,
        }
    }

    pub fn from_slice(s: &'de [u8]) -> Self {
        Self {
            input: s,
            data_depth: 0,
        }
    }

    pub(crate) fn peek_byte(&self) -> GResult<&u8> {
        self.input.first().ok_or(Eof)
    }

    pub(crate) fn next_byte(&mut self) -> GResult<u8> {
        let b = *self.peek_byte()?;
        self.input = &self.input[1..];
        Ok(b)
    }

    pub(crate) fn get_byte(&mut self, n: usize) -> GResult<&u8> {
        self.input.get(n).ok_or(Eof)
    }

    pub(crate) fn get_str(&mut self) -> GResult<&'de str> {
        match self.next_byte()? {
            b'"' => {
                let mut escaped = false;
                let mut n = 0;
                loop {
                    match self.get_byte(n)? {
                        b'"' if !escaped => break,
                        b'\\' => {
                            escaped = !escaped;
                            n += 1;
                        }
                        _ => {
                            escaped = false;
                            n += 1;
                        }
                    }
                }

                let v = from_utf8(&self.input[..n])?;
                self.input = &self.input[(n + 1)..];
                Ok(v)
            }
            x => Err(GsonError::invalid_char("\"", x)),
        }
    }

    pub(crate) fn take_next(&mut self, n: usize) -> GResult<&[u8]> {
        if n > self.input.len() {
            return Err(Eof);
        }
        let v = &self.input[..n];
        self.input = &self.input[n..];
        Ok(v)
    }

    pub(crate) fn peek_next(&mut self, n: usize) -> GResult<&[u8]> {
        if n > self.input.len() {
            return Err(Eof);
        }
        let v = &self.input[..n];
        Ok(v)
    }

    pub(crate) fn get_float<T: FromStr<Err = ParseFloatError>>(&mut self) -> GResult<T> {
        let mut end = 0;
        loop {
            match self.get_byte(end)? {
                b'0'..=b'9' | b'.' | b'-' | b'E' => end += 1,
                _ => {
                    let v = FromStr::from_str(from_utf8(self.take_next(end)?)?)
                        .map_err(|e: ParseFloatError| e.into());
                    match self.next_byte()? {
                        b',' | b'}' | b']' => (),
                        x => return Err(GsonError::invalid_char(",|}|]", x)),
                    };
                    return v;
                }
            }
        }
    }

    pub(crate) fn get_int<T: FromStr<Err = ParseIntError>>(&mut self) -> GResult<T> {
        let mut end = 0;
        loop {
            match self.get_byte(end)? {
                b'0'..=b'9' => end += 1,
                _ => {
                    let v = FromStr::from_str(from_utf8(self.take_next(end)?)?)
                        .map_err(|e: ParseIntError| e.into());
                    match self.next_byte()? {
                        b',' | b'}' | b']' => (),
                        x => return Err(GsonError::invalid_char(",|}|]", x)),
                    };
                    return v;
                }
            }
        }
    }

    pub(crate) fn get_string(&mut self) -> GResult<String> {
        self.get_str().map(String::from)
    }

    pub(crate) fn skip_type(&mut self) -> GResult<()> {
        match self.take_next(8)? {
            br#""@type":"# => (),
            x => return Err(GsonError::invalid_str(r#""@type":"#, x)),
        }
        Ok(())
    }

    pub(crate) fn skip_value(&mut self) -> GResult<()> {
        match self.take_next(9)? {
            br#""@value":"# => (),
            x => return Err(GsonError::invalid_str(r#""@value":"#, x)),
        }
        Ok(())
    }

    pub(crate) fn get_gv_type(&mut self) -> GResult<&'de str> {
        match self.next_byte()? {
            b'{' => {
                self.skip_type()?;
                let t = self.get_str()?;
                match self.next_byte()? {
                    b',' => {
                        self.skip_value()?;
                        Ok(t)
                    }
                    x => Err(GsonError::invalid_char(",", x)),
                }
            }
            x => Err(GsonError::invalid_char("{", x)),
        }
    }

    pub(crate) fn peek_str(&mut self, offset: usize) -> GResult<&'de [u8]> {
        if offset >= self.input.len() {
            return Err(Eof);
        }
        match *self.get_byte(offset)? {
            b'"' => {
                let mut escaped = false;
                let mut n = offset;
                n += 1;
                loop {
                    match self.get_byte(n)? {
                        b'"' if !escaped => break,
                        b'\\' => {
                            escaped = !escaped;
                            n += 1
                        }
                        _ => {
                            escaped = false;
                            n += 1
                        }
                    }
                }
                Ok(&self.input[(offset + 1)..n])
            }
            x => return Err(GsonError::invalid_char("\"", x)),
        }
    }

    fn get_uuid(&mut self) -> GResult<&'de str> {
        let t = self.get_gv_type()?;
        match t {
            "g:UUID" => self.get_str(),
            x => Err(GsonError::invalid_string("g:UUID", x)),
        }
    }

    fn ignore_next(&mut self) -> GResult<()> {
        let mut recursion_lvl = 0;
        let mut in_string = false;
        let mut in_num = false;
        let mut escaped = false;
        loop {
            let b = self.next_byte()?;

            if in_string {
                if !escaped {
                    if b == b'"' {
                        in_string = false;
                    } else if b == b'\\' {
                        escaped = true;
                    }
                } else {
                    escaped = false;
                }
            } else {
                match b {
                    b'{' | b'[' => {
                        in_num = false;
                        recursion_lvl += 1
                    }
                    b'}' | b']' => {
                        in_num = false;
                        recursion_lvl -= 1
                    }
                    b'"' => {
                        in_num = false;
                        in_string = true
                    }
                    b't' => {
                        in_num = false;
                        if *b"rue" != self.input[..3] {
                            return Err(GsonError::invalid_str("rue", &self.input[..3]));
                        }
                        self.input = &self.input[3..]
                    }
                    b'f' => {
                        in_num = false;
                        if *b"alse" != self.input[..4] {
                            return Err(GsonError::invalid_str("alse", &self.input[..4]));
                        }
                        self.input = &self.input[4..]
                    }
                    b'n' => {
                        in_num = false;
                        if *b"ull" != self.input[..3] {
                            return Err(GsonError::invalid_str("ull", &self.input[..3]));
                        }
                        self.input = &self.input[3..]
                    }
                    b'0'..=b'9' | b'-' => {
                        in_num = true;
                    }
                    x @ (b'e' | b'E' | b'.') => {
                        if !in_num {
                            return Err(GsonError::invalid_char("valid json", x));
                        }
                    }
                    b':' | b',' => in_num = false,
                    y => return Err(GsonError::invalid_char("valid json", y)),
                }
            }
            if recursion_lvl == 0 && !in_string && !in_num {
                break;
            }
        }
        Ok(())
    }

    fn unwrap_traverser(&mut self) -> GResult<bool> {
        if self.input.len() > 23 {
            if *br#"{"@type":"g:Traverser","# == self.input[..23] {
                let _ = self.get_gv_type()?;
                if self.input[..8] != *br#"{"bulk":"# {
                    return Err(GsonError::invalid_str(r#"{"bulk":"#, &self.input[..8]));
                }
                self.input = &self.input[8..];
                self.ignore_next()?;
                if *br#","value":"# != self.input[..9] {
                    return Err(GsonError::invalid_str(r#","value":"#, &self.input[..9]));
                }
                self.input = &self.input[9..];
                return Ok(true);
            }
        }
        Ok(false)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = GsonError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.peek_byte()? {
            b'{' => {
                // br#"{"@type":"g:Date","@value":0}"#.len()
                if self.input.len() < 29 {
                    return self.deserialize_map(visitor);
                }
                match self.peek_next(9)? {
                    br#"{"@type":"# => {
                        let t = self.peek_str(9)?;
                        match t {
                            b"g:Date" => self.deserialize_i64(visitor),
                            b"g:Double" => self.deserialize_f64(visitor),
                            b"g:Float" => self.deserialize_f32(visitor),
                            b"g:Int32" => self.deserialize_i32(visitor),
                            b"g:Int64" => self.deserialize_i64(visitor),
                            b"g:Timestamp" => self.deserialize_i64(visitor),
                            b"g:UUID" => self.deserialize_str(visitor),
                            b"g:Edge" => self.deserialize_struct(
                                "Edge",
                                &["id", "label", "inVLabel", "outVLabel", "inV", "outV"],
                                visitor,
                            ),
                            b"g:Path" => {
                                self.deserialize_struct("Path", &["labels", "objects"], visitor)
                            }
                            b"g:Property" => {
                                self.deserialize_struct("Property", &["key", "value"], visitor)
                            }
                            b"g:Vertex" => {
                                self.deserialize_struct("Vertex", &["id", "label"], visitor)
                            }
                            b"g:VertexProperty" => self.deserialize_struct(
                                "VertexProperty",
                                &["id", "value", "label"],
                                visitor,
                            ),
                            _ => return Err(GsonError::invalid_str("g:Identifier", t)),
                        }
                    }
                    _ => self.deserialize_map(visitor),
                }
            }
            b'[' => self.deserialize_seq(visitor),
            b'"' => self.deserialize_string(visitor),
            b't' | b'f' => self.deserialize_bool(visitor),
            x => Err(GsonError::invalid_char("valid json", *x)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.next_byte()? {
            b't' => match self.take_next(3)? {
                b"rue" => visitor.visit_bool(true),
                x => Err(GsonError::invalid_str("rue", x)),
            },
            b'f' => match self.take_next(4)? {
                b"alse" => visitor.visit_bool(true),
                x => Err(GsonError::invalid_str("alse", x)),
            },
            x => Err(GsonError::invalid_char("bool", x)),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.get_int()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.get_int()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let t = self.get_gv_type()?;
        match t {
            "g:Int32" => visitor.visit_i32(self.get_int()?),
            t => Err(GsonError::invalid_string("g:Int32", t)),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let t = self.get_gv_type()?;
        match t {
            "g:Int64" => visitor.visit_i64(self.get_int()?),
            t => Err(GsonError::invalid_string("g:Int64", t)),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.get_int()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.get_int()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.get_int()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.get_int()?)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let t = self.get_gv_type()?;
        match t {
            "g:Timestamp" | "g:Date" => visitor.visit_u64(self.get_int()?),
            t => Err(GsonError::invalid_string("g:Timestamp", t)),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let t = self.get_gv_type()?;
        match t {
            "g:Float" => visitor.visit_f32(self.get_float()?),
            t => Err(GsonError::invalid_string("g:Float", t)),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let t = self.get_gv_type()?;
        match t {
            "g:Double" => visitor.visit_f64(self.get_float()?),
            t => Err(GsonError::invalid_string("g:Double", t)),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_char(self.next_byte()?.into())
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.input.len() > 27 {
            if *br#"{"@type":"g:UUID","@value":"# == self.input[0..27] {
                return visitor.visit_str(self.get_uuid()?);
            }
        }
        visitor.visit_borrowed_str(self.get_str()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(self.get_string()?)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if b"null" == self.peek_next(4)? {
            self.input = &self.input[4..];
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if b"null" == self.peek_next(4)? {
            self.input = &self.input[4..];
            visitor.visit_unit()
        } else {
            Err(GsonError::invalid_str("null", self.peek_next(4)?))
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let b = self.next_byte()?;
        if b'[' != b {
            return Err(GsonError::invalid_char("[", b));
        }
        self.data_depth += 1;
        println!("deserialing seq, depth: {}", self.data_depth);
        let val = visitor.visit_seq(CommaSeparated::new(self, self.data_depth == 1));
        val
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let b = self.next_byte()?;
        if b != b'{' {
            return Err(GsonError::invalid_char("{", b));
        }
        let val = visitor.visit_map(CommaSeparated::new(self, false));
        val
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        #[cfg(test)]
        println!("deserializing struct: {}", _name);
        if _name == "VertexProperty" {
            println!(
                "{}",
                self.peek_next(50)
                    .map(|x| from_utf8(x).unwrap())
                    .unwrap_or("invalid len")
            );
        }
        let val = self.deserialize_map(visitor);
        val
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.peek_byte()? {
            b'"' => visitor.visit_enum(self.get_str()?.into_deserializer()),
            b'{' => {
                let val = visitor.visit_enum(EnumParser::new(self));
                match self.next_byte()? {
                    b'}' => val,
                    x => Err(GsonError::invalid_char("}", x)),
                }
            }
            x => Err(GsonError::invalid_char("\" | {", *x)),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.ignore_next()?;
        visitor.visit_unit()
    }
}

struct CommaSeparated<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    first: bool,
    check_traverser: bool,
}

impl<'a, 'de> CommaSeparated<'a, 'de> {
    fn new(deserializer: &'a mut Deserializer<'de>, check_traverser: bool) -> Self {
        Self {
            de: deserializer,
            first: true,
            check_traverser,
        }
    }
}

impl<'de, 'a> SeqAccess<'de> for CommaSeparated<'a, 'de> {
    type Error = GsonError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.de.peek_byte()? {
            b']' => {
                println!(
                    "end of sequence detected, depth: {}",
                    self.de.data_depth - 1
                );
                self.de.data_depth -= 1;
                self.de.input = &self.de.input[1..];
                return Ok(None);
            }
            b',' => {
                self.de.input = &self.de.input[1..];
                let t = self.check_traverser && self.de.unwrap_traverser()?;
                let val = seed.deserialize(&mut *self.de).map(Some);
                if t {
                    match self.de.take_next(2)? {
                        b"}}" => (),
                        x => return Err(GsonError::invalid_str("}}", x)),
                    }
                } else {
                    println!("did not unwrap traverser")
                }
                return val;
            }
            _ => {
                if self.first {
                    let t = self.check_traverser && self.de.unwrap_traverser()?;
                    if t {
                        println!(
                            "unwrapped traverser, deserializing value: {}",
                            self.de
                                .peek_next(50)
                                .map(|x| from_utf8(x).unwrap())
                                .unwrap_or("invalid len")
                        )
                    }
                    let val = seed.deserialize(&mut *self.de).map(Some);

                    if t {
                        match self.de.take_next(2)? {
                            b"}}" => (),
                            x => return Err(GsonError::invalid_str("}}", x)),
                        }
                    } else {
                        println!("did not unwrap traverser")
                    }
                    return val;
                } else {
                    return Err(ExpectedSeparator(','));
                }
            }
        }
    }
}

impl<'de, 'a> MapAccess<'de> for CommaSeparated<'a, 'de> {
    type Error = GsonError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        match self.de.peek_byte()? {
            b'}' => {
                self.de.input = &self.de.input[1..];
                return Ok(None);
            }
            b',' => {
                self.de.input = &self.de.input[1..];
                return seed.deserialize(&mut *self.de).map(Some);
            }
            _ => {
                if self.first {
                    return seed.deserialize(&mut *self.de).map(Some);
                } else {
                    return Err(ExpectedSeparator(','));
                }
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.de.next_byte()? {
            b':' => seed.deserialize(&mut *self.de),
            x => Err(GsonError::invalid_char(":", x)),
        }
    }
}

struct EnumParser<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> EnumParser<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        EnumParser { de }
    }
}

impl<'de, 'a> EnumAccess<'de> for EnumParser<'a, 'de> {
    type Error = GsonError;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;
        match self.de.next_byte()? {
            b':' => Ok((val, self)),
            x => Err(GsonError::invalid_char(":", x)),
        }
    }
}

impl<'de, 'a> VariantAccess<'de> for EnumParser<'a, 'de> {
    type Error = GsonError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(CustomError("expected string".to_string()))
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_map(self.de, visitor)
    }
}
