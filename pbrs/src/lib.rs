#![allow(unused_variables)]
#![allow(dead_code)]

use pbni::{pbx::*, prelude::*, syslib};

struct RustObject {}

#[nonvisualobject(name = "n_cst_test")]
impl RustObject {
    #[constructor]
    fn new(session: Session, ctx: ContextObject) -> RustObject { RustObject {} }

    #[method(name = "of_Array")]
    fn of_array(&mut self, mut arg: Array) -> Result<String> {
        arg.set_item_long(&[10], 12333223)?;
        let mut s = String::new();
        for item in arg.iter::<pblong>() {
            s += &format!("item: {:?}\n", item);
        }
        Ok(s)
    }
    #[method(name = "of_Invoke")]
    fn of_invoke(&mut self, mut obj: Object) -> Result<String> {
        let rv = obj.invoke_method("of_Test", pbargs!["abcd", 123])?;
        Ok(rv)
    }
}

struct ParentObject {
    session: Session,
    ctx: ContextObject,
    foo: Option<PBString>
}

impl ParentObject {
    fn context(&self) -> &ContextObject { &self.ctx }
    fn context_mut(&mut self) -> &mut ContextObject { &mut self.ctx }
}

#[nonvisualobject(name = "n_cst_parent")]
impl ParentObject {
    #[constructor]
    fn new_pbobject(session: Session, ctx: ContextObject) -> ParentObject {
        ParentObject {
            session,
            ctx,
            foo: None
        }
    }
    #[method(overload = 1)]
    fn of_test<'a>(&mut self, session: Session, a: &'a PBStr, b: Option<&'a PBStr>) -> Result<&'a PBStr> {
        let invoker = session.begin_invoke_function(("MessageBox", "ISS"))?;
        invoker.arg(0).set_str("title")?;
        invoker.arg(1).set_str("content")?;
        invoker.invoke()?;
        Ok(if let Some(b) = b {
            b
        } else {
            a
        })
    }
    #[method(name = "of_hello", overload = 1)]
    fn hello(&self, arg: String, b: Option<String>) -> String { format!("hello {},{:?}", arg, b) }
    //fn of_hello2(&mut self, a: String, b: String) -> String { format!("hello {}, {}", a, b) }
    #[method(name = "of_foo")]
    fn of_foo(&self, obj: &Self) -> Result<String> { Ok(format!("fooxxx {:?}", obj.foo)) }
    #[method(name = "of_SetFoo")]
    fn of_setfoo(&mut self, arg: &PBStr) -> bool {
        self.foo = Some(arg.to_owned());
        true
    }
    #[method(name = "of_trigger")]
    fn trigger(&mut self, arg: &PBStr) -> Result<String> {
        self.ontest(arg)?;
        let eid = self.ctx.get_event_id(("ontest", "LS"));
        let mid = self.ctx.get_method_id("of_test");
        Ok(format!("eid: {:?}, mid: {:?}", eid, mid))
    }
    #[event(name = "ontest")]
    fn ontest(&mut self, arg: &PBStr) -> Result<pblong> {}
}

struct ChildObject {
    parent: ParentObject
}

#[nonvisualobject(name = "n_cst_child", inherit = "parent")]
impl ChildObject {
    #[constructor]
    fn new_pbobject(session: Session, ctx: ContextObject) -> ChildObject {
        ChildObject {
            parent: ParentObject {
                session,
                ctx,
                foo: None
            }
        }
    }
    #[method(name = "of_child_hello")]
    fn of_hello(&self, arg: String) -> Result<String> { Ok(format!("child hello {}", arg)) }
}

#[global_function(name = "gf_bitor")]
fn bit_or(session: Session, a: pblong, b: pblong) -> pblong { a | b }

#[global_function(name = "gf_Test")]
fn global_function_test(
    session: Session,
    a: &PBStr,
    b: NaiveDate,
    c: NaiveTime,
    d: NaiveDateTime,
    e: Decimal,
    f: &[u8]
) -> Result<()> {
    let a = a.to_string_lossy();
    let b = b.to_string();
    let c = c.to_string();
    let d = d.to_string();
    let e = e.to_string();
    let f = String::from_utf8_lossy(f).into_owned();

    let mut obj = session.new_object("n_cst_pbtest")?;
    obj.set_var_str("is_test", "我爱RUST")?;
    let is_test = obj.get_var_string("is_test");
    let invoker = obj.begin_invoke_method("of_test")?;
    invoker.arg(0).set_str("call from rust to")?;
    let rv = invoker.invoke()?.get_string();

    Ok(())
}
