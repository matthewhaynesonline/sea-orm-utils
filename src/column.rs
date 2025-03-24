use async_trait::async_trait;
use chrono::Utc;
use sea_orm::prelude::*;
use uuid::Uuid;

// TODO: adjust these for different DBs? E.g. UUID type for postgres, etc.

#[async_trait]
pub trait HasUuid {
    fn set_uuid(&mut self, uuid: String);
}

#[macro_export]
macro_rules! impl_has_uuid {
    ($model:ty) => {
        #[async_trait::async_trait]
        impl HasUuid for $model {
            fn set_uuid(&mut self, uuid: String) {
                self.uuid = Set(uuid);
            }
        }
    };
}

pub async fn apply_uuid<C, U>(model: U, _db: &C, insert: bool) -> Result<U, DbErr>
where
    C: ConnectionTrait,
    U: HasUuid,
{
    let mut this = model;

    if insert {
        this.set_uuid(Uuid::new_v4().to_string());
    }

    Ok(this)
}

#[async_trait]
pub trait HasTimestamps {
    fn set_created_at(&mut self, date: DateTimeUtc);
    fn set_updated_at(&mut self, date: DateTimeUtc);
}

#[macro_export]
macro_rules! impl_has_timestamps {
    ($model:ty) => {
        #[async_trait::async_trait]
        impl HasTimestamps for $model {
            fn set_created_at(&mut self, date: DateTimeUtc) {
                self.created_at = Set(date);
            }

            fn set_updated_at(&mut self, date: DateTimeUtc) {
                self.updated_at = Set(date);
            }
        }
    };
}

pub async fn apply_timestamps<C, T>(model: T, _db: &C, insert: bool) -> Result<T, DbErr>
where
    C: ConnectionTrait,
    T: HasTimestamps,
{
    let mut this = model;
    let now_date = Utc::now();

    if insert {
        this.set_created_at(now_date);
    }

    this.set_updated_at(now_date);

    Ok(this)
}

#[macro_export]
macro_rules! impl_has_uuid_and_timestamps {
    ($model:ty) => {
        #[async_trait::async_trait]
        impl HasUuid for $model {
            fn set_uuid(&mut self, uuid: String) {
                self.uuid = Set(uuid);
            }
        }

        #[async_trait::async_trait]
        impl HasTimestamps for $model {
            fn set_created_at(&mut self, date: DateTimeUtc) {
                self.created_at = Set(date);
            }

            fn set_updated_at(&mut self, date: DateTimeUtc) {
                self.updated_at = Set(date);
            }
        }
    };
}

pub async fn apply_uuid_and_timestamps<C, T>(model: T, _db: &C, insert: bool) -> Result<T, DbErr>
where
    C: ConnectionTrait,
    T: HasUuid + HasTimestamps,
{
    let mut this = model;
    let now_date = Utc::now();

    if insert {
        this.set_uuid(Uuid::new_v4().to_string());
        this.set_created_at(now_date);
    }

    this.set_updated_at(now_date);

    Ok(this)
}
