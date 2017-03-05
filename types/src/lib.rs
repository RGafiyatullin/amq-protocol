extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;

/* Fields with a custom rename are for deserializing
 * the secifications json file in amq-protocol-codegen
 */
#[derive(Debug, Deserialize, Serialize)]
pub enum AMQPType {
    #[serde(rename="bit")]
    Boolean,
    #[serde(rename="octet")]
    ShortShortInt,
    ShortShortUInt,
    #[serde(rename="short")]
    ShortInt,
    ShortUInt,
    #[serde(rename="long")]
    LongInt,
    LongUInt,
    #[serde(rename="longlong")]
    LongLongInt,
    LongLongUInt,
    Float,
    Double,
    DecimalValue,
    #[serde(rename="shortstr")]
    ShortString,
    #[serde(rename="longstr")]
    LongString,
    FieldArray,
    #[serde(rename="timestamp")]
    TimeStamp,
    #[serde(rename="table")]
    FieldTable,
    Void,
}

impl AMQPType {
    pub fn from_id(id: char) -> AMQPType {
        match id {
            't' => AMQPType::Boolean,
            'b' => AMQPType::ShortShortInt,
            'B' => AMQPType::ShortShortUInt,
            'U' => AMQPType::ShortInt,
            'u' => AMQPType::ShortUInt,
            'I' => AMQPType::LongInt,
            'i' => AMQPType::LongUInt,
            'L' => AMQPType::LongLongInt,
            'l' => AMQPType::LongLongUInt,
            'f' => AMQPType::Float,
            'd' => AMQPType::Double,
            'D' => AMQPType::DecimalValue,
            's' => AMQPType::ShortString,
            'S' => AMQPType::LongString,
            'A' => AMQPType::FieldArray,
            'T' => AMQPType::TimeStamp,
            'F' => AMQPType::FieldTable,
            'V' => AMQPType::Void,
            _ => panic!("Unknown type id {}", id), /*FIXME: don't panic*/
        }
    }

    pub fn get_id(&self) -> char {
        match *self {
            AMQPType::Boolean => 't',
            AMQPType::ShortShortInt => 'b',
            AMQPType::ShortShortUInt => 'B',
            AMQPType::ShortInt => 'U',
            AMQPType::ShortUInt => 'u',
            AMQPType::LongInt => 'I',
            AMQPType::LongUInt => 'i',
            AMQPType::LongLongInt => 'L',
            AMQPType::LongLongUInt => 'l',
            AMQPType::Float => 'f',
            AMQPType::Double => 'd',
            AMQPType::DecimalValue => 'D',
            AMQPType::ShortString => 's',
            AMQPType::LongString => 'S',
            AMQPType::FieldArray => 'A',
            AMQPType::TimeStamp => 'T',
            AMQPType::FieldTable => 'F',
            AMQPType::Void => 'V',
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub type Boolean        = bool;
pub type ShortShortInt  = i8;
pub type ShortShortUInt = u8;
pub type ShortInt       = i16;
pub type ShortUInt      = u16;
pub type LongInt        = i32;
pub type LongUInt       = u32;
pub type LongLongInt    = i64;
pub type LongLongUInt   = u64;
pub type Float          = f32;
pub type Double         = f64;
pub type DecimalValue   = i64;    /* FIXME: find out what this is exactly */
pub type ShortString    = String; /* TODO: don't allow size >= 255 */
pub type LongString     = String;
pub type FieldArray     = Vec<AMQPValue>;
pub type TimeStamp      = u64;
pub type FieldTable     = HashMap<String, AMQPValue>;
pub type Void           = ();

pub enum AMQPValue {
    Boolean(Boolean),
    ShortShortInt(ShortShortInt),
    ShortShortUInt(ShortShortUInt),
    ShortInt(ShortInt),
    ShortUInt(ShortUInt),
    LongInt(LongInt),
    LongUInt(LongUInt),
    LongLongInt(LongLongInt),
    LongLongUInt(LongLongUInt),
    Float(Float),
    Double(Double),
    DecimalValue(DecimalValue),
    ShortString(ShortString),
    LongString(LongString),
    FieldArray(FieldArray),
    TimeStamp(TimeStamp),
    FieldTable(FieldTable),
    Void(Void),
}
