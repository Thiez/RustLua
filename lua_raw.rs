use libc::*;
use ptr::*;
use str::*;
use lua::*;

pub use lua_atpanic, lua_call, lua_checkstack, lua_close, lua_concat, lua_cpcall, lua_createtable,
	lua_dump, lua_equal, lua_error, lua_gc, lua_getallocf, lua_getfenv, lua_getfield, lua_getglobal,
	lua_getmetatable, lua_gettable, lua_gettop, lua_insert, lua_isboolean, lua_iscfunction,
	lua_isfunction, lua_islightuserdata, lua_isnil, lua_isnone, lua_isnoneornil, lua_isnumber,
	lua_isstring, lua_istable, lua_isthread, lua_isuserdata, lua_lessthan, lua_load, lua_newstate,
	lua_newtable, lua_newthread, lua_newuserdata, lua_next, lua_objlen, lua_pcall, lua_pop,
	lua_pushboolean, lua_pushcclosure, lua_pushcfunction, lua_pushfstring, lua_pushinteger,
	lua_pushlightuserdata, lua_pushliteral, lua_pushlstring, lua_pushnil, lua_pushnumber,
	lua_pushstring, lua_pushthread, lua_pushvalue, lua_pushvfstring, lua_rawequal, lua_rawget,
	lua_rawgeti, lua_rawset, lua_rawseti, lua_register, lua_remove, lua_replace, lua_resume,
	lua_setallocf, lua_setfenv, lua_setfield, lua_setglobal, lua_setmetatable, lua_settable,
	lua_settop, lua_status, lua_toboolean, lua_tocfunction, lua_tointeger, lua_tolstring,
	lua_tonumber, lua_topointer, lua_tostring, lua_tothread, lua_touserdata, lua_type,
	lua_typename, lua_xmove, lua_yield;

pub extern mod lua {
	fn lua_atpanic(L: LuaState, panicf: LuaCFunction) -> LuaCFunction;
	fn lua_call(L: LuaState, nargs: c_int, nresults: c_int) -> c_void;
	fn lua_checkstack(L: LuaState, extra: c_int) -> c_int;
	fn lua_close(L: LuaState) -> c_void;
	fn lua_concat(L: LuaState, n: c_int) -> c_void;
	fn lua_cpcall(L: LuaState, func: LuaCFunction, ud: *c_void) -> c_int;
	fn lua_createtable(L: LuaState, narr: c_int, nrec: c_int) -> c_void;
	fn lua_dump(L: LuaState, writer: LuaWriter, data: *c_void) -> c_int;
	fn lua_equal(L: LuaState, index1: c_int, index2: c_int) -> c_int;
	fn lua_error(L: LuaState) -> c_int;
	fn lua_gc(L: LuaState, what: c_int, data: c_int) -> c_int;
	fn lua_getallocf(L: LuaState, ud: **c_void) -> LuaAlloc;
	fn lua_getfenv(L: LuaState, index: c_int) -> c_void;
	fn lua_getfield(L: LuaState, index: c_int, k: *c_char) -> c_void;

	fn lua_getmetatable(L: LuaState, index: c_int) -> c_int;
	fn lua_gettable(L: LuaState, index: c_int) -> c_void;
	fn lua_gettop(L: LuaState) -> c_int;
	fn lua_insert(L: LuaState, index: c_int) -> c_void;

	fn lua_iscfunction(L: LuaState, index: c_int) -> c_int;

	fn lua_isnumber(L: LuaState, index: c_int) -> c_int;
	fn lua_isstring(L: LuaState, index: c_int) -> c_int;

	fn lua_isuserdata(L: LuaState, index: c_int) -> c_int;
	fn lua_lessthan(L: LuaState, index1: c_int, index2: c_int) -> c_int;
	fn lua_load(L: LuaState, reader: LuaReader, data: *c_void, chunkname: *c_char);
	fn lua_newstate(f: LuaAlloc, ud: *c_void) -> LuaState;

	fn lua_newthread(L: LuaState) -> LuaState;
	fn lua_newuserdata(L: LuaState, size: size_t) -> *c_void;
	fn lua_next(L: LuaState, index: c_int) -> c_int;
	fn lua_objlen(L: LuaState, inedx: c_int) -> size_t;
	fn lua_pcall(L: LuaState, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int;
	fn lua_pushboolean(L: LuaState, b: c_int);
	fn lua_pushcclosure(L: LuaState, f: LuaCFunction, n: c_int) -> c_void;
	
	fn lua_pushfstring(L: LuaState, fmt: *c_char) -> *c_char; // FIXME: should be varargs.
	fn lua_pushinteger(L: LuaState, n: LuaInteger) -> c_void;
	fn lua_pushlightuserdata(L: LuaState, p: *c_void) -> c_void;

	fn lua_pushlstring(L: LuaState, s: *c_char, len: size_t) -> c_void;
	fn lua_pushnil(L: LuaState) -> c_void;
	fn lua_pushnumber(L: LuaState, n: LuaNumber) -> c_void;
	fn lua_pushstring(L: LuaState, s: *c_char) -> c_void;
	fn lua_pushthread(L: LuaState) -> c_void;
	fn lua_pushvalue(L: LuaState, index: c_int) -> c_void;
	fn lua_pushvfstring(L: LuaState, fmt: *c_char);	// FIXME: should be varargs using va_list
	fn lua_rawequal(L: LuaState, index1: c_int, index2: c_int) -> c_int;
	fn lua_rawget(L: LuaState, index: c_int) -> c_void;
	fn lua_rawgeti(L: LuaState, index: c_int, n: c_int) -> c_void;
	fn lua_rawset(L: LuaState, index: c_int) -> c_void;
	fn lua_rawseti(L: LuaState, index: c_int, n: c_int) -> c_void;

	fn lua_remove(L: LuaState, index: c_int) -> c_void;
	fn lua_replace(L: LuaState, index: c_int) -> c_void;
	fn lua_resume(L: LuaState, narg: c_int) -> c_int;
	fn lua_setallocf(L: LuaState, f: LuaAlloc, ud: *c_void) -> c_void;
	fn lua_setfenv(L: LuaState, index: c_int) -> c_int;
	fn lua_setfield(L: LuaState, index: c_int, k: *c_char) -> c_void;

	fn lua_setmetatable(L: LuaState, index: c_int) -> c_int;
	fn lua_settable(L: LuaState, index: c_int) -> c_void;
	fn lua_settop(L: LuaState, index: c_int) -> c_void;
	fn lua_status(L: LuaState) -> c_int;
	fn lua_toboolean(L: LuaState, index: c_int) -> c_int;
	fn lua_tocfunction(L: LuaState, index: c_int) -> LuaCFunction;
	fn lua_tointeger(L: LuaState, index: c_int) -> LuaInteger;
	fn lua_tolstring(L: LuaState, index: c_int, len: *size_t) -> *c_char;
	fn lua_tonumber(L: LuaState, index: c_int) -> LuaNumber;
	fn lua_topointer(L: LuaState, index: c_int) -> *c_void;

	fn lua_tothread(L: LuaState, index: c_int) -> LuaState;
	fn lua_touserdata(L: LuaState, index: c_int) -> *c_void;
	fn lua_type(L: LuaState, index: c_int) -> c_int;
	fn lua_typename(L: LuaState, tp: c_int) -> *c_char;
	fn lua_xmove(from: LuaState, to: LuaState, n: c_int) -> c_void;
	fn lua_yield(L: LuaState, nresults: c_int) -> c_int;
}



const LUA_MULTRET: c_int = (-1);

const LUA_REGISTRYINDEX: c_int = (-10000);
const LUA_ENVIRONINDEX: c_int = (-10001);
const LUA_GLOBALSINDEX: c_int = (-10002);

const LUA_YIELD: c_int = 1;
const LUA_ERRRUN: c_int = 2;
const LUA_ERRSYNTAX: c_int = 3;
const LUA_ERRMEM: c_int = 4;
const LUA_ERRERR: c_int = 5;

const LUA_TNONE: c_int = (-1);
const LUA_TNIL: c_int = 0;
const LUA_TBOOLEAN: c_int = 1;
const LUA_TLIGHTUSERDATA: c_int = 2;
const LUA_TNUMBER: c_int = 3;
const LUA_TSTRING: c_int = 4;
const LUA_TTABLE: c_int = 5;
const LUA_TFUNCTION: c_int = 6;
const LUA_TUSERDATA: c_int = 7;
const LUA_TTHREAD: c_int = 8;

const LUA_MINSTACK: c_int = 20;

const LUA_GCSTOP: c_int = 0;
const LUA_GCRESTART: c_int = 1;
const LUA_GCCOLLECT: c_int = 2;
const LUA_GCCOUNT: c_int = 3;
const LUA_GCCOUNTB: c_int = 4;
const LUA_GCSTEP: c_int = 5;
const LUA_GCSETPAUSE: c_int = 6;
const LUA_GCSETSTEPMUL: c_int = 7;



type LuaAlloc = fn (ud: *c_void, ptr: *c_void, osize: size_t, nsize: size_t) -> *c_void;
type LuaCFunction = fn (L: LuaState) -> c_int;
enum LuaInteger = int;	// Should be same size as ptrdiff_t
enum LuaNumber = c_double;
type LuaReader = fn (L: LuaState, data: *c_void, size: size_t) -> *c_char;
type LuaState = *c_void;
type LuaWriter = fn (L: LuaState, p: *c_void, sz: size_t, ud: *c_void) -> c_int;


fn lua_pop(L: LuaState, n: c_int) -> c_void {
	lua_settop(L, -(n)-1)
}

fn lua_newtable(L: LuaState) -> c_void {
	lua_createtable(L, 0, 0)
}

fn lua_register(L: LuaState, name: *c_char, f: LuaCFunction) -> c_void {
	lua_pushcfunction(L, f);
	lua_setglobal(L, name)
}

fn lua_pushcfunction(L: LuaState, f: LuaCFunction) -> c_void {
	lua_pushcclosure(L, f, 0)
}

fn lua_strlen(L: LuaState, index: c_int) -> size_t {
	lua_objlen(L,index)
}

fn lua_isboolean(L: LuaState, index: c_int) -> c_int {
	(lua_type(L,index) == LUA_TBOOLEAN) as c_int
}

fn lua_isfunction(L: LuaState, index: c_int) -> c_int {
	(lua_type(L,index) == LUA_TFUNCTION) as c_int
}

fn lua_islightuserdata(L: LuaState, index: c_int) -> c_int {
	(lua_type(L,index) == LUA_TLIGHTUSERDATA) as c_int
}

fn lua_isnil(L: LuaState, index: c_int) -> c_int {
	(lua_type(L,index) == LUA_TNIL) as c_int
}

fn lua_isnone(L: LuaState, index: c_int) -> c_int {
	(lua_type(L,index) == LUA_TNONE) as c_int
}

fn lua_isnoneornil(L: LuaState, index: c_int) -> c_int {
	let t = lua_type(L,index);
	(t == LUA_TNIL || t == LUA_TNONE) as c_int
}

fn lua_istable(L: LuaState, index: c_int) -> c_int {
	(lua_type(L,index) == LUA_TTABLE) as c_int
}

fn lua_isthread(L: LuaState, index: c_int) -> c_int {
	(lua_type(L,index) == LUA_TTHREAD) as c_int
}

fn lua_pushliteral(L: LuaState, s: *c_char) -> c_void {
	lua_pushlstring(L, s, strlen(s)-1)
}

fn lua_setglobal(L: LuaState, name: *c_char) -> c_void {
	lua_setfield(L, LUA_GLOBALSINDEX, name)
}

fn lua_getglobal(L: LuaState, name: *c_char) -> c_void {
	lua_getfield(L, LUA_GLOBALSINDEX, name)
}

fn lua_tostring(L: LuaState, index: c_int) -> *c_char {
	lua_tolstring(L, index, ptr::null())
}

