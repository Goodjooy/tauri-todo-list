use sqlx::{Database, Decode, Encode, Sqlite, Type};

use crate::todo_storage::PriorityLevel;

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum Priority {
    VeryHigh = 0,
    High = 1,
    Medium = 2,
    Low = 3,
    VeryLow = 4,
}

#[derive(Debug, thiserror::Error)]
#[error("Expect u8 from 9 to 4, but get {0}")]
pub struct BadPriorityError(u8);

impl TryFrom<u8> for Priority {
    type Error = BadPriorityError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::VeryHigh,
            1 => Self::High,
            2 => Self::Medium,
            3 => Self::Low,
            4 => Self::VeryLow,
            u => Err(BadPriorityError(u))?,
        })
    }
}

impl Into<u8> for Priority {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Into<sea_query::Value> for Priority {
    fn into(self) -> sea_query::Value {
        u8::into(self.into())
    }
}

impl Into<u8> for &Priority {
    fn into(self) -> u8 {
        (*self) as u8
    }
}

impl Into<PriorityLevel> for Priority {
    fn into(self) -> PriorityLevel {
        match self {
            Priority::VeryHigh => PriorityLevel::VeryHigh,
            Priority::High => PriorityLevel::High,
            Priority::Medium => PriorityLevel::Medium,
            Priority::Low => PriorityLevel::Low,
            Priority::VeryLow => PriorityLevel::VeryLow,
        }
    }
}

impl From<PriorityLevel> for Priority {
    fn from(value: PriorityLevel) -> Self {
        match value {
            PriorityLevel::VeryHigh => Self::VeryHigh,
            PriorityLevel::High => Self::High,
            PriorityLevel::Medium => Self::Medium,
            PriorityLevel::Low => Self::Low,
            PriorityLevel::VeryLow => Self::VeryLow,
        }
    }
}

impl Type<Sqlite> for Priority {
    fn type_info() -> <Sqlite as Database>::TypeInfo {
        u8::type_info()
    }
}

impl<'r> Decode<'r, Sqlite> for Priority {
    fn decode(
        value: <Sqlite as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let v = u8::decode(value)?;
        Ok(v.try_into()?)
    }
}

impl<'q> Encode<'q, Sqlite> for Priority {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let v: u8 = self.into();
        v.encode_by_ref(buf)
    }
}
