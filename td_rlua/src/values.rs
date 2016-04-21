use std::ffi::{CStr, CString};
use std::mem;

use td_clua;
use td_clua::lua_State;
use libc;

use LuaRead;
use LuaPush;

macro_rules! integer_impl(
    ($t:ident) => (
        impl LuaPush for $t {
            fn push_to_lua(self, lua: *mut lua_State) -> i32 {
                unsafe { td_clua::lua_pushinteger(lua, self as td_clua::lua_Integer) };
                1
            }
        }

        impl LuaRead for $t {
            fn lua_read_with_pop(lua: *mut lua_State, index: i32, _pop: i32) -> Option<$t> {
                let mut success = unsafe { mem::uninitialized() };
                let val = unsafe { td_clua::lua_tointegerx(lua, index, &mut success) };
                match success {
                    0 => None,
                    _ => Some(val as $t)
                }
            }
        }
    );
);

integer_impl!(i8);
integer_impl!(i16);
integer_impl!(i32);
integer_impl!(u8);
integer_impl!(u16);
integer_impl!(u32);
integer_impl!(usize);

macro_rules! numeric_impl(
    ($t:ident) => (
        impl LuaPush for $t {
            fn push_to_lua(self, lua: *mut lua_State) -> i32 {
                unsafe { td_clua::lua_pushnumber(lua, self as f64) };
                1
            }
        }

        impl LuaRead for $t {
            fn lua_read_with_pop(lua: *mut lua_State, index: i32, _pop: i32) -> Option<$t> {
                let mut success = unsafe { mem::uninitialized() };
                let val = unsafe { td_clua::lua_tonumberx(lua, index, &mut success) };
                match success {
                    0 => None,
                    _ => Some(val as $t)
                }
            }
        }
    );
);

numeric_impl!(f32);
numeric_impl!(f64);

impl LuaPush for String {
    fn push_to_lua(self, lua: *mut lua_State) -> i32 {
        let value = CString::new(&self[..]).unwrap();
        unsafe { td_clua::lua_pushstring(lua, value.as_ptr()) };
        1
    }
}

impl LuaRead for String {
    fn lua_read_with_pop(lua: *mut lua_State, index: i32, _pop: i32) -> Option<String> {
        let mut size: libc::size_t = unsafe { mem::uninitialized() };
        let c_str_raw = unsafe { td_clua::lua_tolstring(lua, index, &mut size) };
        if c_str_raw.is_null() {
            return None;
        }

        let c_str = unsafe { CStr::from_ptr(c_str_raw) };
        let c_str = String::from_utf8_lossy(c_str.to_bytes());
        Some(c_str.to_string())
    }
}

impl<'s> LuaPush for &'s str {
    fn push_to_lua(self, lua: *mut lua_State) -> i32 {
        let value = CString::new(&self[..]).unwrap();
        unsafe { td_clua::lua_pushstring(lua, value.as_ptr()) };
        1
    }
}

impl LuaPush for bool {
    fn push_to_lua(self, lua: *mut lua_State) -> i32 {
        unsafe { td_clua::lua_pushboolean(lua, self.clone() as libc::c_int) };
        1
    }
}

impl LuaRead for bool {
    fn lua_read_with_pop(lua: *mut lua_State, index: i32, _pop: i32) -> Option<bool> {
        if unsafe { td_clua::lua_isboolean(lua, index) } != true {
            return None;
        }

        Some(unsafe { td_clua::lua_toboolean(lua, index) != 0 })
    }
}

impl LuaPush for () {
    fn push_to_lua(self, lua: *mut lua_State) -> i32 {
        unsafe { td_clua::lua_pushnil(lua) };
        1
    }
}

impl LuaRead for () {
    fn lua_read_with_pop(_: *mut lua_State, _: i32, _pop: i32) -> Option<()> {
        Some(())
    }
}
