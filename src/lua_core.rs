use hlua::Lua;

pub struct ScriptContext<'a> {
    context: Lua<'a>
}

impl<'a> ScriptContext<'a> {
    fn new() -> ScriptContext<'a> {
        let lua = Lua::new();
        ScriptContext {context: lua}
    }
}

#[test]
fn test_script_context_init() {
    let mut context = ScriptContext::new();
    context.context.execute::<()>("x = 3").unwrap();
    let x: i32 = context.context.get("x").unwrap();
    assert_eq!(x, 3);
}
