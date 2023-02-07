use hdi::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct Sample {
    pub data: String,
}

app_entry!(Sample);

#[derive(Clone, Debug, Serialize, Deserialize, SerializedBytes)]
pub struct SamplePrivate {
    pub data: String,
}

app_entry!(SamplePrivate);

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def(visibility = "public")]
    Sample(Sample),
    #[entry_def(visibility = "private")]
    SamplePrivate(SamplePrivate),
}
