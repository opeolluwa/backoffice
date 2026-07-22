use sea_orm::rbac::{RbacAddRoleHierarchy, RbacContext};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let mut context = RbacContext::load(db).await?;
        context
            .add_roles(
                db,
                &["admin", "super_admin", "public", "manager", "team_member"],
            )
            .await?;

        context
            .add_role_hierarchy(
                db,
                &[
                    RbacAddRoleHierarchy {
                        super_role: "super_admin",
                        role: "admin",
                    },
                    RbacAddRoleHierarchy {
                        super_role: "admin",
                        role: "manager",
                    },
                    RbacAddRoleHierarchy {
                        super_role: "manager",
                        role: "team_member",
                    },
                    RbacAddRoleHierarchy {
                        super_role: "team_member",
                        role: "public",
                    },
                ],
            )
            .await?;

        // Super roles
        for role in ["super_admin", "admin", "manager", "team_member"] {
            context
                .add_role_permissions(db, role, &["insert", "update", "delete"], &["*"])
                .await?;
        }

        // Public permissions
        // context
        //     .add_role_permissions(
        //         db,
        //         "public",
        //         &["select"],
        //         &["countries", "products", "uploads"],
        //     )
        //     .await?;

        // context
        //     .add_role_permissions(db, "public", &["insert"], &["emails", "newsletters"])
        //     .await?;

        // Additional team member permissions
        // context
        //     .add_role_permissions(
        //         db,
        //         "team_member",
        //         &["insert", "select"],
        //         &["emails", "newsletters"],
        //     )
        //     .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
