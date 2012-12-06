use lua_raw::*;

pub use LuaStack, createLuaStack;

/* const LUA_TNONE: c_int = (-1);
const LUA_TNIL: c_int = 0;
const LUA_TBOOLEAN: c_int = 1;
const LUA_TLIGHTUSERDATA: c_int = 2;
const LUA_TNUMBER: c_int = 3;
const LUA_TSTRING: c_int = 4;
const LUA_TTABLE: c_int = 5;
const LUA_TFUNCTION: c_int = 6;
const LUA_TUSERDATA: c_int = 7;
const LUA_TTHREAD: c_int = 8;
*/

struct LuaStack {
	state: LuaState
}

impl LuaStack {
	fn hi() { io::println("Hello world"); }
	fn push<T: LuaStackItem>(item: &T) { item.push(self.state) }
	fn pcall(nargs: int, results: int) -> int {
			lua_pcall(self.state, nargs as c_int, results as c_int, 0) as int
	}
	fn openlibs() { luaL_openlibs(self.state) }
	fn loadFile(file: &str) {
		let loadresult = str::as_c_str(file,{|fname| luaL_loadfile(self.state, fname)}) as int;
		if (loadresult != 0) { fail ~"Some failure reading file" }
	}
}


fn createLuaStack() -> LuaStack {
	LuaStack { state: luaL_newstate() }
}



enum LuaType {
	None = -1,
	Nil = 0,
	Bool = 1,
	LightUserData = 2,
	Number = 3,
	String = 4,
	Table = 5,
	Function = 6,
	UserData = 7,
	Thread = 8
}

trait LuaStackItem {
	fn luaType() -> LuaType;
	fn push(state: LuaState);
}

impl bool : LuaStackItem {
	fn luaType() -> LuaType { Bool }
	fn push(state: LuaState) { lua_pushboolean(state,self as c_int) }
}

impl int : LuaStackItem {
	fn luaType() -> LuaType { Number }
	fn push(state: LuaState) { lua_pushinteger(state,LuaInteger(self)) }
}

impl float : LuaStackItem {
	fn luaType() -> LuaType { Number }
	fn push(state: LuaState) { (self as c_double).push(state) }
}

impl c_double : LuaStackItem {
	fn luaType() -> LuaType { Number }
	fn push(state: LuaState) { LuaNumber(self).push(state) }
}

impl LuaNumber : LuaStackItem {
	fn luaType() -> LuaType { Number }
	fn push(state: LuaState) { lua_pushnumber(state,self) }
}

impl () : LuaStackItem {
	fn luaType() -> LuaType { Nil }
	fn push(state: LuaState) { lua_pushnil(state) }
}

impl &str : LuaStackItem {
	fn luaType() -> LuaType { String }
	fn push(state: LuaState) { str::as_c_str(self,{|s| lua_pushstring(state, s)}) }
}
