use std::borrow::Cow;

use mysql_common::row::convert::{FromRow, FromRowError};
use mysql_common::row::Row;
use mysql_common::value::Value;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct User<'name, 'password> {
    pub created_at: chrono::NaiveDateTime,
    pub id: uuid::Uuid,
    pub name: Cow<'name, str>,
    pub password: Cow<'password, str>,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Tracker<'name> {
    pub created_at: chrono::NaiveDateTime,
    pub id: uuid::Uuid,
    pub name: Cow<'name, str>,
    pub updated_at: chrono::NaiveDateTime,
    pub user_id: uuid::Uuid,
    pub views: u64,
}

impl<'name, 'password> FromRow for User<'name, 'password> {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        if row.len() < 4 {
            return Err(FromRowError(row));
        }

        let mut user = User::default();

        for (index, column) in row.columns_ref().iter().enumerate() {
            match column.name_str().as_ref() {
                "created_at" => {
                    let value = row.as_ref(index);

                    if let Some(Value::UInt(timestamp)) = value {
                        user.created_at =
                            chrono::naive::NaiveDateTime::from_timestamp(*timestamp as i64, 0)
                    } else {
                        return Err(FromRowError(row));
                    }
                }
                "id" => {
                    let value = row.as_ref(index);

                    if let Some(Value::Bytes(bytes)) = value {
                        if let Some(uuid) = get_uuid_from_bytes(bytes) {
                            user.id = uuid;
                        } else {
                            return Err(FromRowError(row));
                        }
                    } else {
                        return Err(FromRowError(row));
                    }
                }
                "name" => {
                    let value = row.get(index);

                    if let Some(Value::Bytes(bytes)) = value {
                        if let Ok(name) = String::from_utf8(bytes) {
                            user.name = Cow::Owned(name)
                        } else {
                            return Err(FromRowError(row));
                        }
                    } else {
                        return Err(FromRowError(row));
                    }
                }
                "updated_at" => {
                    let value = row.as_ref(index);

                    if let Some(Value::UInt(timestamp)) = value {
                        user.updated_at =
                            chrono::naive::NaiveDateTime::from_timestamp(*timestamp as i64, 0)
                    } else {
                        return Err(FromRowError(row));
                    }
                }
                _ => {}
            }
        }

        Ok(user)
    }
}

fn get_uuid_from_bytes(val: &Vec<u8>) -> Option<uuid::Uuid> {
    if val.len() != 16 {
        return None;
    }

    Some(uuid::Uuid::from_bytes(
        <[u8; 16]>::try_from(val.as_slice()).unwrap(),
    ))
}
