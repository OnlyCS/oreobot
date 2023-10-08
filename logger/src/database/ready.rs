use std::collections::HashMap;

use crate::prelude::*;

async fn roles() -> Result<(), RoleLogError> {
    let prisma = prisma::create().await?;

    let roles: HashMap<serenity::RoleId, serenity::Role> = todo!("Comms: Get list of roles");

    let prisma_roles = prisma
        .role()
        .find_many(vec![])
        .exec()
        .await?
        .into_iter()
        .map(|n| (n.id, n))
        .collect::<HashMap<_, _>>();

    // for every role in the database
    for (id_i64, prisma_role) in prisma_roles {
        if !nci::roles::can_log(id_i64) {
            continue;
        }

        // if the role does not exist in the guild
        let id = serenity::RoleId::new(id_i64 as u64);

        if let Some(role) = roles.get(&id) {
            let mut updates = vec![];

            if Color::from(role.colour) != Color::from_hex(prisma_role.color)? {
                updates.push(role::color::set(Color::from(role.colour).to_raw_hex()));
            }

            if role.name != prisma_role.name {
                updates.push(role::name::set(role.name.clone()));
            }

            if !updates.is_empty() {
                prisma
                    .role()
                    .update(role::id::equals(id), updates)
                    .exec()
                    .await?;
            }
        } else {
            prisma
                .role()
                .update(role::id::equals(id_i64), vec![role::deleted::set(true)])
                .exec()
                .await?;
        }
    }

    // for every role in the guild
    for (id, role) in roles {
        if !nci::roles::can_log(id) {
            continue;
        }

        // custom roles can only be managed when the bot is up, therefore, we don't have to catch up
        if todo!("Comms: check role custom role") {
            continue;
        }

        if !prisma_roles.contains_key(&id.into()) {
            prisma
                .role()
                .create(
                    id,
                    &role.name,
                    Color::from(role.colour).to_raw_hex(),
                    vec![role::color_role::set(nci::roles::is_color_role(id))],
                )
                .exec()
                .await?;
        }
    }

    Ok(())
}
