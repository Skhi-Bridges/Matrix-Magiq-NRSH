local config = {
    quantum_db = {
        enabled = true,
        max_connections = 10,
        auto_tune = false
    },
    vector_stores = {
        enabled = true,
        implementations = {
            annoy = { dimension = 128, trees = 10 },
            hnsw = { M = 16, efConstruction = 200 },
            lsh = { hash_tables = 4, hash_size = 24 },
            pq = { clusters = 256, bytes_per_vector = 8 }
        }
    },
    key_value = {
        enabled = true,
        implementations = {
            lmdb = { map_size = 1024 * 1024 * 1024 }, -- 1GB
            rocksdb = { create_if_missing = true },
            sqlite3 = { journal_mode = "WAL" }
        }
    }
}

return config
