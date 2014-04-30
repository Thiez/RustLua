
use super::lua_raw;
use super::lua_raw::LuaNum;
use super::lua_raw::{LUA_TNONE,LUA_TNIL,LUA_TBOOLEAN,LUA_TLIGHTUSERDATA,LUA_TNUMBER,LUA_TSTRING,
                     LUA_TTABLE,LUA_TFUNCTION,LUA_TUSERDATA,LUA_TTHREAD};

use std::c_str::CString;
use libc::{c_int,c_void};

pub struct LuaStack {
  pub state: lua_raw::LuaState
}

#[allow(dead_code)]
impl LuaStack {
  pub fn new() -> LuaStack {
    unsafe {
      LuaStack{ state: lua_raw::luaL_newstate() }
    }
  }
  pub fn hi() { println!("Hello world"); }
  pub fn pushi<T: LuaConvert>(&mut self, item: T) {
    self.push(item.toLuaItem())
  }
  pub fn push(&mut self, item: LuaStackItem) {
    match item {
      LuaNone => fail!("Cannot push None!"),
      LuaNil => unsafe { lua_raw::lua_pushnil(self.state) },
      LuaBool(b) => unsafe { lua_raw::lua_pushboolean(self.state,b as c_int) },
      LuaLightUserData(v) => unsafe { lua_raw::lua_pushlightuserdata(self.state,v) },
      LuaNumber(n) => unsafe { lua_raw::lua_pushnumber(self.state,n) },
      LuaString(s) => unsafe { s.with_ref(|cstr|lua_raw::lua_pushstring(self.state,cstr)) },
      LuaTable(_) => fail!("Pushing table not yet implemented"),
      LuaFunction(_) => fail!("Pushing function not yet implemented"),
      LuaUserData(_) => fail!("Pushing userdata not yet implemented"),
      LuaThread(_) => fail!("Pushing thread not yet implemented")
    };
  }
  pub fn pop(&mut self) -> LuaStackItem {
    let n = unsafe { lua_raw::lua_gettop(self.state) };
    if n < 1 {
      LuaNone
    } else {
      let result =
      match unsafe{lua_raw::lua_type(self.state,n)} {
        LUA_TNONE => LuaNone,
        LUA_TNIL => LuaNil,
        LUA_TBOOLEAN => LuaBool( unsafe { lua_raw::lua_toboolean(self.state,n) == 1 } ),
        LUA_TLIGHTUSERDATA => LuaLightUserData( unsafe { lua_raw::lua_touserdata(self.state,n) } ),
        LUA_TNUMBER => fail!(),
        LUA_TSTRING => fail!(),
        LUA_TTABLE => fail!(),
        LUA_TFUNCTION => fail!(),
        LUA_TUSERDATA => LuaUserData( unsafe { lua_raw::lua_touserdata(self.state,n) } ),
        LUA_TTHREAD => LuaThread( unsafe { LuaStack{ state: lua_raw::lua_tothread(self.state,n) } } ),
        _ => LuaNone
      };
      unsafe { lua_raw::lua_pop(self.state,1) };
      result
    }
  }
  pub fn call(&mut self, nargs: int, results: int) {
    unsafe { lua_raw::lua_call(self.state, nargs as c_int, results as c_int) }
  }
  pub fn pcall(&mut self, nargs: int, results: int) -> int {
    unsafe { lua_raw::lua_pcall(self.state, nargs as c_int, results as c_int, 0) as int }
  }
  pub fn openlibs(&mut self) {
    unsafe { lua_raw::luaL_openlibs(self.state) }
  }
  pub fn loadFile(&mut self, file: &str) {
    let loadresult = file.with_c_str(|fname|unsafe{lua_raw::luaL_loadfile(self.state, fname) as int});
    if loadresult != 0 {
      fail!("Some failure reading file")
    }
  }
}

pub enum LuaStackItem {
  LuaNone,
  LuaNil,
  LuaBool(bool),
  LuaLightUserData(*c_void),
  LuaNumber(LuaNum),
  LuaString(CString),
  LuaTable(c_void),
  LuaFunction(c_void),
  LuaUserData(*c_void),
  LuaThread(LuaStack)
}

pub trait LuaConvert {
  fn toLuaItem(&self) -> LuaStackItem;
}

impl LuaConvert for uint {
  fn toLuaItem(&self) -> LuaStackItem { LuaNumber(*self as LuaNum) }
}

impl LuaConvert for int {
  fn toLuaItem(&self) -> LuaStackItem { LuaNumber(*self as LuaNum) }
}

impl LuaConvert for f32 {
  fn toLuaItem(&self) -> LuaStackItem { LuaNumber(*self as LuaNum) }
}

impl LuaConvert for f64 {
  fn toLuaItem(&self) -> LuaStackItem { LuaNumber(*self as LuaNum) }
}

impl LuaConvert for bool {
  fn toLuaItem(&self) -> LuaStackItem { LuaBool(*self) }
}

impl<'a> LuaConvert for &'a str {
  fn toLuaItem(&self) -> LuaStackItem { LuaString(self.to_c_str()) }
}

