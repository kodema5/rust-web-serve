// https://github.com/actix/examples/blob/master/form/src/main.rs
use serde_json::json;
use r2d2_redis::redis::Commands;
use actix_web::{web, HttpRequest, HttpResponse};
pub fn index (
    state: web::Data<crate::State>,
    body: web::Json<serde_json::Value>,
    req: HttpRequest)
-> HttpResponse
{

    let payload = body.into_inner();
    if !payload.is_object() {
        return HttpResponse::Ok().json(json!({
            "error": "payload must be an object",
        }));
    }

    // replace .key = "name" with [name] = redis(.key)
    let input = {
        let mut js = json!({});
        let m = js.as_object_mut().unwrap();

        let mut redis = state.redis.get_connection();

        let p_map = payload.as_object().unwrap();

        for k in p_map.keys() {
            let ch = k.chars().nth(0).unwrap();
            if ch == '.' {
                let field_name = p_map.get(k).unwrap();
                if !field_name.is_string() { continue; }

                let value:String = redis.get(k).unwrap_or_default();
                if value.is_empty() { continue; }

                {
                    let n = field_name.as_str().unwrap().to_string();
                    let v = serde_json::from_str(&value).unwrap();
                    m.insert(n, v);
                }

            } else {
                let n = k.to_string();
                let v = payload.get(k).unwrap().clone();
                m.insert(n,v);
            }
        }
        js
    };


    let output = {
        let name = req.match_info().get("pg_function_name").unwrap();
        let sql = state.postgres.get_sql(name);

        let conn = state.postgres.get_connection();
        let rows =  &conn.query(&sql, &[&input]).unwrap();
        let row = rows.get(0);
        let a : serde_json::Value = row.get(0);
        a
    };


    // filters .key from output
    // sets redis(.key) = output[.key]
    let response = {
        let mut js = json!({});
        let m = js.as_object_mut().unwrap();

        let mut redis = state.redis.get_connection();

        for k in output.as_object().unwrap().keys() {
            let ch = k.chars().nth(0).unwrap();
            if ch == '.' {
                let a = output.get(k).unwrap();
                if a.is_null() {
                    let _:() = redis.del(k).unwrap();
                } else {
                    let _:() = redis.set(k, a.to_string()).unwrap();
                }
            } else {
                let n = k.to_string();
                let v = output.get(k).unwrap().clone();
                m.insert(n,v);
            }
        }
        js
    };

    HttpResponse::Ok().json(response)
}