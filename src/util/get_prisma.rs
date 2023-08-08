macro_rules! from_serenity_context {
    ($var_name:ident, $ctx:expr) => {
        use crate::prelude::*;

        let data_locked = &$ctx.data;
        let data = data_locked.read().await;

        let $var_name = data
            .get::<PrismaTypeKey>()
            .context("Could not find prismaclient in data")?
            .lock()
            .await;
    };
}

macro_rules! from_poise_context {
    ($var_name:ident, $ctx:expr) => {
        use crate::prelude::*;

        let prisma_mutex = Arc::clone(&$ctx.data().prisma);
        let $var_name = prisma_mutex.lock().await;
    };
}

pub(crate) use from_poise_context;
pub(crate) use from_serenity_context;
