local sqlite = require('lsqlite3')
local metrics = require('metrics')
local tuning = require('tuning')

local M = {}

-- Database connections
local connections = {
    quantum = nil,
    game = nil,
    user = nil
}

-- Initialize databases with AO processes
function M.init()
    -- Quantum state database
    connections.quantum = sqlite.open('quantum_states.db')
    connections.quantum:exec[[
        CREATE TABLE IF NOT EXISTS states (
            id TEXT PRIMARY KEY,
            state BLOB,
            error_rates TEXT,
            last_updated INTEGER
        )
    ]]

    -- Game state database
    connections.game = sqlite.open('game_states.db')
    connections.game:exec[[
        CREATE TABLE IF NOT EXISTS states (
            id TEXT PRIMARY KEY,
            state BLOB,
            arweave_tx TEXT,
            last_updated INTEGER
        )
    ]]

    -- User database
    connections.user = sqlite.open('user_states.db')
    connections.user:exec[[
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            data BLOB,
            last_sync INTEGER
        )
    ]]

    -- Start AO processes
    metrics.init()
    tuning.init()
end

-- Store quantum state with metrics
function M.store_quantum_state(id, state, error_rates)
    local start_time = os.time()
    
    -- Store state
    local stmt = connections.quantum:prepare([[
        INSERT OR REPLACE INTO states (id, state, error_rates, last_updated)
        VALUES (?, ?, ?, ?)
    ]])
    
    stmt:bind(1, id)
    stmt:bind(2, state)
    stmt:bind(3, error_rates)
    stmt:bind(4, start_time)
    stmt:step()
    stmt:finalize()

    -- Record metrics
    metrics.record('quantum', 'write_time', os.time() - start_time)
    metrics.record('quantum', 'state_size', #state)

    -- Trigger tuning process
    tuning.optimize_db('quantum')
end

-- Store game state with Arweave sync
function M.store_game_state(id, state)
    local start_time = os.time()
    
    -- Store state
    local stmt = connections.game:prepare([[
        INSERT OR REPLACE INTO states (id, state, last_updated)
        VALUES (?, ?, ?)
    ]])
    
    stmt:bind(1, id)
    stmt:bind(2, state)
    stmt:bind(3, start_time)
    stmt:step()
    stmt:finalize()

    -- Record metrics
    metrics.record('game', 'write_time', os.time() - start_time)
    metrics.record('game', 'state_size', #state)

    -- Trigger Arweave sync via tuning process
    tuning.sync_to_arweave('game', id)
end

-- Query with metrics and tuning
function M.query_state(db_name, id)
    local start_time = os.time()
    
    local db = connections[db_name]
    if not db then
        return nil, "Invalid database"
    end

    local stmt = db:prepare("SELECT * FROM states WHERE id = ?")
    stmt:bind(1, id)
    
    local result = nil
    for row in stmt:nrows() do
        result = row
        break
    end
    stmt:finalize()

    -- Record query metrics
    metrics.record(db_name, 'query_time', os.time() - start_time)
    
    -- Trigger optimization if needed
    if os.time() - start_time > 0.1 then  -- Slow query threshold
        tuning.optimize_query(db_name, id)
    end

    return result
end

-- Cleanup and close
function M.cleanup()
    for _, conn in pairs(connections) do
        if conn then
            conn:close()
        end
    end
end

return M
