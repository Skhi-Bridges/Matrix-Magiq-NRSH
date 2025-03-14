from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
import asyncio
from concurrent.futures import ProcessPoolExecutor
from typing import List, Dict, Any
import uvicorn
from pydantic import BaseModel
import qiskit
from pathlib import Path
import numpy as np
from app import process_vector_operation, calculate_distance

app = FastAPI()

# Add CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # In production, replace with specific origins
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Database process pool
db_pool = ProcessPoolExecutor(max_workers=3)  # For Supabase, GraphQL, and other DB processes

class VectorOperation(BaseModel):
    operation: str
    vectors: List[List[float]]
    store_type: str = "annoy"

class DistanceMetric(BaseModel):
    metric_name: str
    vector1: List[float]
    vector2: List[float]

@app.post("/api/vector/process")
async def handle_vector_operation(op: VectorOperation):
    try:
        # Run in process pool to not block the main thread
        loop = asyncio.get_event_loop()
        result = await loop.run_in_executor(
            db_pool,
            process_vector_operation,
            op.operation,
            op.vectors,
            op.store_type
        )
        return {"status": "success", "result": result}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/api/distance/calculate")
async def handle_distance_calculation(metric: DistanceMetric):
    try:
        loop = asyncio.get_event_loop()
        result = await loop.run_in_executor(
            db_pool,
            calculate_distance,
            metric.metric_name,
            metric.vector1,
            metric.vector2
        )
        return {"status": "success", "result": result}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# Database health check endpoints
@app.get("/api/health/supabase")
async def check_supabase():
    # Add Supabase health check
    return {"status": "healthy"}

@app.get("/api/health/graphql")
async def check_graphql():
    # Add GraphQL health check
    return {"status": "healthy"}

if __name__ == "__main__":
    uvicorn.run("api:app", host="0.0.0.0", port=8000, reload=True)
