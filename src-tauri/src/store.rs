use crate::prelude::*;
use crate::utils::{map, XTake, XTakeVal};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Array, Datetime, Object, Value};
use surrealdb::{Datastore, Session};

mod try_froms;
mod x_takes;

pub trait Createable: Into<Value> {}
pub trait Patchable: Into<Value> {}
pub trait Filterable: Into<Value> {}

pub struct Store {
    ds: Datastore,
    ses: Session,
}

impl Store {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            ds: Datastore::new("memory").await?,
            ses: Session::for_db("appns", "appdb"),
        })
    }

    pub async fn get(&self, tid: &str) -> Result<Object> {
        let sql = "SELECT * FROM $th";

        let vars = map!["th".into() => thing(tid)?.into()];
        let ress = self.ds.execute(sql, &self.ses, Some(vars), true).await?;
        let first_res = ress.into_iter().next().expect("Did not get a response");
        W(first_res.result?.first()).try_into()
    }

    pub async fn create<T: Createable>(&self, tb: &str, data: T) -> Result<String> {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN id";

        let mut data: Object = W(data.into()).try_into()?;
        let now = Datetime::default().timestamp_nanos();
        data.insert("ctime".into(), now.into());

        let vars = map![
            "tb".into() => tb.into(),
            "data".into() => Value::from(data)
        ];
        let ress = self.ds.execute(sql, &self.ses, Some(vars), false).await?;
        let first_val = ress
            .into_iter()
            .next()
            .map(|r| r.result)
            .expect("id not returned")?;

        if let Value::Object(mut val) = first_val.first() {
            val.x_take_val::<String>("id")
                .map_err(|err| Error::StoreFailToCreate(f!("exec_create {tb} {err}")))
        } else {
            Err(Error::StoreFailToCreate(f!(
                "exec_create {tb}, nothing returned."
            )))
        }
    }

    pub async fn merge<T: Patchable>(&self, tid: &str, data: T) -> Result<String> {
        let sql = "UPDATE $th MERGE $data RETURN id";
        let vars = map![
            "th".into() => thing(tid)?.into(),
            "data".into() => data.into()
        ];

        let ress = self.ds.execute(sql, &self.ses, Some(vars), true).await?;
        let first_res = ress.into_iter().next().expect("id not returned");
        let result = first_res.result?;

        if let Value::Object(mut val) = result.first() {
            val.x_take_val("id")
        } else {
            Err(Error::StoreFailToCreate(f!(
                "exec_merge {tid}, nothing returned."
            )))
        }
    }

    pub async fn delete(&self, tid: &str) -> Result<String> {
        let sql = "DELETE $th";

        let vars = map!["th".into() => thing(tid)?.into()];
        let ress = self.ds.execute(sql, &self.ses, Some(vars), false).await?;
        let first_res = ress.into_iter().next().expect("Did not get a response");

        // Return the error if result failed
        first_res.result?;

        // return Success
        Ok(tid.to_string())
    }

    pub async fn select(&self, tb: &str, filter: Option<Value>) -> Result<Vec<Object>> {
        let mut sql = String::from("SELECT * FROM type::table($tb)");

        let mut vars = BTreeMap::from([("tb".into(), tb.into())]);

        // --- Apply the filter
        if let Some(filter) = filter {
            let obj: Object = W(filter).try_into()?;
            sql.push_str(" WHERE");
            for (idx, (k, v)) in obj.into_iter().enumerate() {
                let var = f!("w{idx}");
                sql.push_str(&f!(" {k} = ${var}"));
                vars.insert(var, v);
            }
        }

        // --- Apply the orderby
        sql.push_str(" ORDER ctime DESC");

        let ress = self.ds.execute(&sql, &self.ses, Some(vars), false).await?;
        let first_res = ress.into_iter().next().expect("Did not get a response");
        let array: Array = W(first_res.result?).try_into()?;

        // build the list of objects
        array.into_iter().map(|v| W(v).try_into()).collect()
    }
}
