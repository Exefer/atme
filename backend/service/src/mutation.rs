use ::entity::{charge_records, charge_records::Entity as ChargeRecord};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_charge_record(
        db: &DbConn,
        start_percentage: i32,
    ) -> Result<charge_records::ActiveModel, DbErr> {
        charge_records::ActiveModel {
            start_percentage: Set(start_percentage),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn update_last_charge_record(
        db: &DbConn,
        end_percentage: i32,
    ) -> Result<charge_records::Model, DbErr> {
        let charge_record: charge_records::ActiveModel = ChargeRecord::find()
            .order_by_desc(charge_records::Column::Id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find charge record".to_owned()))?
            .into();

        charge_records::ActiveModel {
            id: charge_record.id,
            start_percentage: charge_record.start_percentage,
            start_timestamp: charge_record.start_timestamp,
            end_percentage: Set(Some(end_percentage)),
            end_timestamp: Set(Some(chrono::Local::now().naive_local())),
        }
        .update(db)
        .await
    }

    pub async fn delete_charge_record(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let charge_record: charge_records::ActiveModel = ChargeRecord::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find charge record".to_owned()))?
            .into();

        charge_record.delete(db).await
    }
}
