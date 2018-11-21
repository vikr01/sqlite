#[macro_use]
extern crate neon;
extern crate rusqlite;

use rusqlite::Connection;
use neon::prelude::*;

fn version(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(rusqlite::version::version()))
}

pub struct Sqlite {
    pub conn: Option<Connection>,
    pub database: Option<String>,
    pub verbose: Option<bool>
}

declare_types! {
    pub class JsSqlite for Sqlite {
        init(_cx) {
            Ok(Sqlite {
                conn: None,
                database: None,
                verbose: None
            })
        }

        method create(mut cx) {
            let database_value: String;
            let verbose_value: bool;

            match cx.argument_opt(0) {
                Some(arg) => {
                    let obj = arg.downcast::<JsObject>().or_throw(&mut cx)?;
                    // Handle verbose property defaults
                    if obj.get(&mut cx, "verbose")?.is_a::<JsUndefined>() {
                        verbose_value = true;
                    } else {
                        verbose_value = obj.get(&mut cx, "verbose")?.downcast::<JsBoolean>().or_throw(&mut cx)?.value();
                    }
                    // Handle database property defaults
                    if obj.get(&mut cx, "database")?.is_a::<JsUndefined>() {
                        database_value = ":memory:".to_string();
                    } else {
                        database_value = obj.get(&mut cx, "database")?.downcast::<JsString>().or_throw(&mut cx)?.value();
                    }
                },
                None => {
                    database_value = ":memory:".to_string();
                    verbose_value = true;
                }
            }

            let conn = if database_value == ":memory:".to_string() {
                Connection::open_in_memory().unwrap();
            } else {
                Connection::open(&database_value).unwrap();
            };

            let this = cx.this();
            let js_database_value = cx.string(database_value);
            let js_verbose_value = cx.boolean(verbose_value);
            this.set(&mut cx, "database", js_database_value)?;
            this.set(&mut cx, "verbose", js_verbose_value)?;

            Ok(this.upcast())
        }

        method execute() {

        }

        method statement(mut cx) {
            Ok(cx.undefined().upcast())
        }
    }
}

register_module!(mut m, {
    m.export_class::<JsSqlite>("Sqlite")?;
    m.export_function("version", version)?;
    Ok(())
});
