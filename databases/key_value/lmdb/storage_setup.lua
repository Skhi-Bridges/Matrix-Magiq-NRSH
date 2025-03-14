local drive_detector = require('db_modules.utils.drive_detector')
local sqlite3 = require('lsqlite3')
local lmdb = require('lightningmdb')
local M = {}

local function ensure_directory(path)
    local ok, err = os.execute('mkdir "' .. path .. '" 2>nul')
    return ok ~= nil
end

local StorageSetup = {}
StorageSetup.__index = StorageSetup

function StorageSetup.new(preferred_drive)
    local self = setmetatable({}, StorageSetup)
    self.preferred_drive = preferred_drive or "C"
    self:setup_storage_drive()
    self:setup_paths()
    self:init_storage()
    return self
end

function StorageSetup:setup_storage_drive()
    local drives = drive_detector.get_available_drives()
    local preferred = self.preferred_drive .. ":"
    
    -- Check preferred drive
    if drives[preferred] then
        local info = drive_detector.get_drive_info(preferred)
        if info and info.free_gb > 10 then
            self.storage_drive = preferred
            return
        end
    end
    
    -- Find alternative
    local best_drive, max_free = nil, 10
    for drive, info in pairs(drives) do
        local info = drive_detector.get_drive_info(drive)
        if info and info.free_gb > max_free then
            max_free = info.free_gb
            best_drive = drive
        end
    end
    
    if not best_drive then
        error("No suitable drive found with at least 10GB free space")
    end
    
    self.storage_drive = best_drive
end

function StorageSetup:setup_paths()
    self.root_path = self.storage_drive .. "\\mcp_storage"
    
    self.paths = {
        root = self.root_path,
        embeddings = self.root_path .. "\\embeddings",
        lmdb = self.root_path .. "\\lmdb",
        cache = self.root_path .. "\\cache",
        books = self.root_path .. "\\books",
        games = self.root_path .. "\\games",
        code = self.root_path .. "\\code"
    }
    
    -- Create directories
    for _, path in pairs(self.paths) do
        ensure_directory(path)
    end
end

function StorageSetup:init_storage()
    self:init_sqlite()
    self:init_lmdb()
    self:set_env_variables()
end

function StorageSetup:init_sqlite()
    local db_path = self.paths.root .. "\\mcp.db"
    self.db = sqlite3.open(db_path)
    
    -- Create tables
    self.db:exec([[
        CREATE TABLE IF NOT EXISTS books (
            id TEXT PRIMARY KEY,
            chapter_number INTEGER,
            title TEXT,
            content TEXT,
            metadata TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS game_boards (
            id TEXT PRIMARY KEY,
            name TEXT,
            config TEXT,
            assets TEXT,
            metadata TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS code_snippets (
            id TEXT PRIMARY KEY,
            language TEXT,
            content TEXT,
            metadata TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS conversations (
            id TEXT PRIMARY KEY,
            user_input TEXT,
            ai_response TEXT,
            metadata TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    ]])
end

function StorageSetup:init_lmdb()
    local env = lmdb.environment()
    env:set_mapsize(10 * 1024 * 1024 * 1024) -- 10GB
    env:set_maxreaders(126) -- Windows limitation
    env:open(self.paths.lmdb, 
        lmdb.MDB_FIXEDMAP + 
        lmdb.MDB_NOSYNC + 
        lmdb.MDB_NOTLS, 
        0664)
    self.lmdb_env = env
end

function StorageSetup:set_env_variables()
    os.setenv("MCP_STORAGE_PATH", self.paths.root)
    os.setenv("MCP_EMBEDDINGS_PATH", self.paths.embeddings)
    os.setenv("MCP_LMDB_PATH", self.paths.lmdb)
    os.setenv("MCP_CACHE_PATH", self.paths.cache)
end

function StorageSetup:close()
    if self.db then
        self.db:close()
    end
    if self.lmdb_env then
        self.lmdb_env:close()
    end
end

M.create = function(preferred_drive)
    return StorageSetup.new(preferred_drive)
end

return M
