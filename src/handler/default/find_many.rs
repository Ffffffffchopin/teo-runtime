use key_path::path;
use crate::request;
use teo_teon::{teon, Value};
use crate::action::action::*;
use crate::response::Response;
use crate::error_runtime_ext::ErrorRuntimeExt;

pub async fn find_many(ctx: &request::Ctx) -> teo_result::Result<Response> {
    let model = ctx.namespace().model_at_path(&ctx.handler_match().path()).unwrap();
    let action = FIND | MANY | ENTRY;
    let results = ctx.transaction_ctx().find_many_internal(
        model,
        ctx.body(),
        false,
        action,
        Some(ctx.clone()),
        path![],
    ).await?;
    let mut count_input = ctx.body().clone();
    let count_input_obj = count_input.as_dictionary_mut().unwrap();
    count_input_obj.remove("skip");
    count_input_obj.remove("take");
    count_input_obj.remove("pageSize");
    count_input_obj.remove("pageNumber");
    let count = ctx.transaction_ctx().count_objects(model, &count_input, path![]).await.unwrap();
    let mut meta = teon!({"count": count});
    let page_size = ctx.body().get("pageSize");
    if page_size.is_some() {
        let page_size = page_size.unwrap().to_int64().unwrap();
        let count = count as i64;
        let mut number_of_pages = count / page_size;
        if count % page_size != 0 {
            number_of_pages += 1;
        }
        meta.as_dictionary_mut().unwrap().insert("numberOfPages".to_string(), number_of_pages.into());
    }

    let mut result_json: Vec<Value> = vec![];
    for (index, result) in results.iter().enumerate() {
        match result.to_teon_internal(&path!["data", index]).await {
            Ok(result) => result_json.push(result),
            Err(_) => return Err(teo_result::Error::unauthorized_error(path!["data", index], "not allowed to read")),
        }
    }
    Ok(Response::data_meta(Value::Array(result_json), meta))
}
