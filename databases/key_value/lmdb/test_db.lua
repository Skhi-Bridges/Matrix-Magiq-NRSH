local quantum_db = require('db_modules.quantum_db')
local vector_stores = require('db_modules.vector_stores')
local key_value = require('db_modules.key_value')

local function test_quantum_db()
    print("\nTesting Quantum DB...")
    
    -- Initialize
    assert(quantum_db.init({
        num_qubits = 4,
        optimization_level = 2
    }))
    
    -- Create bell state
    assert(quantum_db.create_superposition(0))
    assert(quantum_db.apply_cnot(0, 1))
    
    -- Measure and get results
    assert(quantum_db.measure(0))
    assert(quantum_db.measure(1))
    
    local probs = quantum_db.run_circuit()
    print("Bell state probabilities:", require('inspect')(probs))
    
    -- Cleanup
    assert(quantum_db.shutdown())
end

local function test_vector_stores()
    print("\nTesting Vector Stores...")
    
    -- Initialize
    assert(vector_stores.init({
        dimension = 4,
        index_type = 'hnsw'
    }))
    
    -- Add vectors
    local vectors = {
        {1, 0, 0, 0},
        {0, 1, 0, 0},
        {0, 0, 1, 0},
        {0, 0, 0, 1}
    }
    
    for i, vec in ipairs(vectors) do
        assert(vector_stores.add_vector(vec, i-1, {name = "vector_" .. i}))
    end
    
    -- Search
    local results = vector_stores.search({1, 0, 0, 0}, 2)
    print("Nearest neighbors:", require('inspect')(results))
    
    -- Cleanup
    assert(vector_stores.shutdown())
end

local function test_key_value()
    print("\nTesting Key-Value Store...")
    
    -- Initialize
    assert(key_value.init({
        backend = 'lmdb',
        max_size = 1024 * 1024 * 1024  -- 1GB
    }))
    
    -- Set values
    assert(key_value.set("test1", "value1", {type = "string"}))
    assert(key_value.set("test2", "value2", {type = "string"}))
    
    -- Get values
    local value1 = key_value.get("test1")
    print("Value1:", require('inspect')(value1))
    
    -- List keys
    local keys = key_value.list_keys("test*")
    print("Keys:", require('inspect')(keys))
    
    -- Delete
    assert(key_value.delete("test1"))
    
    -- Cleanup
    assert(key_value.shutdown())
end

local function main()
    print("Starting DB Tests...")
    
    local ok, err = pcall(function()
        test_quantum_db()
        test_vector_stores()
        test_key_value()
    end)
    
    if not ok then
        print("Error during tests:", err)
        os.exit(1)
    end
    
    print("\nAll tests completed successfully!")
end

main()
