use axum::Extension;
use prisma::prisma;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Context {
    pub db: Arc<prisma::PrismaClient>,
}

pub type Ctx = Extension<Context>;
