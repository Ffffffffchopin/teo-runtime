use key_path::path;
use teo_teon::Value;
use crate::request;
use crate::response::Response;
use crate::action::action::*;
use crate::connection::transaction;
use crate::model::object::object::ErrorIfNotFound;

pub async fn delete(req_ctx: &request::Ctx) -> teo_result::Result<Response> {
    let model = req_ctx.namespace().model_at_path(&req_ctx.handler_match().path()).unwrap();
    let action = DELETE | ENTRY | SINGLE;
    let value: Value = req_ctx.transaction_ctx().run_transaction(|ctx: transaction::Ctx| async move {
        let object = ctx.find_unique_internal(model, req_ctx.body(), true, action, Some(req_ctx.clone()), path![]).await.into_not_found_error(path![])?;
        object.delete_internal(path!["delete"]).await?;
        Ok(object.to_teon_internal(&path!["data"]).await?)
    }).await?;
    Ok(Response::data(value))
}
