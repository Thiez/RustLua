use libc::*;
use ptr::*;
use str::*;
use lua::*;
use lua_raw::*;

pub use /*luaL_addchar,*/ luaL_addlstring, /*luaL_addsize,*/ luaL_addstring, luaL_addvalue, luaL_argcheck,
	luaL_checkinteger, luaL_checklong, luaL_checklstring, luaL_checknumber, luaL_checkoption,
	luaL_checkstack, luaL_checkstring, luaL_checktype, luaL_checkudata, luaL_dofile, luaL_dostring,
	luaL_error, luaL_getmetafield, luaL_getmetatable, luaL_gsub, luaL_loadbuffer, luaL_loadfile,
	luaL_loadstring, luaL_newmetatable, luaL_newstate, luaL_openlibs, luaL_optint, luaL_optinteger,
	luaL_optlong, luaL_optlstring, luaL_optnumber, luaL_optstring, luaL_prepbuffer, luaL_pushresult,
	luaL_ref, luaL_register, luaL_typename, luaL_typerror, luaL_unref, luaL_where;

pub extern mod lua {
	fn luaL_addlstring(B: LuaLBuffer, s: *c_char, l: size_t) -> c_void;
	fn luaL_addstring(B: LuaLBuffer, s: *c_char) -> c_void;
	fn luaL_addvalue(B: LuaLBuffer) -> c_void;
	fn luaL_argerror(L: LuaState, narg: c_int, extramsg: *c_char) -> c_void;
	fn luaL_buffinit(L: LuaState, B: LuaLBuffer) -> c_void;
	fn luaL_callmeta(L: LuaState, obj: c_int, e: *c_char) -> c_int;
	fn luaL_checkany(L: LuaState, narg: c_int) -> c_void;
	fn luaL_checkinteger(L: LuaState, narg: c_int) -> LuaInteger;
	fn luaL_checklstring(L: LuaState, narg: c_int, l: *size_t) -> *c_char;
	fn luaL_checknumber(L: LuaState, narg: c_int) -> LuaNumber;
	fn luaL_checkoption(L: LuaState, narg: c_int, def: *c_char, lst: **c_char) -> c_int;
	fn luaL_checkstack(L: LuaState, sz: c_int, msg: *c_char) -> c_void;
	fn luaL_checktype(L: LuaState, narg: c_int, t: c_int) -> c_void;
	fn luaL_checkudata(L: LuaState, narg: c_int, tname: *c_char) -> c_void;
	fn luaL_error(L: LuaState, fmt: *c_char) -> c_int; // FIXME: should accept varargs
	fn luaL_getmetafield(L: LuaState, obj: c_int, e: *c_char) -> c_int;
	fn luaL_gsub(L: LuaState, s: *c_char, p: *c_char, r: *c_char) -> *c_char;
	fn luaL_loadbuffer(L: LuaState, buff: *c_char, sz: size_t, name: *c_char) -> c_int;
	fn luaL_loadfile(L: LuaState, filename: *c_char) -> c_int;
	fn luaL_loadstring(L: LuaState, s: *c_char) -> c_int;
	fn luaL_newmetatable(L: LuaState, tname: *c_char) -> c_int;
	fn luaL_newstate() -> LuaState;
	fn luaL_openlibs(L: LuaState) -> c_void;
	fn luaL_optinteger(L: LuaState, narg: c_int, d: LuaInteger) -> LuaInteger;
	fn luaL_optlstring(L: LuaState, narg: c_int, d: *c_char, l: *size_t) -> *c_char;
	fn luaL_optnumber(L: LuaState, narg: c_int, d: LuaNumber) -> LuaNumber;
	fn luaL_prepbuffer(B: LuaLBuffer) -> *c_char;
	fn luaL_pushresult(B: LuaLBuffer) -> c_void;
	fn luaL_ref(L: LuaState, t: c_int) -> c_int;
	fn luaL_register(L: LuaState, libname: *c_char, l: LuaLReg) -> c_void;
	fn luaL_typerror(L: LuaState, narg: c_int, tname: *c_char) -> c_int;
	fn luaL_unref(L: LuaState, t: c_int, rf: c_int) -> c_void;
	fn luaL_where(L: LuaState, lvl: c_int) -> c_void;
}

enum LuaLBuffer = *c_void;
type LuaLReg = *_LuaLReg;
struct _LuaLReg {
	name: *c_char,
	func: LuaCFunction
}

const LUA_ERRFILE: c_int = 6;



//	fn luaL_addchar(B: LuaLBuffer, c: c_char) -> c_void; FIXME: cannot support without defining LuaBuffer
//	fn luaL_addsize(B: LuaLBuffer, n: size_t) -> c_void; FIXME: cannot support without defining LuaLBuffer
fn luaL_argcheck(L: LuaState, cond: c_int, narg: c_int, extramsg: *c_char) -> c_void {
	if (cond as bool) {
		luaL_argerror(L, narg, extramsg)
	} else {
		() as c_void
	}
}

fn luaL_checkint(L: LuaState, narg: c_int) -> c_int {
	luaL_checkinteger(L,narg) as c_int
}

fn luaL_checklong(L: LuaState, narg: c_int) -> c_long {
	luaL_checkinteger(L,narg) as c_long
}

fn luaL_checkstring(L: LuaState, narg: c_int) -> *c_char {
	luaL_checklstring(L,narg,ptr::null())
}

fn luaL_dofile(L: LuaState, filename: *c_char) -> c_int {
	luaL_loadfile(L, filename);
	lua_pcall(L, 0, LUA_MULTRET, 0)
}

fn luaL_dostring(L: LuaState, string: *c_char) -> c_int {
	luaL_loadstring(L, string);
	lua_pcall(L, 0, LUA_MULTRET, 0)
}

fn luaL_getmetatable(L: LuaState, tname: *c_char) -> c_void {
	lua_getfield(L, LUA_REGISTRYINDEX, tname)
}

fn luaL_optint(L: LuaState, narg: c_int, d: c_int) -> c_int {
	luaL_optinteger(L, narg, d) as c_int
}

fn luaL_optlong(L: LuaState, narg: c_int, d: c_long) -> c_long {
	luaL_optinteger(L, narg, d) as c_long
}

fn luaL_optstring(L: LuaState, narg: c_int, d: *c_char) -> *c_char {
	luaL_optlstring(L, narg, d, ptr::null())
}

fn luaL_typename(L: LuaState, index: c_int) -> *c_char {
	lua_typename(L, lua_type(L, index))
}

