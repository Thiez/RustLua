use libc::{c_char,c_double,c_int,c_long,c_void,size_t,strlen};

use std::ptr::RawPtr;

#[link(name = "lua5.2")]
extern {
    /************************************************************/
    /*                                                            */
    /*                    The stuff from lua.h                    */
    /*                                                            */
    /************************************************************/
    pub fn lua_atpanic(L: LuaState, panicf: LuaCFunction) -> LuaCFunction;
    pub fn lua_callk(L: LuaState, nargs: c_int, nresults: c_int, ctx: c_int, k: LuaCFunction);
    pub fn lua_checkstack(L: LuaState, extra: c_int) -> c_int;
    pub fn lua_close(L: LuaState);
    pub fn lua_compare(L: LuaState, index1: c_int, index2: c_int, op: c_int) -> c_int;
    pub fn lua_concat(L: LuaState, n: c_int);
    pub fn lua_createtable(L: LuaState, narr: c_int, nrec: c_int) ;
    pub fn lua_dump(L: LuaState, writer: LuaWriter, data: *c_void) -> c_int;
    pub fn lua_equal(L: LuaState, index1: c_int, index2: c_int) -> c_int;
    pub fn lua_error(L: LuaState) -> c_int;
    pub fn lua_gc(L: LuaState, what: c_int, data: c_int) -> c_int;
    pub fn lua_getallocf(L: LuaState, ud: **c_void) -> LuaAlloc;
    pub fn lua_getfenv(L: LuaState, index: c_int) ;
    pub fn lua_getfield(L: LuaState, index: c_int, k: *c_char) ;
    pub fn lua_getmetatable(L: LuaState, index: c_int) -> c_int;
    pub fn lua_gettable(L: LuaState, index: c_int) ;
    pub fn lua_gettop(L: LuaState) -> c_int;
    pub fn lua_insert(L: LuaState, index: c_int) ;
    pub fn lua_iscfunction(L: LuaState, index: c_int) -> c_int;
    pub fn lua_isnumber(L: LuaState, index: c_int) -> c_int;
    pub fn lua_isstring(L: LuaState, index: c_int) -> c_int;
    pub fn lua_isuserdata(L: LuaState, index: c_int) -> c_int;
    pub fn lua_lessthan(L: LuaState, index1: c_int, index2: c_int) -> c_int;
    pub fn lua_load(L: LuaState, reader: LuaReader, data: *c_void, source: *c_char, mode: *c_char)-> c_int;
    pub fn lua_newstate(f: LuaAlloc, ud: *c_void) -> LuaState;
    pub fn lua_newthread(L: LuaState) -> LuaState;
    pub fn lua_newuserdata(L: LuaState, size: size_t) -> *c_void;
    pub fn lua_next(L: LuaState, index: c_int) -> c_int;
    pub fn lua_rawlen(L: LuaState, inedx: c_int) -> size_t;
    pub fn lua_pcallk(L: LuaState, nargs: c_int, nresults: c_int, errfunc: c_int, ctx: c_int, k: LuaCFunction) -> c_int;
    pub fn lua_pushboolean(L: LuaState, b: c_int);
    pub fn lua_pushcclosure(L: LuaState, f: LuaCFunction, n: c_int) ;
    pub fn lua_pushfstring(L: LuaState, fmt: *c_char) -> *c_char; // FIXME: should be varargs.
    pub fn lua_pushinteger(L: LuaState, n: LuaInteger) ;
    pub fn lua_pushlightuserdata(L: LuaState, p: *c_void) ;
    pub fn lua_pushlstring(L: LuaState, s: *c_char, len: size_t) ;
    pub fn lua_pushnil(L: LuaState) ;
    pub fn lua_pushnumber(L: LuaState, n: LuaNum) ;
    pub fn lua_pushstring(L: LuaState, s: *c_char) ;
    pub fn lua_pushthread(L: LuaState) ;
    pub fn lua_pushvalue(L: LuaState, index: c_int) ;
    pub fn lua_pushvfstring(L: LuaState, fmt: *c_char);    // FIXME: should be varargs using va_list
    pub fn lua_rawequal(L: LuaState, index1: c_int, index2: c_int) -> c_int;
    pub fn lua_rawget(L: LuaState, index: c_int) ;
    pub fn lua_rawgeti(L: LuaState, index: c_int, n: c_int) ;
    pub fn lua_rawset(L: LuaState, index: c_int) ;
    pub fn lua_rawseti(L: LuaState, index: c_int, n: c_int) ;
    pub fn lua_remove(L: LuaState, index: c_int) ;
    pub fn lua_replace(L: LuaState, index: c_int) ;
    pub fn lua_resume(L: LuaState, from: LuaState, nargs: c_int) -> c_int;
    pub fn lua_setallocf(L: LuaState, f: LuaAlloc, ud: *c_void) ;
    pub fn lua_setfenv(L: LuaState, index: c_int) -> c_int;
    pub fn lua_setfield(L: LuaState, index: c_int, k: *c_char) ;
    pub fn lua_setmetatable(L: LuaState, index: c_int) -> c_int;
    pub fn lua_settable(L: LuaState, index: c_int) ;
    pub fn lua_settop(L: LuaState, index: c_int) ;
    pub fn lua_status(L: LuaState) -> c_int;
    pub fn lua_toboolean(L: LuaState, index: c_int) -> c_int;
    pub fn lua_tocfunction(L: LuaState, index: c_int) -> LuaCFunction;
    pub fn lua_tointeger(L: LuaState, index: c_int) -> LuaInteger;
    pub fn lua_tolstring(L: LuaState, index: c_int, len: *size_t) -> *c_char;
    pub fn lua_tonumber(L: LuaState, index: c_int) -> LuaNum;
    pub fn lua_topointer(L: LuaState, index: c_int) -> *c_void;
    pub fn lua_tothread(L: LuaState, index: c_int) -> LuaState;
    pub fn lua_touserdata(L: LuaState, index: c_int) -> *c_void;
    pub fn lua_type(L: LuaState, index: c_int) -> c_int;
    pub fn lua_typename(L: LuaState, tp: c_int) -> *c_char;
    pub fn lua_xmove(from: LuaState, to: LuaState, n: c_int) ;
    pub fn lua_yield(L: LuaState, nresults: c_int) -> c_int;
    
    /************************************************************/
    /*                                                            */
    /*                    The stuff from lauxlib.h                */
    /*                                                            */
    /************************************************************/
    pub fn luaL_addlstring(B: LuaLBuffer, s: *c_char, l: size_t) ;
    pub fn luaL_addstring(B: LuaLBuffer, s: *c_char) ;
    pub fn luaL_addvalue(B: LuaLBuffer) ;
    pub fn luaL_argerror(L: LuaState, narg: c_int, extramsg: *c_char) ;
    pub fn luaL_buffinit(L: LuaState, B: LuaLBuffer) ;
    pub fn luaL_callmeta(L: LuaState, obj: c_int, e: *c_char) -> c_int;
    pub fn luaL_checkany(L: LuaState, narg: c_int) ;
    pub fn luaL_checkinteger(L: LuaState, narg: c_int) -> LuaInteger;
    pub fn luaL_checklstring(L: LuaState, narg: c_int, l: *size_t) -> *c_char;
    pub fn luaL_checknumber(L: LuaState, narg: c_int) -> LuaNum;
    pub fn luaL_checkoption(L: LuaState, narg: c_int, def: *c_char, lst: **c_char) -> c_int;
    pub fn luaL_checkstack(L: LuaState, sz: c_int, msg: *c_char) ;
    pub fn luaL_checktype(L: LuaState, narg: c_int, t: c_int) ;
    pub fn luaL_checkudata(L: LuaState, narg: c_int, tname: *c_char) ;
    pub fn luaL_error(L: LuaState, fmt: *c_char) -> c_int; // FIXME: should accept varargs
    pub fn luaL_getmetafield(L: LuaState, obj: c_int, e: *c_char) -> c_int;
    pub fn luaL_gsub(L: LuaState, s: *c_char, p: *c_char, r: *c_char) -> *c_char;
    pub fn luaL_loadbuffer(L: LuaState, buff: *c_char, sz: size_t, name: *c_char) -> c_int;
    pub fn luaL_loadfilex(L: LuaState, filename: *c_char, mode: *c_char) -> c_int;
    pub fn luaL_loadstring(L: LuaState, s: *c_char) -> c_int;
    pub fn luaL_newmetatable(L: LuaState, tname: *c_char) -> c_int;
    pub fn luaL_newstate() -> LuaState;
    pub fn luaL_openlibs(L: LuaState) ;
    pub fn luaL_optinteger(L: LuaState, narg: c_int, d: LuaInteger) -> LuaInteger;
    pub fn luaL_optlstring(L: LuaState, narg: c_int, d: *c_char, l: *size_t) -> *c_char;
    pub fn luaL_optnumber(L: LuaState, narg: c_int, d: LuaNum) -> LuaNum;
    pub fn luaL_prepbuffer(B: LuaLBuffer) -> *c_char;
    pub fn luaL_pushresult(B: LuaLBuffer) ;
    pub fn luaL_ref(L: LuaState, t: c_int) -> c_int;
    pub fn luaL_register(L: LuaState, libname: *c_char, l: LuaLReg) ;
    pub fn luaL_typerror(L: LuaState, narg: c_int, tname: *c_char) -> c_int;
    pub fn luaL_unref(L: LuaState, t: c_int, rf: c_int) ;
    pub fn luaL_where(L: LuaState, lvl: c_int) ;
}


/****************************************************************/
/*                                                                */
/*                    Constants from lua.h                        */
/*                                                                */
/****************************************************************/

pub static LUA_MULTRET: c_int = (-1);

pub static LUA_REGISTRYINDEX: c_int = (-10000);
pub static LUA_ENVIRONINDEX: c_int = (-10001);
pub static LUA_GLOBALSINDEX: c_int = (-10002);

pub static LUA_YIELD: c_int = 1;
pub static LUA_ERRRUN: c_int = 2;
pub static LUA_ERRSYNTAX: c_int = 3;
pub static LUA_ERRMEM: c_int = 4;
pub static LUA_ERRERR: c_int = 5;

pub static LUA_TNONE: c_int = (-1);
pub static LUA_TNIL: c_int = 0;
pub static LUA_TBOOLEAN: c_int = 1;
pub static LUA_TLIGHTUSERDATA: c_int = 2;
pub static LUA_TNUMBER: c_int = 3;
pub static LUA_TSTRING: c_int = 4;
pub static LUA_TTABLE: c_int = 5;
pub static LUA_TFUNCTION: c_int = 6;
pub static LUA_TUSERDATA: c_int = 7;
pub static LUA_TTHREAD: c_int = 8;

pub static LUA_MINSTACK: c_int = 20;

pub static LUA_GCSTOP: c_int = 0;
pub static LUA_GCRESTART: c_int = 1;
pub static LUA_GCCOLLECT: c_int = 2;
pub static LUA_GCCOUNT: c_int = 3;
pub static LUA_GCCOUNTB: c_int = 4;
pub static LUA_GCSTEP: c_int = 5;
pub static LUA_GCSETPAUSE: c_int = 6;
pub static LUA_GCSETSTEPMUL: c_int = 7;


/****************************************************************/
/*                                                                */
/*                        Types from lua.h                        */
/*                                                                */
/****************************************************************/

pub type LuaAlloc = fn (ud: *c_void, ptr: *c_void, osize: size_t, nsize: size_t) -> *c_void;
pub type LuaCFunction = fn (L: LuaState) -> c_int;
pub type LuaInteger = int;    // Int in rust should be same size as ptrdiff_t
pub type LuaNum = c_double;
pub type LuaReader = fn (L: LuaState, data: *c_void, size: size_t) -> *c_char;
pub type LuaState = *c_void;
pub type LuaWriter = fn (L: LuaState, p: *c_void, sz: size_t, ud: *c_void) -> c_int;


/****************************************************************/
/*                                                                */
/*        Some functions that are macros in lua.h                    */
/*                                                                */
/****************************************************************/

pub unsafe fn lua_call(L: LuaState, nargs: c_int, nresults: c_int) {
  use std::cast::transmute;
  lua_callk(L, nargs, nresults, 0, transmute(0))
}

pub unsafe fn lua_pcall(L: LuaState, nargs: c_int, nresults: c_int, msgh: c_int) -> c_int {
  use std::cast::transmute;
  lua_pcallk(L, nargs, nresults, msgh, 0, transmute(0))
}

pub unsafe fn lua_pop(L: LuaState, n: c_int)  {
  lua_settop(L, -(n)-1)
}

pub unsafe fn lua_newtable(L: LuaState)  {
  lua_createtable(L, 0, 0)
}

pub unsafe fn lua_register(L: LuaState, name: *c_char, f: LuaCFunction)  {
  lua_pushcfunction(L, f);
  lua_setglobal(L, name)
}

pub unsafe fn lua_pushcfunction(L: LuaState, f: LuaCFunction)  {
  lua_pushcclosure(L, f, 0)
}

pub unsafe fn lua_strlen(L: LuaState, index: c_int) -> size_t {
  lua_rawlen(L,index)
}

pub unsafe fn lua_isboolean(L: LuaState, index: c_int) -> c_int {
  (lua_type(L,index) == LUA_TBOOLEAN) as c_int
}

pub unsafe fn lua_isfunction(L: LuaState, index: c_int) -> c_int {
  (lua_type(L,index) == LUA_TFUNCTION) as c_int
}

pub unsafe fn lua_islightuserdata(L: LuaState, index: c_int) -> c_int {
  (lua_type(L,index) == LUA_TLIGHTUSERDATA) as c_int
}

pub unsafe fn lua_isnil(L: LuaState, index: c_int) -> c_int {
  (lua_type(L,index) == LUA_TNIL) as c_int
}

pub unsafe fn lua_isnone(L: LuaState, index: c_int) -> c_int {
  (lua_type(L,index) == LUA_TNONE) as c_int
}

pub unsafe fn lua_isnoneornil(L: LuaState, index: c_int) -> c_int {
  let t = lua_type(L,index);
  (t == LUA_TNIL || t == LUA_TNONE) as c_int
}

pub unsafe fn lua_istable(L: LuaState, index: c_int) -> c_int {
  (lua_type(L,index) == LUA_TTABLE) as c_int
}

pub unsafe fn lua_isthread(L: LuaState, index: c_int) -> c_int {
  (lua_type(L,index) == LUA_TTHREAD) as c_int
}

pub unsafe fn lua_pushliteral(L: LuaState, s: *c_char)  {
  lua_pushlstring(L, s, strlen(s)-1)
}

pub unsafe fn lua_setglobal(L: LuaState, name: *c_char)  {
  lua_setfield(L, LUA_GLOBALSINDEX, name)
}

pub unsafe fn lua_getglobal(L: LuaState, name: *c_char)  {
  lua_getfield(L, LUA_GLOBALSINDEX, name)
}

pub unsafe fn lua_tostring(L: LuaState, index: c_int) -> *c_char {
  lua_tolstring(L, index, RawPtr::null())
}

/****************************************************************/
/*                                                              */
/*                  Constants from lauxlib.h                    */
/*                                                              */
/****************************************************************/
pub static LUA_ERRFILE: c_int = 6;


/****************************************************************/
/*                                                              */
/*                    Types from lauxlib.h                        */
/*                                                                */
/****************************************************************/

pub struct LuaLBuffer(*c_void);
pub type LuaLReg = *_LuaLReg;
pub struct _LuaLReg {
    name: *c_char,
    func: LuaCFunction
}




/****************************************************************/
/*                                                                */
/*        Some functions that are macros in lauxlib.h                */
/*                                                                */
/****************************************************************/

//    pub fn luaL_addchar(B: LuaLBuffer, c: c_char) ; FIXME: cannot support without defining LuaBuffer
//    pub fn luaL_addsize(B: LuaLBuffer, n: size_t) ; FIXME: cannot support without defining LuaLBuffer
pub unsafe fn luaL_argcheck(L: LuaState, cond: c_int, narg: c_int, extramsg: *c_char)  {
    if (cond != 0) {
        luaL_argerror(L, narg, extramsg);
    }
}

pub unsafe fn luaL_checkint(L: LuaState, narg: c_int) -> c_int {
    luaL_checkinteger(L,narg) as c_int
}

pub unsafe fn luaL_checklong(L: LuaState, narg: c_int) -> c_long {
    luaL_checkinteger(L,narg) as c_long
}

pub unsafe fn luaL_checkstring(L: LuaState, narg: c_int) -> *c_char {
    luaL_checklstring(L,narg,RawPtr::null())
}

pub unsafe fn luaL_dofile(L: LuaState, filename: *c_char) -> c_int {
    luaL_loadfile(L, filename);
    lua_pcall(L, 0, LUA_MULTRET, 0)
}

pub unsafe fn luaL_dostring(L: LuaState, string: *c_char) -> c_int {
    luaL_loadstring(L, string);
    lua_pcall(L, 0, LUA_MULTRET, 0)
}

pub unsafe fn luaL_getmetatable(L: LuaState, tname: *c_char)  {
    lua_getfield(L, LUA_REGISTRYINDEX, tname)
}

pub unsafe fn luaL_loadfile(L: LuaState, filename: *c_char) -> c_int {
  luaL_loadfilex(L,filename,RawPtr::null())
}

pub unsafe fn luaL_optint(L: LuaState, narg: c_int, d: c_int) -> c_int {
    luaL_optinteger(L, narg, d as LuaInteger) as c_int
}

pub unsafe fn luaL_optlong(L: LuaState, narg: c_int, d: c_long) -> c_long {
    luaL_optinteger(L, narg, d as LuaInteger) as c_long
}

pub unsafe fn luaL_optstring(L: LuaState, narg: c_int, d: *c_char) -> *c_char {
    luaL_optlstring(L, narg, d, RawPtr::null())
}

pub unsafe fn luaL_typename(L: LuaState, index: c_int) -> *c_char {
    lua_typename(L, lua_type(L, index))
}

/*fn main() {
    let L: LuaState = lua::luaL_newstate();
    io::println("Hello world");
    io::println(fmt!("L = %b",is_not_null(L)));
    io::println("Now, let's load hello.lua");
    let loadresult = str::as_c_str("hello.lua",{|s| lua::luaL_loadfile(L, s)}) as int;
    io::println(fmt!("Result of loading: %d", loadresult));
    lua::luaL_openlibs(L);
    let pcallresult = lua::lua_pcall(L, 0, -1, 0) as int;
    io::println(fmt!("Result of pcall: %d", pcallresult));
//    runLuaWithParams(L,~[]);
    
}
*/
