-- LuaRocks configuration file

-- Use local tree by default
local_by_default = true

-- Rocks servers
rocks_servers = {
   "https://luarocks.org/repositories/rocks",
   "https://luarocks.org/dev"
}

-- Lua version and paths
lua_version = "5.4"
lua_interpreter = "lua"

-- Build settings
variables = {
   LUA_INCDIR = "$(LUA_BINDIR)/../include",
   LUA_LIBDIR = "$(LUA_BINDIR)/../lib"
}

-- Project specific paths
rocks_trees = {
   { name = "user", root = home .. "/.luarocks" },
   { name = "project", root = "../.luarocks" }
}

-- Development settings
local_cache = "local-cache"
upload_server = "https://luarocks.org"
upload_user = os.getenv("LUAROCKS_USER")
upload_api_key = os.getenv("LUAROCKS_API_KEY")
