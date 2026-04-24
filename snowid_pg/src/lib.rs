use pgrx::prelude::*;

//import core function
use snowid::generate_id;

pgrx::pg_module_magic!();

//sql function
#[pg_extern]
fn snowid() -> i64 {
    generate_id() as i64
}
