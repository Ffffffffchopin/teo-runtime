use teo_parser::ast::info_provider::InfoProvider;
use teo_parser::ast::schema::Schema;
use crate::arguments::Arguments;

pub fn fetch_decorator_arguments<I>(decorator: &teo_parser::ast::decorator::Decorator, schema: &Schema, info_provider: I) -> Arguments where I: InfoProvider {
    unreachable!()
}