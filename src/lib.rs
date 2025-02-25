mod ulid;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use schemars::schema::{InstanceType, Metadata, Schema, SchemaObject};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
use sqlx::{Decode, Encode, Postgres, Type};
use thiserror::Error;
use crate::ulid::Ulid;

#[derive(Debug, Error)]
pub enum ResourceIDError {
    #[error("Unable to decode internal Ulid: {0}")]
    UnableToDecodeUlid(ulid::DecodeError),

    #[error("Invalid resource type on identifier: {0}")]
    InvalidIdentifierResourceLength(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceID {
    resource: String,
    ulid: Ulid,
}

impl JsonSchema for ResourceID {
    fn schema_name() -> String {
        "ResourceID".to_string()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: Some("ResourceID".to_string()),
            metadata: Some(Box::new(Metadata {
                title: Some(String::from("ResourceID")),
                description: Some(String::from(
                    "A unique resource identifier",
                )),
                ..Default::default()
            })),
            ..Default::default()
        }
            .into()
    }
}

impl ResourceID {
    pub fn new(resource: &str) -> Result<Self, ResourceIDError> {
        let ulid = Ulid::new();

        Self::validate_resource(resource)?;

        Ok(Self {
            resource: resource.to_uppercase(),
            ulid,
        })
    }


    fn validate_resource<S: ToString>(resource: S) -> Result<(), ResourceIDError> {
        let value = resource.to_string();
        if value.len() < 4 {
            Err(ResourceIDError::InvalidIdentifierResourceLength(
                value,
            ))
        } else {
            Ok(())
        }
    }


    // Should there be a method to get_resource_type?
}

impl Display for ResourceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.resource, self.ulid.to_string())
    }
}

impl FromStr for ResourceID {
    type Err = ResourceIDError;

    fn from_str(s: &str) -> Result<Self, ResourceIDError> {
        let resource_str = &s[..4];
        let ulid_str = &s[4..];

        let ulid = Ulid::from_str(ulid_str).map_err(ResourceIDError::UnableToDecodeUlid)?;

        Self::validate_resource(resource_str)?;

        Ok(ResourceID { resource: String::from(resource_str.to_uppercase()), ulid })
    }
}

impl Type<Postgres> for ResourceID {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("VARCHAR")
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        *ty == PgTypeInfo::with_name("VARCHAR")
    }
}

impl Encode<'_, Postgres> for ResourceID {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let value = self.to_string();
        buf.extend_from_slice(value.as_bytes());
        Ok(sqlx::encode::IsNull::No)
    }
}

impl<'r> Decode<'r, Postgres> for ResourceID {
    fn decode(value: PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let str_value = value.as_str()?;
        ResourceID::from_str(str_value).map_err(|e| e.into())
    }
}

impl Serialize for ResourceID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ResourceID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ResourceID::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_id_minimal_length_error() {
        let id = ResourceID::new("foo");

        assert!(id.is_err());
    }

    #[test]
    fn resource_id_serialization() {
        let id = ResourceID::new("user").unwrap();

        assert_eq!(&id.to_string()[..4], "USER");
    }

    // TODO: Add a test that shows how to use strum with this library
    // #[derive(Debug, Clone, PartialEq, Eq, EnumString, Display, JsonSchema)]
    // pub enum ResourceIDResource {
    //     #[strum(serialize = "USER")]
    //     User,
    //     #[strum(serialize = "ACCT")]
    //     Account,
    // }
}