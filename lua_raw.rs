use libc::{c_char,c_double,c_int,c_long,c_void,size_t,strlen};

use std::ptr::RawPtr;

#[link(name = "lua5.2")]
#[allow(dead_code)]
extern {
    /************************************************************/
    /*                                                            */
    /*                    The stuff from lua.h                    */
    /*                                                            */
    /************************************************************/
    pub fn lua_atpanic(lS: LuaState, panicf: LuaCFunction) -> LuaCFunction;
    pub fn lua_callk(lS: LuaState, nargs: c_int, nresults: c_int, ctx: c_int, k: LuaCFunction);
    pub fn lua_checkstack(lS: LuaState, extra: c_int) -> c_int;
    pub fn lua_close(lS: LuaState);
    pub fn lua_compare(lS: LuaState, index1: c_int, index2: c_int, op: c_int) -> c_int;
    pub fn lua_concat(lS: LuaState, n: c_int);
    pub fn lua_createtable(lS: LuaState, narr: c_int, nrec: c_int) ;
    pub fn lua_dump(lS: LuaState, writer: LuaWriter, data: *c_void) -> c_int;
    pub fn lua_equal(lS: LuaState, index1: c_int, index2: c_int) -> c_int;
    pub fn lua_error(lS: LuaState) -> c_int;
    pub fn lua_gc(lS: LuaState, what: c_int, data: c_int) -> c_int;
    pub fn lua_getallocf(lS: LuaState, ud: **c_void) -> LuaAlloc;
    pub fn lua_getfenv(lS: LuaState, index: c_int) ;
    pub fn lua_getfield(lS: LuaState, index: c_int, k: *c_char) ;
    pub fn lua_getmetatable(lS: LuaState, index: c_int) -> c_int;
    pub fn lua_gettable(lS: LuaState, index: c_int) ;
    pub fn lua_gettop(lS: LuaState) -> c_int;
    pub fn lua_insert(lS: LuaState, index: c_int) ;
    pub fn lua_iscfunction(lS: LuaState, index: c_int) -> c_int;
    pub fn lua_isnumber(lS: LuaState, index: c_int) -> c_int;
    pub fn lua_isstring(lS: LuaState, index: c_int) -> c_int;
    pub fn lua_isuserdata(lS: LuaState, index: c_int) -> c_int;
    pub fn lua_lessthan(lS: LuaState, index1: c_int, index2: c_int) -> c_int;
    pub fn lua_load(lS: LuaState, reader: LuaReader, data: *c_void, source: *c_char, mode: *c_char)-> c_int;
    pub fn lua_newstate(f: LuaAlloc, ud: *c_void) -> LuaState;
    pub fn lua_newthread(lS: LuaState) -> LuaState;
    pub fn lua_newuserdata(lS: LuaState, size: size_t) -> *c_void;
    pub fn lua_next(lS: LuaState, index: c_int) -> c_int;
    pub fn lua_rawlen(lS: LuaState, inedx: c_int) -> size_t;
    pub fn lua_pcallk(lS: LuaState, nargs: c_int, nresults: c_int, errfunc: c_int, ctx: c_int, k: LuaCFunction) -> c_int;
    pub fn lua_pushboolean(lS: LuaState, b: c_int);
    pub fn lua_pushcclosure(lS: LuaState, f: LuaCFunction, n: c_int) ;
    pub fn lua_pushfstring(lS: LuaState, fmt: *c_char) -> *c_char; // FIXME: should be varargs.
    pub fn lua_pushinteger(lS: LuaState, n: LuaInteger) ;
    pub fn lua_pushlightuserdata(lS: LuaState, p: *c_void) ;
    pub fn lua_pushlstring(lS: LuaState, s: *c_char, len: size_t) ;
    pub fn lua_pushnil(lS: LuaState) ;
    pub fn lua_pushnumber(lS: LuaState, n: LuaNum) ;
    pub fn lua_pushstring(lS: LuaState, s: *c_char) ;
    pub fn lua_pushthread(lS: LuaState) ;
    pub fn lua_pushvalue(lS: LuaState, index: c_int) ;
    pub fn lua_pushvfstring(lS: LuaState, fmt: *c_char);    // FIXME: should be varargs using va_list
    pub fn lua_rawequal(lS: LuaState, index1: c_int, index2: c_int) -> c_int;
    pub fn lua_rawget(lS: LuaState, index: c_int) ;
    pub fn lua_rawgeti(lS: LuaState, index: c_int, n: c_int) ;
    pub fn lua_rawset(lS: LuaState, index: c_int) ;
    pub fn lua_rawseti(lS: LuaState, index: c_int, n: c_int) ;
    pub fn lua_remove(lS: LuaState, index: c_int) ;
    pub fn lua_replace(lS: LuaState, index: c_int) ;
    pub fn lua_resume(lS: LuaState, from: LuaState, nargs: c_int) -> c_int;
    pub fn lua_setallocf(lS: LuaState, f: LuaAlloc, ud: *c_void) ;
    pub fn lua_setfenv(lS: LuaState, index: c_int) -> c_int;
    pub fn lua_setfield(lS: LuaState, index: c_int, k: *c_char) ;
    pub fn lua_setmetatable(lS: LuaState, index: c_int) -> c_int;
    pub fn lua_settable(lS: LuaState, index: c_int) ;
    pub fn lua_settop(lS: LuaState, index: c_int) ;
    pub fn lua_status(lS: LuaState) -> c_int;
    pub fn lua_toboolean(lS: LuaState, index: c_int) -> c_int;
    pub fn lua_tocfunction(lS: LuaState, index: c_int) -> LuaCFunction;
    pub fn lua_tointeger(lS: LuaState, index: c_int) -> LuaInteger;
    pub fn lua_tolstring(lS: LuaState, index: c_int, len: *size_t) -> *c_char;
    pub fn lua_tonumber(lS: LuaState, index: c_int) -> LuaNum;
    pub fn lua_topointer(lS: LuaState, index: c_int) -> *c_void;
    pub fn lua_tothread(lS: LuaState, index: c_int) -> LuaState;
    pub fn lua_touserdata(lS: LuaState, index: c_int) -> *c_void;
    pub fn lua_type(lS: LuaState, index: c_int) -> c_int;
    pub fn lua_typename(lS: LuaState, tp: c_int) -> *c_char;
    pub fn lua_xmove(from: LuaState, to: LuaState, n: c_int) ;
    pub fn lua_yield(lS: LuaState, nresults: c_int) -> c_int;
    
    /************************************************************/
    /*                                                            */
    /*                    The stuff from lauxlib.h                */
    /*                                                            */
    /************************************************************/
    pub fn luaL_addlstring(B: LuaLBuffer, s: *c_char, l: size_t) ;
    pub fn luaL_addstring(B: LuaLBuffer, s: *c_char) ;
    pub fn luaL_addvalue(B: LuaLBuffer) ;
    pub fn luaL_argerror(lS: LuaState, narg: c_int, extramsg: *c_char) ;
    pub fn luaL_buffinit(lS: LuaState, B: LuaLBuffer) ;
    pub fn luaL_callmeta(lS: LuaState, obj: c_int, e: *c_char) -> c_int;
    pub fn luaL_checkany(lS: LuaState, narg: c_int) ;
    pub fn luaL_checkinteger(lS: LuaState, narg: c_int) -> LuaInteger;
    pub fn luaL_checklstring(lS: LuaState, narg: c_int, l: *size_t) -> *c_char;
    pub fn luaL_checknumber(lS: LuaState, narg: c_int) -> LuaNum;
    pub fn luaL_checkoption(lS: LuaState, narg: c_int, def: *c_char, lst: **c_char) -> c_int;
    pub fn luaL_checkstack(lS: LuaState, sz: c_int, msg: *c_char) ;
    pub fn luaL_checktype(lS: LuaState, narg: c_int, t: c_int) ;
    pub fn luaL_checkudata(lS: LuaState, narg: c_int, tname: *c_char) ;
    pub fn luaL_error(lS: LuaState, fmt: *c_char) -> c_int; // FIXME: should accept varargs
    pub fn luaL_getmetafield(lS: LuaState, obj: c_int, e: *c_char) -> c_int;
    pub fn luaL_gsub(lS: LuaState, s: *c_char, p: *c_char, r: *c_char) -> *c_char;
    pub fn luaL_loadbuffer(lS: LuaState, buff: *c_char, sz: size_t, name: *c_char) -> c_int;
    pub fn luaL_loadfilex(lS: LuaState, filename: *c_char, mode: *c_char) -> c_int;
    pub fn luaL_loadstring(lS: LuaState, s: *c_char) -> c_int;
    pub fn luaL_newmetatable(lS: LuaState, tname: *c_char) -> c_int;
    pub fn luaL_newstate() -> LuaState;
    pub fn luaL_openlibs(lS: LuaState) ;
    pub fn luaL_optinteger(lS: LuaState, narg: c_int, d: LuaInteger) -> LuaInteger;
    pub fn luaL_optlstring(lS: LuaState, narg: c_int, d: *c_char, l: *size_t) -> *c_char;
    pub fn luaL_optnumber(lS: LuaState, narg: c_int, d: LuaNum) -> LuaNum;
    pub fn luaL_prepbuffer(B: LuaLBuffer) -> *c_char;
    pub fn luaL_pushresult(B: LuaLBuffer) ;
    pub fn luaL_ref(lS: LuaState, t: c_int) -> c_int;
    pub fn luaL_register(lS: LuaState, libname: *c_char, l: LuaLReg) ;
    pub fn luaL_typerror(lS: LuaState, narg: c_int, tname: *c_char) -> c_int;
    pub fn luaL_unref(lS: LuaState, t: c_int, rf: c_int) ;
    pub fn luaL_where(lS: LuaState, lvl: c_int) ;
}


/****************************************************************/
/*                                                                */
/*                    Constants from lua.h                        */
/*                                                                */
/****************************************************************/

#[allow(dead_code)]
pub static LUA_MULTRET: c_int = (-1);

#[allow(dead_code)]
pub static LUA_REGISTRYINDEX: c_int = (-10000);
#[allow(dead_code)]
pub static LUA_ENVIRONINDEX: c_int = (-10001);
#[allow(dead_code)]
pub static LUA_GLOBALSINDEX: c_int = (-10002);

#[allow(dead_code)]
pub static LUA_YIELD: c_int = 1;
#[allow(dead_code)]
pub static LUA_ERRRUN: c_int = 2;
#[allow(dead_code)]
pub static LUA_ERRSYNTAX: c_int = 3;
#[allow(dead_code)]
pub static LUA_ERRMEM: c_int = 4;
#[allow(dead_code)]
pub static LUA_ERRERR: c_int = 5;

#[allow(dead_code)]
pub static LUA_TNONE: c_int = (-1);
#[allow(dead_code)]
pub static LUA_TNIL: c_int = 0;
#[allow(dead_code)]
pub static LUA_TBOOLEAN: c_int = 1;
#[allow(dead_code)]
pub static LUA_TLIGHTUSERDATA: c_int = 2;
#[allow(dead_code)]
pub static LUA_TNUMBER: c_int = 3;
#[allow(dead_code)]
pub static LUA_TSTRING: c_int = 4;
#[allow(dead_code)]
pub static LUA_TTABLE: c_int = 5;
#[allow(dead_code)]
pub static LUA_TFUNCTION: c_int = 6;
#[allow(dead_code)]
pub static LUA_TUSERDATA: c_int = 7;
#[allow(dead_code)]
pub static LUA_TTHREAD: c_int = 8;

#[allow(dead_code)]
pub static LUA_MINSTACK: c_int = 20;

#[allow(dead_code)]
pub static LUA_GCSTOP: c_int = 0;
#[allow(dead_code)]
pub static LUA_GCRESTART: c_int = 1;
#[allow(dead_code)]
pub static LUA_GCCOLLECT: c_int = 2;
#[allow(dead_code)]
pub static LUA_GCCOUNT: c_int = 3;
#[allow(dead_code)]
pub static LUA_GCCOUNTB: c_int = 4;
#[allow(dead_code)]
pub static LUA_GCSTEP: c_int = 5;
#[allow(dead_code)]
pub static LUA_GCSETPAUSE: c_int = 6;
#[allow(dead_code)]
pub static LUA_GCSETSTEPMUL: c_int = 7;


/****************************************************************/
/*                                                                */
/*                        Types from lua.h                        */
/*                                                                */
/****************************************************************/

pub type LuaAlloc = fn (ud: *c_void, ptr: *c_void, osize: size_t, nsize: size_t) -> *c_void;
pub type LuaCFunction = fn (lS: LuaState) -> c_int;
pub type LuaInteger = int;    // Int in rust should be same size as ptrdiff_t
pub type LuaNum = c_double;
pub type LuaReader = fn (lS: LuaState, data: *c_void, size: size_t) -> *c_char;
pub type LuaState = *c_void;
pub type LuaWriter = fn (lS: LuaState, p: *c_void, sz: size_t, ud: *c_void) -> c_int;


/****************************************************************/
/*                                                                */
/*        Some functions that are macros in lua.h                    */
/*                                                                */
/****************************************************************/

#[allow(dead_code)]
pub unsafe fn lua_call(lS: LuaState, nargs: c_int, nresults: c_int) {
  use std::cast::transmute;
  lua_callk(lS, nargs, nresults, 0, transmute(0))
}

#[allow(dead_code)]
pub unsafe fn lua_pcall(lS: LuaState, nargs: c_int, nresults: c_int, msgh: c_int) -> c_int {
  use std::cast::transmute;
  lua_pcallk(lS, nargs, nresults, msgh, 0, transmute(0))
}

#[allow(dead_code)]
pub unsafe fn lua_pop(lS: LuaState, n: c_int)  {
  lua_settop(lS, -(n)-1)
}

#[allow(dead_code)]
pub unsafe fn lua_newtable(lS: LuaState)  {
  lua_createtable(lS, 0, 0)
}

#[allow(dead_code)]
pub unsafe fn lua_register(lS: LuaState, name: *c_char, f: LuaCFunction)  {
  lua_pushcfunction(lS, f);
  lua_setglobal(lS, name)
}

#[allow(dead_code)]
pub unsafe fn lua_pushcfunction(lS: LuaState, f: LuaCFunction)  {
  lua_pushcclosure(lS, f, 0)
}

#[allow(dead_code)]
pub unsafe fn lua_strlen(lS: LuaState, index: c_int) -> size_t {
  lua_rawlen(lS,index)
}

#[allow(dead_code)]
pub unsafe fn lua_isboolean(lS: LuaState, index: c_int) -> c_int {
  (lua_type(lS,index) == LUA_TBOOLEAN) as c_int
}

#[allow(dead_code)]
pub unsafe fn lua_isfunction(lS: LuaState, index: c_int) -> c_int {
  (lua_type(lS,index) == LUA_TFUNCTION) as c_int
}

#[allow(dead_code)]
pub unsafe fn lua_islightuserdata(lS: LuaState, index: c_int) -> c_int {
  (lua_type(lS,index) == LUA_TLIGHTUSERDATA) as c_int
}

#[allow(dead_code)]
pub unsafe fn lua_isnil(lS: LuaState, index: c_int) -> c_int {
  (lua_type(lS,index) == LUA_TNIL) as c_int
}

#[allow(dead_code)]
pub unsafe fn lua_isnone(lS: LuaState, index: c_int) -> c_int {
  (lua_type(lS,index) == LUA_TNONE) as c_int
}

#[allow(dead_code)]
pub unsafe fn lua_isnoneornil(lS: LuaState, index: c_int) -> c_int {
  let t = lua_type(lS,index);
  (t == LUA_TNIL || t == LUA_TNONE) as c_int
}

#[allow(dead_code)]
pub unsafe fn lua_istable(lS: LuaState, index: c_int) -> c_int {
  (lua_type(lS,index) == LUA_TTABLE) as c_int
}

#[allow(dead_code)]
pub unsafe fn lua_isthread(lS: LuaState, index: c_int) -> c_int {
  (lua_type(lS,index) == LUA_TTHREAD) as c_int
}

#[allow(dead_code)]
pub unsafe fn lua_pushliteral(lS: LuaState, s: *c_char)  {
  lua_pushlstring(lS, s, strlen(s)-1)
}

#[allow(dead_code)]
pub unsafe fn lua_setglobal(lS: LuaState, name: *c_char)  {
  lua_setfield(lS, LUA_GLOBALSINDEX, name)
}

#[allow(dead_code)]
pub unsafe fn lua_getglobal(lS: LuaState, name: *c_char)  {
  lua_getfield(lS, LUA_GLOBALSINDEX, name)
}

#[allow(dead_code)]
pub unsafe fn lua_tostring(lS: LuaState, index: c_int) -> *c_char {
  lua_tolstring(lS, index, RawPtr::null())
}

/****************************************************************/
/*                                                              */
/*                  Constants from lauxlib.h                    */
/*                                                              */
/****************************************************************/
#[allow(dead_code)]
pub static LUA_ERRFILE: c_int = 6;


/****************************************************************/
/*                                                              */
/*                    Types from lauxlib.h                        */
/*                                                                */
/****************************************************************/

#[allow(dead_code)]
pub struct LuaLBuffer(*c_void);
pub type LuaLReg = *_LuaLReg;
#[allow(dead_code)]
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
#[allow(dead_code)]
pub unsafe fn luaL_argcheck(lS: LuaState, cond: c_int, narg: c_int, extramsg: *c_char)  {
    if cond != 0 {
        luaL_argerror(lS, narg, extramsg);
    }
}

#[allow(dead_code)]
pub unsafe fn luaL_checkint(lS: LuaState, narg: c_int) -> c_int {
    luaL_checkinteger(lS,narg) as c_int
}

#[allow(dead_code)]
pub unsafe fn luaL_checklong(lS: LuaState, narg: c_int) -> c_long {
    luaL_checkinteger(lS,narg) as c_long
}

#[allow(dead_code)]
pub unsafe fn luaL_checkstring(lS: LuaState, narg: c_int) -> *c_char {
    luaL_checklstring(lS,narg,RawPtr::null())
}

#[allow(dead_code)]
pub unsafe fn luaL_dofile(lS: LuaState, filename: *c_char) -> c_int {
    luaL_loadfile(lS, filename);
    lua_pcall(lS, 0, LUA_MULTRET, 0)
}

#[allow(dead_code)]
pub unsafe fn luaL_dostring(lS: LuaState, string: *c_char) -> c_int {
    luaL_loadstring(lS, string);
    lua_pcall(lS, 0, LUA_MULTRET, 0)
}

#[allow(dead_code)]
pub unsafe fn luaL_getmetatable(lS: LuaState, tname: *c_char)  {
    lua_getfield(lS, LUA_REGISTRYINDEX, tname)
}

#[allow(dead_code)]
pub unsafe fn luaL_loadfile(lS: LuaState, filename: *c_char) -> c_int {
  luaL_loadfilex(lS,filename,RawPtr::null())
}

#[allow(dead_code)]
pub unsafe fn luaL_optint(lS: LuaState, narg: c_int, d: c_int) -> c_int {
    luaL_optinteger(lS, narg, d as LuaInteger) as c_int
}

#[allow(dead_code)]
pub unsafe fn luaL_optlong(lS: LuaState, narg: c_int, d: c_long) -> c_long {
    luaL_optinteger(lS, narg, d as LuaInteger) as c_long
}

#[allow(dead_code)]
pub unsafe fn luaL_optstring(lS: LuaState, narg: c_int, d: *c_char) -> *c_char {
    luaL_optlstring(lS, narg, d, RawPtr::null())
}

#[allow(dead_code)]
pub unsafe fn luaL_typename(lS: LuaState, index: c_int) -> *c_char {
    lua_typename(lS, lua_type(lS, index))
}

/*fn main() {
    let lS: LuaState = lua::luaL_newstate();
    io::println("Hello world");
    io::println(fmt!("L = %b",is_not_null(L)));
    io::println("Now, let's load hello.lua");
    let loadresult = str::as_c_str("hello.lua",{|s| lua::luaL_loadfile(lS, s)}) as int;
    io::println(fmt!("Result of loading: %d", loadresult));
    lua::luaL_openlibs(L);
    let pcallresult = lua::lua_pcall(lS, 0, -1, 0) as int;
    io::println(fmt!("Result of pcall: %d", pcallresult));
//    runLuaWithParams(lS,~[]);
    
}
*/
