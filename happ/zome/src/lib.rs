use chrono::{DateTime, NaiveDateTime, Utc};
use hdk::prelude::*;

use integrity::{EntryTypes, Sample, SamplePrivate};

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}

pub fn get_now() -> DateTime<Utc> {
    match sys_time() {
        Ok(time) => {
            let now = time.as_seconds_and_nanos();
            let out = DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp_opt(now.0, now.1).unwrap(),
                Utc,
            );
            out
        }
        Err(_err) => Utc::now(),
    }
}

#[hdk_extern]
fn call(data: String) -> ExternResult<i64> {
    let now = get_now();
    let entry = EntryTypes::Sample(Sample { data: data.clone() });
    create_entry(entry)?;
    let after = get_now();
    let diff = (after - now).num_microseconds().unwrap();
    //debug!("call: {} microseconds", diff);
    Ok(diff)
}

#[hdk_extern]
fn call_private_entry(data: String) -> ExternResult<i64> {
    let now = get_now();
    let entry = EntryTypes::SamplePrivate(SamplePrivate { data: data.clone() });
    create_entry(entry)?;
    let after = get_now();
    let diff = (after - now).num_microseconds().unwrap();
    //debug!("call_private_entry: {} microseconds", diff);
    Ok(diff)
}

#[hdk_extern]
fn fetch_last_5_from_source_chain(_: ()) -> ExternResult<i64> {
    let now = get_now();

    let _query = query(
        QueryFilter::new()
            .entry_type(EntryType::App(AppEntryDef {
                entry_index: 1.into(),
                zome_index: 0.into(),
                visibility: EntryVisibility::Private,
            }))
            .include_entries(true)
            .descending(),
    );

    let after = get_now();
    let diff = (after - now).num_milliseconds();
    Ok(diff)
}
