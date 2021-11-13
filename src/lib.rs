use std::{str::FromStr, time::Duration};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Edm {
    #[serde(rename = "Edm.Binary")]
    Binary,
    #[serde(rename = "Edm.Boolean")]
    Boolean,
    #[serde(rename = "Edm.Byte")]
    Byte,
    #[serde(rename = "Edm.DateTime")]
    DateTime,
    #[serde(rename = "Edm.DateTimeOffset")]
    DateTimeOffset,
    #[serde(rename = "Edm.Decimal")]
    Decimal,
    #[serde(rename = "Edm.Double")]
    Double,
    #[serde(rename = "Edm.Int16")]
    Int16,
    #[serde(rename = "Edm.Int32")]
    Int32,
    #[serde(rename = "Edm.String")]
    String,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EdmxVersion {
    #[serde(rename = "1.0")]
    V1_0,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Edmx {
    pub version: EdmxVersion,
    pub data_services: DataServices,
}

impl Edmx {
    pub fn default_schema(&self) -> Option<&Schema> {
        self.data_services.default_schema()
    }
}

impl FromStr for Edmx {
    type Err = quick_xml::DeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        quick_xml::de::from_str(s)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataServices {
    #[serde(rename = "Schema", default)]
    pub schemas: Vec<Schema>,
}

impl DataServices {
    pub fn default_schema(&self) -> Option<&Schema> {
        self.schemas
            .iter()
            .find(|schema| schema.namespace == "Default")
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Schema {
    pub namespace: String,
    #[serde(rename = "EntityType", default)]
    pub entities: Vec<EntityType>,
    #[serde(rename = "Association", default)]
    pub associations: Vec<Association>,
    pub entity_container: Option<EntityContainer>,
}

impl Schema {
    pub fn entity_sets(&self) -> Option<&Vec<EntitySet>> {
        self.entity_container
            .as_ref()
            .map(|container| &container.entity_sets)
    }

    pub fn association_sets(&self) -> Option<&Vec<AssociationSet>> {
        self.entity_container
            .as_ref()
            .map(|container| &container.association_sets)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntityContainer {
    pub name: String,
    #[serde(rename = "EntitySet", default)]
    pub entity_sets: Vec<EntitySet>,
    #[serde(rename = "AssociationSet", default)]
    pub association_sets: Vec<AssociationSet>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Association {
    pub name: String,

    #[serde(rename = "End")]
    pub ends: [End; 2],
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssociationSet {
    pub name: String,
    pub association: String,

    #[serde(rename = "End")]
    pub ends: [End; 2],
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct End {
    pub role: Option<String>,
    pub entity_set: Option<String>,
    #[serde(rename = "Type")]
    pub entity_type: Option<String>,
    pub multiplicity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntitySet {
    pub name: String,
    pub entity_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EntityType {
    pub name: String,
    pub key: Key,
    #[serde(rename = "Property", default)]
    pub properties: Vec<Property>,
    #[serde(rename = "NavigationProperty", default)]
    pub navigations: Vec<NavigationProperty>,
}

impl EntityType {
    pub fn key_property(&self) -> Option<&Property> {
        self.properties
            .iter()
            .find(|property| property.name == self.key.property_ref.name)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NavigationProperty {
    pub name: String,
    pub relationship: String,
    pub to_role: String,
    pub from_role: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Key {
    pub property_ref: PropertyRef,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PropertyRef {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Property {
    pub name: String,
    #[serde(flatten)]
    pub inner: PropertyType,
    #[serde(default = "default_true")]
    pub nullable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", tag = "Type")]
pub enum PropertyType {
    #[serde(rename = "Edm.Binary")]
    Binary {
        max_length: Option<u32>,
        fixed_length: Option<u32>,
        default: Option<Vec<u8>>,
    },
    #[serde(rename = "Edm.Boolean")]
    Boolean { default: Option<bool> },
    #[serde(rename = "Edm.Byte")]
    Byte {
        precision: Option<u8>,
        default: Option<Vec<u8>>,
    },
    #[serde(rename = "Edm.DateTime")]
    DateTime {
        precision: Option<u8>,
        default: Option<NaiveDateTime>,
    },
    #[serde(rename = "Edm.DateTimeOffset")]
    DateTimeOffset {
        precision: Option<u8>,
        default: Option<Duration>,
    },
    #[serde(rename = "Edm.Decimal")]
    Decimal {
        precision: Option<u8>,
        default: Option<f64>,
    },
    #[serde(rename = "Edm.Double")]
    Double {
        precision: Option<u8>,
        default: Option<f64>,
    },
    #[serde(rename = "Edm.Int16")]
    Int16 {
        precision: Option<u8>,
        default: Option<i16>,
    },
    #[serde(rename = "Edm.Int32")]
    Int32 {
        precision: Option<u8>,
        default: Option<Vec<u8>>,
    },
    #[serde(rename = "Edm.String")]
    String {
        precision: Option<u8>,
        max_length: Option<u32>,
        fixed_length: Option<u32>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_folketinget_metadata() {
        let edmx = Edmx::from_str(include_str!("../tests/folketinget.xml")).unwrap();

        for set in edmx.default_schema().unwrap().entity_sets().unwrap() {
            println!("{:#?}", set);
        }

        assert_eq!(
            50,
            edmx.default_schema().unwrap().entity_sets().unwrap().len()
        );
    }
}
