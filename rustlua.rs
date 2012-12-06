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
	lua_pushinteger(S.state, LuaInteger(1));
	lua_pushinteger(S.state, LuaInteger(1));
	lua_rawset(S.state,-3);
	lua_pushinteger(S.state, LuaInteger(2));
	lua_pushinteger(S.state, LuaInteger(1));
	lua_rawset(S.state,-3);
	lua_pushinteger(S.state, LuaInteger(3));
	lua_pushinteger(S.state, LuaInteger(2));
	lua_rawset(S.state,-3);
	str::as_c_str("foo",|cstr| lua_setglobal(S.state,cstr));
	S.pcall(0,-1);
}
