use crate::services::{
    document::manager::DocumentManager,
    web_socket::{WSServer, WebSocketReceivers},
};
use actix::Addr;
use actix_web::web::Data;
use lib_ws::WSModule;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppContext {
    pub ws_server: Data<Addr<WSServer>>,
    pub pg_pool: Data<PgPool>,
    pub ws_receivers: Data<WebSocketReceivers>,
    pub document_mng: Data<Arc<DocumentManager>>,
}

impl AppContext {
    pub fn new(ws_server: Addr<WSServer>, db_pool: PgPool) -> Self {
        let ws_server = Data::new(ws_server);
        let pg_pool = Data::new(db_pool);

        let mut ws_receivers = WebSocketReceivers::new();
        let document_mng = Arc::new(DocumentManager::new(pg_pool.clone()));
        ws_receivers.set(WSModule::Doc, document_mng.clone());

        AppContext {
            ws_server,
            pg_pool,
            ws_receivers: Data::new(ws_receivers),
            document_mng: Data::new(document_mng),
        }
    }
}
