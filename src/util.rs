use napi::{bindgen_prelude::*, JsUnknown};
use netidx::protocol::value::Value;

pub(crate) fn js_of_value(env: &mut Env, v: &Value) -> Result<JsUnknown> {
    match v {
        Value::U32(u) | Value::V32(u) => Ok(env.create_uint32(*u)?.into_unknown()),
        Value::I32(i) | Value::Z32(i) => Ok(env.create_int32(*i)?.into_unknown()),
        Value::U64(u) | Value::V64(u) => Ok(env.create_int64(*u as i64)?.into_unknown()),
        Value::I64(i) | Value::Z64(i) => Ok(env.create_int64(*i)?.into_unknown()),
        Value::F32(f) => Ok(env.create_double(*f as f64)?.into_unknown()),
        Value::F64(f) => Ok(env.create_double(*f as f64)?.into_unknown()),
        Value::String(s) => Ok(env.create_string(s)?.into_unknown()),
        Value::Bytes(b) => {
            let mut buf = env.create_buffer(b.len())?;
            buf.copy_from_slice(&*b);
            Ok(buf.into_unknown())
        }
        Value::True => Ok(env.get_boolean(true)?.into_unknown()),
        Value::False => Ok(env.get_boolean(false)?.into_unknown()),
        Value::Ok => Ok(env.create_symbol(Some("Ok"))?.into_unknown()),
        Value::Error(e) => {
            // CR estokes: do something better than this
            Ok(env.create_string(&*e)?.into_unknown())
        }
        Value::Array(a) => {
            let mut jsa = env.create_array(a.len() as u32)?;
            for v in &**a {
                jsa.insert(js_of_value(env, v)?)?
            }
            Ok(jsa.coerce_to_object()?.into_unknown())
        }
        Value::Null => Ok(env.get_null()?.into_unknown()),
        Value::Duration(d) => {
            // CR estokes: do something better
            let s = format!("{}s", d.as_secs_f64());
            Ok(env.create_string(&s)?.into_unknown())
        }
        Value::DateTime(d) => {
            // CR estokes: do something better
            let s = d.to_rfc3339();
            Ok(env.create_string(&s)?.into_unknown())
        },
    }
}
