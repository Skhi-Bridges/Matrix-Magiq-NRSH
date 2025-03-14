-- db_manager.lua
local M = {}

-- Vector Store Management
M.vector_stores = {
  faiss = require('vector_stores.faiss'),
  annoy = require('vector_stores.annoy'),
  hnsw = require('vector_stores.hnsw')
}

-- Key-Value Store Management
M.key_value_stores = {
  lmdb = require('key_value_stores.lmdb'),
  rocksdb = require('key_value_stores.rocksdb')
}

-- Index Store Management
M.index_stores = {
  sqlite = require('index_stores.sqlite'),
  inverted_index = require('index_stores.inverted_index')
}

-- Statistical Store Management
M.statistical_stores = {
  r_statistical = require('statistical_stores.r_statistical')
}

-- Blockchain Store Management
M.blockchain_stores = {
  weavedb = require('blockchain_stores.weavedb')
}

return M
