use std::collections::HashMap;

use crate::prelude::*;

use super::{member, role::log_check};

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

async fn members() -> Result<(), MemberLogError> {
    let prisma = prisma::create().await?;

    let members: HashMap<serenity::UserId, serenity::Member> = todo!("Comms: get members");

    let prisma_members = prisma
        .user()
        .find_many(vec![])
        .exec()
        .await?
        .into_iter()
        .map(|n| (n.id, n))
        .collect::<HashMap<_, _>>();

    for (id, member) in members {
        if let Some(prisma_member) = prisma_members.get(&id.into()) {
            let mut updates = vec![];

            if member.user.name != prisma_member.username {
                updates.push(user::username::set(&member.user.name));
            }

            if member.nick != prisma_member.nickname {
                updates.push(user::nickname::set(member.nick));
            }

            if member.user.bot != prisma_member.bot {
                updates.push(user::bot::set(member.user.bot));
            }

            if member.user.bot && !member.roles.contains(&nci::roles::BOTS) {
                todo!("Comms: add bot role to user");
            }

            if !member.user.bot && member.roles.contains(&nci::roles::BOTS) {
                todo!("Comms: remove bot role from user");
            }

            let has_overrides = member.roles.contains(&nci::roles::OVERRIDES);
            let has_members = member.roles.contains(&nci::roles::MEMBERS);

            if has_overrides != prisma_member.admin {
                updates.push(user::admin::set(has_overrides));
            }

            if has_members != prisma_member.verified {
                updates.push(user::verified::set(has_members));
            }

            let mut role_where = vec![];
            for role in member.roles {
                if log_check(role).is_err() {
                    continue;
                }

                role_where.push(role::id::equals(role));
            }
            updates.push(user::roles::set(role_where));

            if !updates.is_empty() {
                prisma
                    .user()
                    .update(user::id::equals(id), updates)
                    .exec()
                    .await?;
            }
        } else {
            let role: serenity::Role = todo!("Comms: create color role role"); // will send role_create event, no need to log separately

            let all_roles = member
                .roles
                .iter()
                .map(|n| role::id::equals(*n))
                .chain(vec![role::id::equals(role.id)])
                .collect_vec();

            prisma
                .user()
                .create(
                    member.user.id,
                    &member.user.name,
                    vec![
                        user::nickname::set(member.nick),
                        user::bot::set(member.user.bot),
                        user::roles::set(all_roles),
                        user::admin::set(member.roles.contains(&nci::roles::OVERRIDES)),
                        user::verified::set(member.roles.contains(&nci::roles::MEMBERS)),
                    ],
                )
                .exec()
                .await?;
        }
    }

    Ok(())
}
