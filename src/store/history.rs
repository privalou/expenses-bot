use std::cell::RefCell;
use std::collections::BTreeMap;

use chrono::Utc;

use crate::store::StoreError;

#[derive(Debug, Clone, PartialEq)]
pub struct History {
    history: RefCell<InteriorState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InteriorState {
    records: BTreeMap<i64, ExpenseRecord>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpenseRecord {
    amount: f32,
    category: Option<String>,
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

impl History {
    pub fn new() -> History {
        History {
            history: RefCell::new(InteriorState {
                records: BTreeMap::new(),
            }),
        }
    }

    pub fn save_record(&self, expenses_record: ExpenseRecord) -> Result<i64, StoreError> {
        let time_stamp = Utc::now().timestamp();
        match self
            .history
            .borrow_mut()
            .records
            .insert(time_stamp, expenses_record)
        {
            None => Ok(time_stamp),
            Some(_) => Err(
                "Can not save record because there is already existing record"
                    .to_string()
                    .into(),
            ),
        }
    }

    pub fn get_record(&self, time_stamp: i64) -> Option<ExpenseRecord> {
        match self.history.borrow().records.get(&time_stamp) {
            Some(record) => Some(record.clone()),
            None => None,
        }
    }

    pub fn update_latest_record(&self, patch: ExpenseRecordPatch) -> Result<(), StoreError> {
        match self
            .history
            .borrow_mut()
            .records
            .range_mut(..Utc::now().timestamp())
            .next_back()
        {
            Some(latest_record) => {
                latest_record.1.category = patch.category;
                Ok(())
            }
            None => Err("Can not update latest record".to_string().into()),
        }
    }

    pub fn get_records_list(&self) -> Vec<(i64, ExpenseRecord)> {
        self.history
            .borrow()
            .records
            .iter()
            // todo: REMOVE CLONING!
            .map(|entry| (*entry.0, entry.1.clone()))
            .collect::<Vec<(i64, ExpenseRecord)>>()
    }
}

impl ExpenseRecord {
    pub fn new(amount: f32) -> Self {
        ExpenseRecord {
            amount,
            category: None,
        }
    }

    pub fn new_with(amount: f32, category: String) -> Self {
        ExpenseRecord {
            amount,
            category: Some(category),
        }
    }
}

pub struct ExpenseRecordPatch {
    category: Option<String>,
}

impl ExpenseRecordPatch {
    pub fn new(category: Option<String>) -> Self {
        ExpenseRecordPatch { category }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_record_saved_at_store() {
        let history = History::new();
        let key = history
            .save_record(ExpenseRecord::new(30.00))
            .expect("Can not save this record");
        let option_record = history.get_record(key);
        assert!(option_record.is_some());
        let record = option_record.unwrap();
        assert_eq!(record.amount, 30.00);
        assert_eq!(record.category, None);
    }

    #[test]
    fn get_records_list_should_return_value() {
        let history = History::new();
        let key = history
            .save_record(ExpenseRecord::new(30.00))
            .expect("Can not save this record");

        let vec = history.get_records_list();
        assert_eq!(vec.len(), 1);
        let tuple = vec.get(0).unwrap();
        assert_eq!(tuple.0, key);
        assert_eq!(tuple.1.category, None);
        assert_eq!(tuple.1.amount, 30.00);
    }

    #[test]
    fn get_records_list_should_return_empty_vec() {
        let history = History::new();

        let vec = history.get_records_list();
        assert!(vec.is_empty());
    }
}
