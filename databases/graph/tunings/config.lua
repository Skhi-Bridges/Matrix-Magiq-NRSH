-- tunings_config.lua
return {
    annoy = {
        dimension = 128,
        n_trees = 10,
        search_k = -1,
    },
    hnsw = {
        dimension = 128,
        m = 16,
        ef_construction = 200,
    },
    lmdb = {
        map_size = 1099511627776, -- 1TB
    },
    rocksdb = {
        create_if_missing = true,
    },
    inverted_indexes = {
        index_type = 'inverted',
    },
    pq = {
        dimension = 128,
        m = 8,
        nbits = 8,
    },
    sqlite3 = {
        journal_mode = 'WAL',
    },
    lsh = {
        dimension = 128,
        num_hashes = 128,
        num_probes = 64,
    },
    janus = {
        dimension = 128,
    },
    weave_db = {
        dimension = 128,
        weave_factor = 10,
    },
    quantum_db = {
        qubit_count = 16,
    },
    extended_quantum_db = {
        extended_qubit_count = 32,
    },
}
