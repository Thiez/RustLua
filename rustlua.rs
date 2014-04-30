extern crate libc;

use std::ptr::RawPtr;

mod lua_raw;
mod lua_wrapper;

fn main() {
  let lS = unsafe{lua_raw::luaL_newstate()};
  println!("L = {}",lS.is_not_null());
  println!("Now, let's load hello.lua");
  let loadresult = "hello.lua".with_c_str(|fname|unsafe{lua_raw::luaL_loadfile(lS, fname) as int});
  println!("Result of loading: {}", loadresult);
  unsafe{lua_raw::luaL_openlibs(lS)};
  let pcallresult = unsafe{lua_raw::lua_pcall(lS, 0, -1, 0) as int};
  println!("Result of pcall: {}", pcallresult);
  
  println!("Now with wrapper:");
  let mut lS = lua_wrapper::LuaStack::new();
  lS.openlibs();
  lS.loadFile("script.lua");
  unsafe{
    lua_raw::lua_newtable(lS.state);
    lua_raw::lua_pushinteger(lS.state, 1);
    lua_raw::lua_pushinteger(lS.state, 1);
  }
  for n in range(1,5) {
    lS.pushi(n);
    lS.pushi(n*n);
    unsafe{lua_raw::lua_rawset(lS.state,-3)};
  };
  println!("Pushing args");

  unsafe{
    "foo".with_c_str(|s|lua_raw::lua_setglobal(lS.state,s));
    "magics!".with_c_str(|s|lua_raw::lua_pushstring(lS.state,s));
    "magic".with_c_str(|s|lua_raw::lua_setglobal(lS.state,s));
  }
  lS.pcall(0,-1);
}
