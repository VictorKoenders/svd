

use std::collections::HashMap;

use xmltree::Element;
use ElementExt;
use failure::ResultExt;

use parse;
use types::{Parse, Encode, new_element};
use error::*;
use svd::usage::Usage;
use svd::enumeratedvalue::EnumeratedValue;

#[derive(Clone, Debug, PartialEq)]
pub struct EnumeratedValues {
    pub name: Option<String>,
    pub usage: Option<Usage>,
    pub derived_from: Option<String>,
    pub values: Vec<EnumeratedValue>,
    // Reserve the right to add more fields to this struct
    pub (crate) _extensible: (),
}

impl Parse for EnumeratedValues {
    type Object = EnumeratedValues;
    type Error = SVDError;

    fn parse(tree: &Element) -> Result<EnumeratedValues, SVDError> {
        assert_eq!(tree.name, "enumeratedValues");

        Ok(EnumeratedValues {
            name: tree.get_child_text_opt("name")?,
            usage: parse::optional("usage", tree, Usage::parse)?,
            derived_from: tree.attributes
                .get(&"derivedFrom".to_owned())
                .map(|s| s.to_owned()),
            values: {
                let values: Result<Vec<_>,_> = tree.children
                    .iter()
                    .filter(|t| t.name == "enumeratedValue")
                    .enumerate()
                    .map(|(e,t)| EnumeratedValue::parse(t).context(SVDErrorKind::Other(format!("Parsing enumerated value #{}", e).into())))
                    .collect();
                values?
            },
            _extensible: (),
        })
    }
}

impl Encode for EnumeratedValues {
    type Error = SVDError;

    fn encode(&self) -> Result<Element, SVDError> {
        let mut base = Element {
            name: String::from("enumeratedValues"),
            attributes: HashMap::new(),
            children: Vec::new(),
            text: None,
        };

        match self.name {
            Some(ref d) => {
                base.children.push(new_element("name", Some((*d).clone())));
            }
            None => (),
        };

        match self.usage {
            Some(ref v) => {
                base.children.push(v.encode()?);
            }
            None => (),
        };

        match self.derived_from {
            Some(ref v) => {
                base.attributes.insert(
                    String::from("derivedFrom"),
                    (*v).clone(),
                );
            }
            None => (),
        }

        for v in &self.values {
            base.children.push(v.encode()?);
        }

        Ok(base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::test;

    #[test]
    fn decode_encode() {
        let tests = vec![
            (
                EnumeratedValues {
                    name: None,
                    usage: None,
                    derived_from: Some(String::from("fake-derivation.png")),
                    values: vec![
                        EnumeratedValue {
                            name: String::from("WS0"),
                            description: Some(String::from(
                                "Zero wait-states inserted in fetch or read transfers",
                            )),
                            value: Some(0),
                            is_default: Some(true),
                            _extensible: (),
                        },
                        EnumeratedValue {
                            name: String::from("WS1"),
                            description: Some(String::from(
                                "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details",
                            )),
                            value: Some(1),
                            is_default: None,
                            _extensible: (),
                        },
                    ],
                    _extensible: (),
                },
                "
                    <enumeratedValues derivedFrom=\"fake-derivation.png\">
                        <enumeratedValue>
                            <name>WS0</name>
                            <description>Zero wait-states inserted in fetch or read transfers</description>
                            <value>0x00000000</value>
                            <isDefault>true</isDefault>
                        </enumeratedValue>
                        <enumeratedValue>
                            <name>WS1</name>
                            <description>One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details</description>
                            <value>0x00000001</value>
                        </enumeratedValue>
                    </enumeratedValues>
                "
            )
        ];

        test::<EnumeratedValues>(&tests[..]);
    }
}
