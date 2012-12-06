use lua_raw::*;
use lua_wrapper::*;
use ptr::is_not_null;

fn main() {
	let L: LuaState = luaL_newstate();
	io::println("Hello world");
	io::println(fmt!("L = %b",is_not_null(L)));
	io::println("Now, let's load hello.lua");
	let loadresult = str::as_c_str("hello.lua",{|s| luaL_loadfile(L, s)}) as int;
	io::println(fmt!("Result of loading: %d", loadresult));
	luaL_openlibs(L);
	let pcallresult = lua_pcall(L, 0, -1, 0) as int;
	io::println(fmt!("Result of pcall: %d", pcallresult));
//	runLuaWithParams(L,~[]);
	
	io::println("Now with wrapper:");
	let S: LuaStack = createLuaStack();
	S.openlibs();
	S.loadFile("script.lua");
	lua_newtable(S.state);
//	lua_pushinteger(S.state, 1);
//	lua_pushinteger(S.state, 1);
	for int::range(1,5) |n| {
		S.pushi(n);
		S.pushi(n*n);
		lua_rawset(S.state,-3);
	};

	str::as_c_str("foo", {|x| lua_setglobal(S.state,x)});
	str::as_c_str("magics!",{|x| lua_pushstring(S.state,x)});
	str::as_c_str("magic", {|x| lua_setglobal(S.state,x)});
	S.pcall(0,-1);
}
