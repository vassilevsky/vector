use super::Transform;
use crate::Event;
use serde::{Deserialize, Serialize};
use string_cache::DefaultAtom as Atom;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FieldFilterConfig {
    pub field: String,
    pub value: String,
}

#[typetag::serde(name = "field_filter")]
impl crate::topology::config::TransformConfig for FieldFilterConfig {
    fn build(&self) -> Result<Box<dyn Transform>, String> {
        Ok(Box::new(FieldFilter::new(
            self.field.clone(),
            self.value.clone(),
        )))
    }
}

pub struct FieldFilter {
    field_name: Atom,
    value: String,
}

impl FieldFilter {
    pub fn new(field_name: String, value: String) -> Self {
        Self {
            field_name: field_name.into(),
            value,
        }
    }
}

impl Transform for FieldFilter {
    fn transform(&self, record: Event) -> Option<Event> {
        if record
            .as_log()
            .get(&self.field_name)
            .map(|f| f.as_bytes())
            .map_or(false, |b| b == self.value.as_bytes())
        {
            Some(record)
        } else {
            None
        }
    }
}
