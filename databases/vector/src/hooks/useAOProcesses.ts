import { useState, useEffect } from 'react';
import { create } from 'zustand';

interface DBProcess {
  id: string;
  name: string;
  type: 'vector' | 'graph' | 'key_value' | 'statistical' | 'index' | 'blockchain';
  status: 'active' | 'idle' | 'error';
  luaScript?: string;
}

interface AOState {
  processes: DBProcess[];
  activeProcesses: string[];
  addProcess: (process: DBProcess) => void;
  removeProcess: (id: string) => void;
  updateProcessStatus: (id: string, status: DBProcess['status']) => void;
}

// Database Process Types in AO System
const DB_PROCESSES = {
  VECTOR_STORES: [
    { name: 'Annoy', type: 'vector', description: 'Approximate Nearest Neighbors', luaScript: 'vector_stores.annoy' },
    { name: 'HNSW', type: 'vector', description: 'Hierarchical Navigable Small World', luaScript: 'vector_stores.hnsw' },
    { name: 'LSH', type: 'vector', description: 'Locality-Sensitive Hashing', luaScript: 'vector_stores.lsh' },
    { name: 'PQ', type: 'vector', description: 'Product Quantization', luaScript: 'vector_stores.pq' }
  ],
  GRAPH_STORES: [
    { name: 'JanusGraph', type: 'graph', description: 'Distributed Graph Database', luaScript: 'graph_stores.janusgraph' },
    { name: 'GraphQL', type: 'graph', description: 'GraphQL Query Layer', luaScript: 'graph_stores.graphql' }
  ],
  KEY_VALUE_STORES: [
    { name: 'Redis', type: 'key_value', description: 'In-Memory Data Store', luaScript: 'key_value_stores.redis' },
    { name: 'LevelDB', type: 'key_value', description: 'Key-Value Store', luaScript: 'key_value_stores.leveldb' }
  ],
  STATISTICAL_STORES: [
    { name: 'TimescaleDB', type: 'statistical', description: 'Time-Series Database', luaScript: 'statistical_stores.timescaledb' },
    { name: 'InfluxDB', type: 'statistical', description: 'Time-Series Data Platform', luaScript: 'statistical_stores.influxdb' }
  ],
  INDEX_STORES: [
    { name: 'Elasticsearch', type: 'index', description: 'Search Engine', luaScript: 'index_stores.elasticsearch' },
    { name: 'Meilisearch', type: 'index', description: 'Search API', luaScript: 'index_stores.meilisearch' }
  ],
  BLOCKCHAIN_STORES: [
    { name: 'Arweave', type: 'blockchain', description: 'Permanent Storage', luaScript: 'blockchain_stores.arweave' },
    { name: 'IPFS', type: 'blockchain', description: 'Distributed File System', luaScript: 'blockchain_stores.ipfs' }
  ]
};

// Create AO Store
const useAOStore = create<AOState>((set) => ({
  processes: [],
  activeProcesses: [],
  addProcess: (process) => 
    set((state) => ({ 
      processes: [...state.processes, process] 
    })),
  removeProcess: (id) =>
    set((state) => ({
      processes: state.processes.filter((p) => p.id !== id)
    })),
  updateProcessStatus: (id, status) =>
    set((state) => ({
      processes: state.processes.map((p) =>
        p.id === id ? { ...p, status } : p
      )
    }))
}));

export const useAOProcesses = () => {
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const { processes, addProcess, removeProcess, updateProcessStatus } = useAOStore();

  const startProcess = async (processType: string, processName: string) => {
    try {
      const process = {
        id: `${processType}-${processName}-${Date.now()}`,
        name: processName,
        type: processType as DBProcess['type'],
        status: 'idle' as const
      };

      // Add process to store
      addProcess(process);

      // Start Lua process via AO
      const luaScript = DB_PROCESSES[processType].find(p => p.name === processName)?.luaScript;
      if (luaScript) {
        // TODO: Implement AO process start with Lua script
        await fetch('/api/ao/start', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ processId: process.id, luaScript })
        });
        updateProcessStatus(process.id, 'active');
      }
    } catch (err) {
      setError(err.message);
    }
  };

  const stopProcess = async (processId: string) => {
    try {
      // Stop AO process
      await fetch('/api/ao/stop', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ processId })
      });
      
      // Remove from store
      removeProcess(processId);
    } catch (err) {
      setError(err.message);
    }
  };

  useEffect(() => {
    const initializeProcesses = async () => {
      try {
        // TODO: Fetch running AO processes
        const response = await fetch('/api/ao/processes');
        const runningProcesses = await response.json();
        
        runningProcesses.forEach((process: DBProcess) => {
          addProcess(process);
        });
        
        setIsLoading(false);
      } catch (err) {
        setError(err.message);
        setIsLoading(false);
      }
    };

    initializeProcesses();
  }, []);

  return {
    processes,
    isLoading,
    error,
    startProcess,
    stopProcess,
    dbProcessTypes: DB_PROCESSES
  };
};
