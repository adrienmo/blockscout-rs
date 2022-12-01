//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "chart_value_type")]
pub enum ChartValueType {
    #[sea_orm(string_value = "DOUBLE")]
    Double,
    #[sea_orm(string_value = "INT")]
    Int,
}
