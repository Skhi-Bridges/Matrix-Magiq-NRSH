local ffi = require('ffi')
local db_core = require('db_modules.db_core')
local json = require('cjson')

local M = {}

-- Create vector store process
local process = db_core.create_db_process('vector_stores', {
    dimension = 384,
    index_type = 'hnsw'  -- or 'annoy'
})

-- FFI definitions for HNSW
ffi.cdef[[
    typedef struct HnswIndex HnswIndex;
    HnswIndex* hnswlib_init(int dim, const char* space);
    void hnswlib_free(HnswIndex* index);
    int hnswlib_add_item(HnswIndex* index, const float* vector, int id);
    void hnswlib_search(HnswIndex* index, const float* query, int k, int* labels, float* distances);
]]

local hnswlib = ffi.load('hnswlib')

-- Initialize vector store
function M.init(opts)
    opts = opts or {}
    process.dimension = opts.dimension or 384
    process.index_type = opts.index_type or 'hnsw'
    
    if process.index_type == 'hnsw' then
        process.index = hnswlib.hnswlib_init(process.dimension, "l2")
    else
        -- Initialize Annoy index via FFI if needed
    end
    
    process:record_metric('init', 1)
    return true
end

-- Add vector to store
function M.add_vector(vector, id, metadata)
    if #vector ~= process.dimension then
        error(string.format("Vector dimension mismatch. Expected %d, got %d", 
            process.dimension, #vector))
    end
    
    -- Convert vector to float array
    local vec = ffi.new("float[?]", process.dimension)
    for i = 1, process.dimension do
        vec[i-1] = vector[i]
    end
    
    -- Store metadata
    if metadata then
        process.metadata = process.metadata or {}
        process.metadata[id] = json.encode(metadata)
    end
    
    -- Add to index
    if process.index_type == 'hnsw' then
        local ret = hnswlib.hnswlib_add_item(process.index, vec, id)
        if ret ~= 0 then
            error("Failed to add item to HNSW index")
        end
    end
    
    process:record_metric('add_vector', 1)
    return true
end

-- Search vectors
function M.search(query_vector, k)
    if #query_vector ~= process.dimension then
        error(string.format("Query vector dimension mismatch. Expected %d, got %d", 
            process.dimension, #query_vector))
    end
    
    -- Convert query vector to float array
    local vec = ffi.new("float[?]", process.dimension)
    for i = 1, process.dimension do
        vec[i-1] = query_vector[i]
    end
    
    -- Prepare result arrays
    local labels = ffi.new("int[?]", k)
    local distances = ffi.new("float[?]", k)
    
    -- Search
    if process.index_type == 'hnsw' then
        hnswlib.hnswlib_search(process.index, vec, k, labels, distances)
    end
    
    -- Convert results to Lua tables
    local results = {}
    for i = 0, k-1 do
        local id = labels[i]
        table.insert(results, {
            id = id,
            distance = distances[i],
            metadata = process.metadata and process.metadata[id] or nil
        })
    end
    
    process:record_metric('search', 1)
    return results
end

-- Cleanup
function M.shutdown()
    if process.index and process.index_type == 'hnsw' then
        hnswlib.hnswlib_free(process.index)
    end
    process:record_metric('shutdown', 1)
    return true
end

return M
