use lua_raw::*;
use std::map::*;

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
	fn pushi<T: LuaConvert>(item: T) {
		self.push(item.toLuaItem())
	}
	fn push(item: LuaStackItem) {
		match item {
		LuaNone => fail ~"Cannot push None!",
		LuaNil => lua_pushnil(self.state),
		LuaBool(b) => lua_pushboolean(self.state,b as c_int),
		LuaLightUserData(v) => lua_pushlightuserdata(self.state,v),
		LuaNumber(n) => lua_pushnumber(self.state,n),
		LuaString(s) => str::as_c_str(s,|x| lua_pushstring(self.state,x)),
		LuaTable(_) => fail ~"Pushing table not yet implemented",
		LuaFunction(_) => fail ~"Pushing function not yet implemented",
		LuaUserData(_) => fail ~"Pushing userdata not yet implemented",
		LuaThread(_) => fail ~"Pushing thread not yet implemented"
		};
	}
	fn pop() -> LuaStackItem {
		let n = lua_gettop(self.state);
		if (n < 1) {
			LuaNone
		} else {
			let result =
			match lua_type(self.state,n) {
				LUA_TNONE => LuaNone,
				LUA_TNIL => LuaNil,
				LUA_TBOOLEAN => LuaBool( lua_toboolean(self.state,n) == 1 ),
				LUA_TLIGHTUSERDATA => LuaLightUserData( lua_touserdata(self.state,n) ),
				LUA_TNUMBER => fail,
				LUA_TSTRING => fail,
				LUA_TTABLE => fail,
				LUA_TFUNCTION => fail,
				LUA_TUSERDATA => LuaUserData( lua_touserdata(self.state,n) ),
				LUA_TTHREAD => LuaThread( LuaStack{ state: lua_tothread(self.state,n) } ),
				_ => LuaNone
			};
			lua_pop(self.state,1);
			result
		}
	}
	fn call(nargs: int, results: int) {
		lua_call(self.state, nargs as c_int, results as c_int)
	}
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

enum LuaStackItem {
	LuaNone,
	LuaNil,
	LuaBool(bool),
	LuaLightUserData(*c_void),
	LuaNumber(LuaNumber),
	LuaString(@str),
	LuaTable(c_void),
	LuaFunction(c_void),
	LuaUserData(*c_void),
	LuaThread(LuaStack)
}

trait LuaConvert {
	fn toLuaItem() -> LuaStackItem;
}

impl uint : LuaConvert {
	fn toLuaItem() -> LuaStackItem { LuaNumber(self as LuaNumber) }
}

impl int : LuaConvert {
	fn toLuaItem() -> LuaStackItem { LuaNumber(self as LuaNumber) }
}

impl float : LuaConvert {
	fn toLuaItem() -> LuaStackItem { LuaNumber(self as LuaNumber) }
}

impl bool : LuaConvert {
	fn toLuaItem() -> LuaStackItem { LuaBool(self) }
}

impl @str : LuaConvert {
	fn toLuaItem() -> LuaStackItem { LuaString(copy self) }
}


