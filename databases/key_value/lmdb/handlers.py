import numpy as np
from annoy import AnnoyIndex
import hnswlib
import lmdb
import sqlite3
import os
import json

class DatabaseManager:
    def __init__(self, tunings):
        self.tunings = tunings
        self.env = {}
        self.initialize_databases()

    def initialize_databases(self):
        # Initialize Annoy
        self.env['annoy'] = AnnoyIndex(self.tunings['annoy']['dimension'], 'angular')
        
        # Initialize HNSW
        self.env['hnsw'] = hnswlib.Index(space='l2', dim=self.tunings['hnsw']['dimension'])
        self.env['hnsw'].init_index(
            max_elements=10000,
            ef_construction=self.tunings['hnsw']['ef_construction'],
            M=self.tunings['hnsw']['m']
        )
        
        # Initialize LMDB
        os.makedirs('databases/lmdb', exist_ok=True)
        self.env['lmdb'] = lmdb.open(
            'databases/lmdb/data.mdb',
            map_size=self.tunings['lmdb']['map_size']
        )
        
        # Initialize SQLite
        os.makedirs('databases/sqlite', exist_ok=True)
        self.env['sqlite'] = sqlite3.connect('databases/sqlite/vectors.db')
        self._setup_sqlite_tables()

    def _setup_sqlite_tables(self):
        cursor = self.env['sqlite'].cursor()
        cursor.execute('''
        CREATE TABLE IF NOT EXISTS vectors (
            id INTEGER PRIMARY KEY,
            vector BLOB,
            metadata TEXT
        )
        ''')
        self.env['sqlite'].commit()

    def add_vector(self, db_type, vector_id, vector, metadata=None):
        if db_type == 'annoy':
            self.env['annoy'].add_item(vector_id, vector)
        elif db_type == 'hnsw':
            self.env['hnsw'].add_items(np.array([vector]), np.array([vector_id]))
        elif db_type == 'lmdb':
            with self.env['lmdb'].begin(write=True) as txn:
                txn.put(
                    str(vector_id).encode(),
                    json.dumps({'vector': vector, 'metadata': metadata}).encode()
                )
        elif db_type == 'sqlite':
            cursor = self.env['sqlite'].cursor()
            cursor.execute(
                'INSERT INTO vectors (id, vector, metadata) VALUES (?, ?, ?)',
                (vector_id, json.dumps(vector), json.dumps(metadata))
            )
            self.env['sqlite'].commit()

    def search_vector(self, db_type, query_vector, n_results=10):
        if db_type == 'annoy':
            return self.env['annoy'].get_nns_by_vector(query_vector, n_results)
        elif db_type == 'hnsw':
            return self.env['hnsw'].knn_query(np.array([query_vector]), k=n_results)
        elif db_type == 'lmdb':
            results = []
            with self.env['lmdb'].begin() as txn:
                cursor = txn.cursor()
                for key, value in cursor:
                    vector_data = json.loads(value.decode())
                    distance = np.linalg.norm(np.array(query_vector) - np.array(vector_data['vector']))
                    results.append((key.decode(), distance))
            return sorted(results, key=lambda x: x[1])[:n_results]
        elif db_type == 'sqlite':
            cursor = self.env['sqlite'].cursor()
            results = []
            for row in cursor.execute('SELECT id, vector FROM vectors'):
                vector = json.loads(row[1])
                distance = np.linalg.norm(np.array(query_vector) - np.array(vector))
                results.append((row[0], distance))
            return sorted(results, key=lambda x: x[1])[:n_results]

    def get_status(self, db_type):
        if db_type == 'annoy':
            return {
                'type': 'Annoy',
                'dimension': self.tunings['annoy']['dimension'],
                'n_trees': self.tunings['annoy']['n_trees'],
                'status': 'Connected'
            }
        elif db_type == 'hnsw':
            return {
                'type': 'HNSW',
                'dimension': self.tunings['hnsw']['dimension'],
                'm': self.tunings['hnsw']['m'],
                'status': 'Connected'
            }
        elif db_type == 'lmdb':
            return {
                'type': 'LMDB',
                'map_size': self.tunings['lmdb']['map_size'],
                'status': 'Connected'
            }
        elif db_type == 'sqlite':
            cursor = self.env['sqlite'].cursor()
            cursor.execute('SELECT COUNT(*) FROM vectors')
            count = cursor.fetchone()[0]
            return {
                'type': 'SQLite',
                'journal_mode': self.tunings['sqlite3']['journal_mode'],
                'vector_count': count,
                'status': 'Connected'
            }
        return {'status': 'Not Connected'}
