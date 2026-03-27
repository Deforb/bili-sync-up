use sea_orm::ConnectionTrait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Collection::Table)
                    .add_column(
                        ColumnDef::new(Collection::FolderMode)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Favorite::Table)
                    .add_column(
                        ColumnDef::new(Favorite::FolderMode)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Submission::Table)
                    .add_column(
                        ColumnDef::new(Submission::FolderMode)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(WatchLater::Table)
                    .add_column(
                        ColumnDef::new(WatchLater::FolderMode)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(VideoSource::Table)
                    .add_column(
                        ColumnDef::new(VideoSource::FolderMode)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        db.execute_unprepared("UPDATE collection SET folder_mode = CASE WHEN flat_folder THEN 1 ELSE 0 END")
            .await?;
        db.execute_unprepared("UPDATE favorite SET folder_mode = CASE WHEN flat_folder THEN 1 ELSE 0 END")
            .await?;
        db.execute_unprepared("UPDATE submission SET folder_mode = CASE WHEN flat_folder THEN 1 ELSE 0 END")
            .await?;
        db.execute_unprepared("UPDATE watch_later SET folder_mode = CASE WHEN flat_folder THEN 1 ELSE 0 END")
            .await?;
        db.execute_unprepared("UPDATE video_source SET folder_mode = CASE WHEN flat_folder THEN 1 ELSE 0 END")
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Collection::Table)
                    .drop_column(Collection::FolderMode)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Favorite::Table)
                    .drop_column(Favorite::FolderMode)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Submission::Table)
                    .drop_column(Submission::FolderMode)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(WatchLater::Table)
                    .drop_column(WatchLater::FolderMode)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(VideoSource::Table)
                    .drop_column(VideoSource::FolderMode)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Collection {
    Table,
    FolderMode,
}

#[derive(DeriveIden)]
enum Favorite {
    Table,
    FolderMode,
}

#[derive(DeriveIden)]
enum Submission {
    Table,
    FolderMode,
}

#[derive(DeriveIden)]
enum WatchLater {
    Table,
    FolderMode,
}

#[derive(DeriveIden)]
enum VideoSource {
    Table,
    FolderMode,
}
