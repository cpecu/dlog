use crate::{
    util,
    models::{
        Entry, Units, Action, Note, Item, fact::{Fact, AbstractFact},
        Attrib, Relation,
        Record,
    },
};
use comfy_table::{
    Table, ContentArrangement, presets::UTF8_BORDERS_ONLY,
    Cell, Attribute, Color as TColor,
};
use std::{
    path::PathBuf, fs,
    convert::TryFrom,
    rc::Rc,
    io::{Read, Write}
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub dir: PathBuf,
    pub records: RecordData,
    pub facts: FactData,
    pub items: ItemData,
    pub units: UnitsData,
    pub notes: NoteData,
    pub actions: ActionData,
    pub attribs: AttribData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordData {
    records: Vec<Record>
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FactData {
    facts: Vec<Fact>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NoteData {
    notes: Vec<Note>
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UnitsData {
    units: Vec<Units>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ActionData {
    actions: Vec<Action>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AttribData {
    attribs: Vec<Attrib>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ItemData {
    items: Vec<Item>,
}


impl Default for Data {
    fn default() -> Self {
        let dir = util::default_data_dir(None).expect("No valid data dir");
        Self {
            dir, ..Default::default()
        }
    }
}

impl Data {

    pub fn new() -> crate::DResult<Self> {
        Ok(Self::default())
    }

    pub fn records(&self) -> RecordData {
        let rec = self.dir.join("records.toml");
        RecordData::default()
    }

}

impl RecordData {

    pub fn path() -> crate::DResult<PathBuf> {
        let path = util::default_data_dir(None)?.join("records.toml");
        Ok(path)
    }

    pub fn read() -> crate::DResult<Self> {
        if !Self::path()?.exists() {
            Self::create()?;
        };
        let mut f = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open(Self::path()?)?;
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        let rd: RecordData = toml::from_str(buf.as_str())
            .expect("Could not parse recod data toml file");
        Ok(rd)
    }

    pub fn create() -> crate::DResult<Self> {
        fs::write(Self::path()?, toml::to_string(&Self::default()).expect("Could not parse toml to record data"))?;
        Ok(Self::default())

    }

}

impl Default for RecordData {
    fn default() -> Self {
        Self { records: vec![Record::default()] }
    }
}
