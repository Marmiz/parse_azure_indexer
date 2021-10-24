//! Internal implementations.
//! At the core this package takes an indexer definition
//! and serialize it into a typescript definition.

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Write},
};

/// The high level indexer definition.
///
/// # Example
/// ```
///     "name": "azure-indexer",
///     "fields": [
///    {
///       "name": "PartitionKey",
///        "type": "Edm.String",
///        "searchable": true,
///        "filterable": true,
///        "retrievable": true,
///        "sortable": true,
///        "facetable": false,
///        "key": false,
///        "indexAnalyzer": null,
///        "searchAnalyzer": null,
///        "analyzer": null,
///        "synonymMaps": []
///   }
///  ]
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Definiton {
    /// Name used to generate Typescritp interface
    name: String,
    fields: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    /// Name used as key
    name: String,
    /// type is a reserved word. use de(deserialized)_type instead.
    /// this is one of the odata types.
    #[serde(rename(deserialize = "type"))]
    de_type: String,
}

/// Core conversion.
/// display the field as `key: value;` pairs
impl Display for Field {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let converted = self.convert_to_ts();
        write!(formatter, "{}: {};", self.name, converted)
    }
}

impl Field {
    fn convert_to_ts(&self) -> String {
        match &self.de_type.starts_with("Collection(") {
            true => self.parse_collection(&self.de_type),
            false => self.parse_str(&self.de_type),
        }
    }

    fn parse_str(&self, s: &str) -> String {
        match s {
            "Edm.String" => String::from("string"),
            "Edm.GeographyPoint" => String::from("Coordinates"),
            "Edm.Double" => String::from("number"),
            "Edm.Int32" => String::from("number"),
            "Edm.Int64" => String::from("number"),
            "Edm.DateTimeOffset" => String::from("Date"),
            "Edm.Boolean" => String::from("boolean"),
            "Edm.ComplexType" => String::from("{}"),
            _ => String::from("any"),
        }
    }

    fn parse_collection(&self, s: &str) -> String {
        let content = &s[11..];
        let end_paren = content.rfind(")").unwrap();
        let content = &content[..end_paren];
        String::from(format!("{}[]", self.parse_str(content)))
    }
}

impl Definiton {
    pub fn write_to_file(&self, mut buff: File) -> File {
        buff.write_all(b"type Coordinates = {\ntype: string;\ncoordinates: number[];\n}\n\n")
            .expect("Error writing static portion to file");

        let name: String = self
            .name
            .split("-")
            .map(|x| make_ascii_titlecase(x))
            .collect();
        write!(buff, "interface {} {{\n", name).expect("Error writing to file");

        for v in &self.fields {
            buff.write_all(format!("{}\n", v).as_bytes())
                .expect("Error writing to file")
        }

        write!(buff, "}}\n").expect("Error writing to file");
        buff
    }
}

/// Serilize a Definition from a Reader.
pub fn parse_json(rdr: BufReader<File>) -> Result<Definiton> {
    let def: Definiton = serde_json::from_reader(rdr)?;
    Ok(def)
}

/// Title case a string.
///
/// # Example
/// ```
/// fn main() {
///     let s = "hello world";
///     let v = make_ascii_titlecase(s);
///     
///     println!("{}", v)
/// }
/// // "Hello world"
/// ```
fn make_ascii_titlecase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
