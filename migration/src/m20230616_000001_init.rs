use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Company::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Company::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Company::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Department::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Department::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Department::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(JobTitle::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(JobTitle::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(JobTitle::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Country::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Country::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Country::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PostalCode::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PostalCode::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PostalCode::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(State::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(State::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(State::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(City::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(City::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(City::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(StreetAddress::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StreetAddress::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(StreetAddress::Name).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Contact::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Contact::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Contact::Name).string().not_null())
                    .col(ColumnDef::new(Contact::DN).string().not_null())
                    .col(ColumnDef::new(Contact::Photo).boolean().not_null())
                    .col(ColumnDef::new(Contact::EmployeeNumber).string())
                    .col(ColumnDef::new(Contact::Mail).string())
                    .col(ColumnDef::new(Contact::TelephoneNumber).string())
                    .col(ColumnDef::new(Contact::Mobile).string())
                    .col(ColumnDef::new(Contact::Birthdate).string())
                    .col(
                        ColumnDef::new(Contact::CompanyId)
                            .integer()
                            .default(Value::Int(None)),
                    )
                    .col(
                        ColumnDef::new(Contact::DepartmentId)
                            .integer()
                            .default(Value::Int(None)),
                    )
                    .col(
                        ColumnDef::new(Contact::JobTitleId)
                            .integer()
                            .default(Value::Int(None)),
                    )
                    .col(
                        ColumnDef::new(Contact::CountryId)
                            .integer()
                            .default(Value::Int(None)),
                    )
                    .col(
                        ColumnDef::new(Contact::PostalCodeId)
                            .integer()
                            .default(Value::Int(None)),
                    )
                    .col(
                        ColumnDef::new(Contact::StateId)
                            .integer()
                            .default(Value::Int(None)),
                    )
                    .col(
                        ColumnDef::new(Contact::CityId)
                            .integer()
                            .default(Value::Int(None)),
                    )
                    .col(
                        ColumnDef::new(Contact::StreetAddressId)
                            .integer()
                            .default(Value::Int(None)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Contact::Table, Contact::CompanyId)
                            .to(Company::Table, Company::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Contact::Table, Contact::DepartmentId)
                            .to(Department::Table, Department::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Contact::Table, Contact::JobTitleId)
                            .to(JobTitle::Table, JobTitle::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Contact::Table, Contact::CountryId)
                            .to(Country::Table, Country::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Contact::Table, Contact::PostalCodeId)
                            .to(PostalCode::Table, PostalCode::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Contact::Table, Contact::StateId)
                            .to(State::Table, State::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Contact::Table, Contact::CityId)
                            .to(City::Table, City::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Contact::Table, Contact::StreetAddressId)
                            .to(StreetAddress::Table, StreetAddress::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Contact::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Company::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Department::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(JobTitle::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Country::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(PostalCode::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(State::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(City::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(StreetAddress::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
pub enum Company {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum Department {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum JobTitle {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum Country {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum PostalCode {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum State {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum City {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum StreetAddress {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
pub enum Contact {
    Table,
    Id,
    Name,
    DN,
    EmployeeNumber,
    Mail,
    TelephoneNumber,
    Mobile,
    Photo,
    Birthdate,
    CompanyId,
    DepartmentId,
    JobTitleId,
    CountryId,
    PostalCodeId,
    StateId,
    CityId,
    StreetAddressId,
}
