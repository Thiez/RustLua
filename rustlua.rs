extern crate libc;

use std::ptr::RawPtr;

mod lua_raw;
mod lua_wrapper;

fn main() {
  let L = unsafe{lua_raw::luaL_newstate()};
  println!("Hello world");
  println!("L = {}",L.is_not_null());
  println!("Now, let's load hello.lua");
  let loadresult = "hello.lua".with_c_str(|fname|unsafe{lua_raw::luaL_loadfile(L, fname) as int});
  println!("Result of loading: {}", loadresult);
  unsafe{lua_raw::luaL_openlibs(L)};
  let pcallresult = unsafe{lua_raw::lua_pcall(L, 0, -1, 0) as int};
  println!("Result of pcall: {}", pcallresult);
//  runLuaWithParams(L,~[]);
  
  println!("Now with wrapper:");
  let mut S = lua_wrapper::LuaStack::new();
  S.openlibs();
  S.loadFile("script.lua");
  unsafe{lua_raw::lua_newtable(S.state)};
//  lua_pushinteger(S.state, 1);
//  lua_pushinteger(S.state, 1);
  for n in range(1,5) {
    S.pushi(n);
    S.pushi(n*n);
    unsafe{lua_raw::lua_rawset(S.state,-3)};
  };

  unsafe{
    "foo".with_c_str(|s|lua_raw::lua_setglobal(S.state,s));
    "magics!".with_c_str(|s|lua_raw::lua_pushstring(S.state,s));
    "magic".with_c_str(|s|lua_raw::lua_setglobal(S.state,s));
  }
  S.pcall(0,-1);
}
