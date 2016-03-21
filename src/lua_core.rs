use hlua::Lua;

pub struct LuaBox<'a> {
    context: Lua<'a>
}

impl<'a> LuaBox<'a> {
    fn new() -> LuaBox<'a> {
        let lua = Lua::new();
        LuaBox {context: lua}
    }
}

#[test]
fn test_script_context_init() {
    let mut context = LuaBox::new();
    context.context.execute::<()>("x = 3").unwrap();
    let x: i32 = context.context.get("x").unwrap();
    assert_eq!(x, 3);
}
