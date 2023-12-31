//! `XTake` implementations for store related types
//!

use crate::prelude::*;
use crate::utils::XTakeImpl;
use surrealdb::sql::Object;

use chrono::{DateTime, Utc};

impl XTakeImpl<String> for Object {
    fn x_take_impl(&mut self, k: &str) -> Result<Option<String>> {
        let v = self.remove(k).map(|v| W(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(ex)) => Err(ex),
        }
    }
}

impl XTakeImpl<i64> for Object {
    fn x_take_impl(&mut self, k: &str) -> Result<Option<i64>> {
        let v = self.remove(k).map(|v| W(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(ex)) => Err(ex),
        }
    }
}

impl XTakeImpl<bool> for Object {
    fn x_take_impl(&mut self, k: &str) -> Result<Option<bool>> {
        Ok(self.remove(k).map(|v| v.is_true()))
    }
}

impl XTakeImpl<DateTime<Utc>> for Object {
    fn x_take_impl(&mut self, k: &str) -> Result<Option<DateTime<Utc>>> {
        let v = self.remove(k).map(|v| W(v).try_into());
        match v {
            None => Ok(None),
            Some(Ok(val)) => Ok(Some(val)),
            Some(Err(ex)) => Err(ex),
        }
    }
}
