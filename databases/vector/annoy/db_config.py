from typing import Dict, Any
import os
from dotenv import load_dotenv
import asyncio
from concurrent.futures import ProcessPoolExecutor
import sqlite3
import lmdb
import rocksdb
from rpy2 import robjects
import janusgraph_python.structure.graph as jg
import faiss
import numpy as np
from pathlib import Path
import nixops  # For Nix integration

load_dotenv()

# Nix configuration for multi-language runtime
NIX_CONFIG = {
    "lua": {
        "runtime": "lua5.4",
        "packages": ["luajit", "luarocks"]
    },
    "rust": {
        "runtime": "rust-nightly",
        "packages": ["cargo", "rustc"]
    },
    "r": {
        "runtime": "R",
        "packages": ["rserve", "stats", "MASS"]
    }
}

# Vector similarity metrics for Annoy
ANNOY_METRICS = [
    "angular",
    "euclidean",
    "manhattan",
    "hamming",
    "dot",
    "cosine",
    "jaccard"
]

# Database Configurations
DB_CONFIGS = {
    "vector_stores": {
        "annoy": {
            "dimension": 1536,
            "metrics": ANNOY_METRICS,
            "n_trees": 10,
            "enabled": True
        },
        "hnsw": {
            "dimension": 1536,
            "ef_construction": 200,
            "M": 16,
            "enabled": True
        },
        "faiss": {
            "dimension": 1536,
            "index_type": "IVFPQx8",  # IVF with Product Quantization
            "nlist": 100,  # Number of centroids
            "gpu_id": 0 if faiss.get_num_gpus() > 0 else None,
            "enabled": True,
            "persistence": {
                "store": "rocksdb",
                "path": "./data/faiss_indexes"
            }
        }
    },
    "graph_stores": {
        "janusgraph": {
            "host": os.getenv("JANUSGRAPH_HOST", "localhost"),
            "port": int(os.getenv("JANUSGRAPH_PORT", "8182")),
            "enabled": True
        },
        "graphql": {
            "endpoint": os.getenv("GRAPHQL_ENDPOINT"),
            "enabled": True
        }
    },
    "key_value_stores": {
        "lmdb": {
            "path": "./data/lmdb",
            "map_size": 1024 * 1024 * 1024,  # 1GB
            "enabled": True
        },
        "rocksdb": {
            "path": "./data/rocksdb",
            "enabled": True
        }
    },
    "statistical_stores": {
        "r_statistical": {
            "packages": ["stats", "MASS", "cluster"],
            "enabled": True
        }
    },
    "cache_and_index": {
        "sqlite": {
            "path": "./data/sqlite/main.db",
            "cache_size": -2000000,  # 2GB cache
            "journal_mode": "WAL",
            "synchronous": "NORMAL",
            "enabled": True,
            "arweave_sync": True
        },
        "inverted_index": {
            "path": "./data/inverted_index",
            "analyzer": "standard",
            "index_options": ["DOCS", "FREQS", "POSITIONS"],
            "enabled": True,
            "store": "sqlite"  # Store inverted index in SQLite
        }
    },
    "weave_db": {
        "arweave_wallet": os.getenv("ARWEAVE_WALLET_PATH"),
        "sqlite_cache": "./data/sqlite/main.db",  # Link to SQLite cache
        "enabled": True
    }
}

class DatabaseManager:
    def __init__(self):
        self.connections = {}
        self.process_pool = ProcessPoolExecutor(max_workers=len(DB_CONFIGS))
        self.nix_runtime = self._init_nix_runtime()
        
    def _init_nix_runtime(self):
        """Initialize Nix runtime for multi-language support"""
        try:
            runtime = nixops.create_runtime(NIX_CONFIG)
            runtime.add_lua_path("./lightning_modules/lua")
            runtime.add_rust_path("./lightning_modules/rust")
            return runtime
        except Exception as e:
            print(f"Error initializing Nix runtime: {str(e)}")
            return None

    async def init_connections(self):
        """Initialize connections to all enabled databases"""
        for category, stores in DB_CONFIGS.items():
            if isinstance(stores, dict):
                for store_name, config in stores.items():
                    if isinstance(config, dict) and config.get("enabled"):
                        await self._init_store(category, store_name, config)
            elif stores.get("enabled"):
                await self._init_store(None, category, stores)

    async def _init_store(self, category: str, store_name: str, config: Dict[str, Any]):
        """Initialize specific database store"""
        connection_key = f"{category}_{store_name}" if category else store_name
        
        try:
            if store_name == "sqlite":
                conn = sqlite3.connect(config["path"])
                conn.execute(f"PRAGMA cache_size = {config['cache_size']}")
                conn.execute(f"PRAGMA journal_mode = {config['journal_mode']}")
                conn.execute(f"PRAGMA synchronous = {config['synchronous']}")
                self.connections[connection_key] = conn
            
            elif store_name == "faiss":
                index = faiss.index_factory(config["dimension"], config["index_type"])
                if config["gpu_id"] is not None:
                    index = faiss.index_cpu_to_gpu(
                        faiss.StandardGpuResources(),
                        config["gpu_id"],
                        index
                    )
                self.connections[connection_key] = index
            
            elif store_name == "lmdb":
                self.connections[connection_key] = lmdb.open(
                    config["path"],
                    map_size=config["map_size"]
                )
            
            elif store_name == "rocksdb":
                opts = rocksdb.Options()
                opts.create_if_missing = True
                self.connections[connection_key] = rocksdb.DB(config["path"], opts)
            
            elif store_name == "r_statistical":
                for package in config["packages"]:
                    robjects.r(f'library({package})')
                self.connections[connection_key] = robjects.r
            
            elif store_name == "janusgraph":
                self.connections[connection_key] = jg.Graph(
                    f"ws://{config['host']}:{config['port']}/gremlin"
                )

            # Add other database initializations as needed
            
        except Exception as e:
            print(f"Error initializing {store_name}: {str(e)}")
            self.connections[connection_key] = None

    async def query_all_dbs(self, query_func: callable, *args, **kwargs) -> Dict[str, Any]:
        """Execute query across all enabled databases in parallel"""
        tasks = []
        for conn_name, connection in self.connections.items():
            if connection is not None:
                task = asyncio.get_event_loop().run_in_executor(
                    self.process_pool,
                    query_func,
                    connection,
                    *args,
                    **kwargs
                )
                tasks.append((conn_name, task))
        
        results = {}
        for conn_name, task in tasks:
            try:
                results[conn_name] = await task
            except Exception as e:
                results[conn_name] = {"error": str(e)}
                
        return results

    def close_all(self):
        """Close all database connections"""
        self.process_pool.shutdown(wait=True)
        for conn_name, connection in self.connections.items():
            try:
                if hasattr(connection, 'close'):
                    connection.close()
                elif conn_name.startswith('lmdb'):
                    connection.close()
                elif conn_name.startswith('rocksdb'):
                    del connection
            except Exception as e:
                print(f"Error closing {conn_name}: {str(e)}")

# Global instance
db_manager = DatabaseManager()
