from lightning.app import LightningWork
import sqlite3
import numpy as np
from pathlib import Path
import json
import logging
from typing import Dict, Any, List, Optional

logger = logging.getLogger(__name__)

class DatabaseWorker(LightningWork):
    def __init__(self, db_path: str):
        super().__init__(parallel=True)
        self.db_path = Path(db_path)
        self.db_path.parent.mkdir(parents=True, exist_ok=True)
        
    def setup(self):
        """Initialize the database"""
        self.conn = sqlite3.connect(self.db_path)
        self.create_tables()
        
    def create_tables(self):
        """Create necessary database tables"""
        cursor = self.conn.cursor()
        
        # Content table
        cursor.execute('''
        CREATE TABLE IF NOT EXISTS content (
            content_id TEXT PRIMARY KEY,
            content_type TEXT NOT NULL,
            file_path TEXT NOT NULL,
            embedding_path TEXT,
            metadata TEXT
        )
        ''')
        
        # Embeddings table
        cursor.execute('''
        CREATE TABLE IF NOT EXISTS embeddings (
            content_id TEXT PRIMARY KEY,
            embedding BLOB NOT NULL,
            FOREIGN KEY (content_id) REFERENCES content(content_id)
        )
        ''')
        
        self.conn.commit()
        
    def store_content(self, content_id: str, result: Dict) -> None:
        """Store content processing results"""
        try:
            cursor = self.conn.cursor()
            
            # Store content info
            cursor.execute('''
            INSERT OR REPLACE INTO content 
            (content_id, content_type, file_path, embedding_path, metadata)
            VALUES (?, ?, ?, ?, ?)
            ''', (
                content_id,
                result['content_type'],
                result['file_path'],
                result.get('embedding_path'),
                json.dumps(result.get('metadata', {}))
            ))
            
            # Store embedding if available
            if 'embedding' in result:
                embedding_bytes = np.array(result['embedding']).tobytes()
                cursor.execute('''
                INSERT OR REPLACE INTO embeddings (content_id, embedding)
                VALUES (?, ?)
                ''', (content_id, embedding_bytes))
            
            self.conn.commit()
            
        except Exception as e:
            logger.error(f"Error storing content: {e}")
            self.conn.rollback()
            raise
            
    def search_similar(self, query_embedding: np.ndarray, limit: int = 10) -> List[Dict]:
        """Search for similar content using embeddings"""
        try:
            cursor = self.conn.cursor()
            results = []
            
            # Fetch all embeddings
            cursor.execute('SELECT content_id, embedding FROM embeddings')
            for content_id, embedding_bytes in cursor.fetchall():
                embedding = np.frombuffer(embedding_bytes).reshape(-1)
                similarity = np.dot(query_embedding, embedding) / (
                    np.linalg.norm(query_embedding) * np.linalg.norm(embedding)
                )
                
                # Fetch content info
                cursor.execute('''
                SELECT content_type, file_path, metadata 
                FROM content WHERE content_id = ?
                ''', (content_id,))
                content_type, file_path, metadata = cursor.fetchone()
                
                results.append({
                    'content_id': content_id,
                    'similarity': float(similarity),
                    'content_type': content_type,
                    'file_path': file_path,
                    'metadata': json.loads(metadata)
                })
            
            # Sort by similarity and return top results
            results.sort(key=lambda x: x['similarity'], reverse=True)
            return results[:limit]
            
        except Exception as e:
            logger.error(f"Error searching similar content: {e}")
            raise
            
    def get_content(self, content_id: str) -> Optional[Dict]:
        """Retrieve content by ID"""
        try:
            cursor = self.conn.cursor()
            
            cursor.execute('''
            SELECT c.content_type, c.file_path, c.metadata, e.embedding
            FROM content c
            LEFT JOIN embeddings e ON c.content_id = e.content_id
            WHERE c.content_id = ?
            ''', (content_id,))
            
            result = cursor.fetchone()
            if not result:
                return None
                
            content_type, file_path, metadata, embedding_bytes = result
            embedding = np.frombuffer(embedding_bytes).reshape(-1) if embedding_bytes else None
            
            return {
                'content_id': content_id,
                'content_type': content_type,
                'file_path': file_path,
                'metadata': json.loads(metadata),
                'embedding': embedding.tolist() if embedding is not None else None
            }
            
        except Exception as e:
            logger.error(f"Error retrieving content: {e}")
            raise
            
    def run(self, action: str, **kwargs) -> Any:
        """Main entry point for database operations"""
        self.setup()
        
        actions = {
            'store': self.store_content,
            'search': self.search_similar,
            'get': self.get_content
        }
        
        handler = actions.get(action)
        if not handler:
            raise ValueError(f"Unsupported action: {action}")
            
        return handler(**kwargs)
