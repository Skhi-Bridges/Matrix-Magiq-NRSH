local sqlite3 = require('lsqlite3')
local lmdb = require('lightningmdb')
local M = {}

local function check_path_access(path)
    local file = io.open(path .. "\\test.tmp", "w")
    if file then
        file:close()
        os.remove(path .. "\\test.tmp")
        return true
    end
    return false
end

function M.verify_directory_structure(paths)
    local results = {}
    for name, path in pairs(paths) do
        local exists = false
        local is_dir = false
        local is_writable = false
        
        local attr = lfs.attributes(path)
        if attr then
            exists = true
            is_dir = attr.mode == "directory"
            is_writable = check_path_access(path)
        end
        
        results[name] = {
            exists = exists,
            is_directory = is_dir,
            is_writable = is_writable
        }
    end
    return results
end

function M.verify_sqlite(db_path)
    local results = {
        database_exists = false,
        tables = {}
    }
    
    local db = sqlite3.open(db_path)
    if not db then
        return results
    end
    
    results.database_exists = true
    
    local tables = {"books", "game_boards", "code_snippets", "conversations"}
    for _, table_name in ipairs(tables) do
        local stmt = db:prepare(string.format([[
            SELECT name FROM sqlite_master 
            WHERE type='table' AND name='%s'
        ]], table_name))
        
        results.tables[table_name] = stmt:step() == sqlite3.ROW
        stmt:finalize()
    end
    
    db:close()
    return results
end

function M.verify_lmdb(lmdb_path)
    local results = {
        is_valid = false
    }
    
    local success, env = pcall(function()
        local env = lmdb.environment()
        env:open(lmdb_path, lmdb.MDB_RDONLY, 0664)
        env:close()
        return true
    end)
    
    results.is_valid = success
    if not success then
        results.error = env
    end
    
    return results
end

function M.verify_setup(storage)
    print("\n=== MCP Storage Verification Report ===\n")
    
    -- Directory Structure
    print("Directory Structure:")
    local dir_results = M.verify_directory_structure(storage.paths)
    for name, status in pairs(dir_results) do
        print(string.format("\n%s:", name))
        print(string.format("  Path: %s", storage.paths[name]))
        print(string.format("  Exists: %s", status.exists and "✓" or "✗"))
        print(string.format("  Is Directory: %s", status.is_directory and "✓" or "✗"))
        print(string.format("  Is Writable: %s", status.is_writable and "✓" or "✗"))
    end
    
    -- SQLite
    print("\nSQLite Database:")
    local db_results = M.verify_sqlite(storage.paths.root .. "\\mcp.db")
    if db_results.database_exists then
        print("  Database exists: ✓")
        print("\n  Tables:")
        for table, exists in pairs(db_results.tables) do
            print(string.format("    %s: %s", table, exists and "✓" or "✗"))
        end
    else
        print("  Database verification failed")
    end
    
    -- LMDB
    print("\nLMDB Environment:")
    local lmdb_results = M.verify_lmdb(storage.paths.lmdb)
    print(string.format("  LMDB is valid: %s", lmdb_results.is_valid and "✓" or "✗"))
    if not lmdb_results.is_valid then
        print(string.format("  Error: %s", lmdb_results.error or "Unknown error"))
    end
end

return M
