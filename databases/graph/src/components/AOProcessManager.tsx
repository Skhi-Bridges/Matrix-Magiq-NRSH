import React from 'react';
import { useAOProcesses } from '../hooks/useAOProcesses';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from './ui/card';
import { Badge } from './ui/badge';
import { Button } from './ui/button';
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from './ui/tabs';

export const AOProcessManager: React.FC = () => {
  const {
    processes,
    isLoading,
    error,
    startProcess,
    stopProcess,
    dbProcessTypes
  } = useAOProcesses();

  if (isLoading) return <div>Loading processes...</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-2xl font-bold mb-4">AO Process Manager</h1>
      
      <Tabs defaultValue="vector">
        <TabsList>
          <TabsTrigger value="vector">Vector Stores</TabsTrigger>
          <TabsTrigger value="graph">Graph Stores</TabsTrigger>
          <TabsTrigger value="quantum">Quantum Stores</TabsTrigger>
          <TabsTrigger value="classical">Classical Stores</TabsTrigger>
        </TabsList>

        <TabsContent value="vector">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {dbProcessTypes.VECTOR_STORES.map((db) => (
              <ProcessCard
                key={db.name}
                db={db}
                status={processes.find(p => p.name === db.name)?.status || 'idle'}
                onStart={() => startProcess(db.name, {})}
                onStop={() => {
                  const process = processes.find(p => p.name === db.name);
                  if (process) stopProcess(process.id);
                }}
              />
            ))}
          </div>
        </TabsContent>

        <TabsContent value="graph">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {dbProcessTypes.GRAPH_STORES.map((db) => (
              <ProcessCard
                key={db.name}
                db={db}
                status={processes.find(p => p.name === db.name)?.status || 'idle'}
                onStart={() => startProcess(db.name, {})}
                onStop={() => {
                  const process = processes.find(p => p.name === db.name);
                  if (process) stopProcess(process.id);
                }}
              />
            ))}
          </div>
        </TabsContent>

        <TabsContent value="quantum">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {dbProcessTypes.QUANTUM_STORES.map((db) => (
              <ProcessCard
                key={db.name}
                db={db}
                status={processes.find(p => p.name === db.name)?.status || 'idle'}
                onStart={() => startProcess(db.name, {})}
                onStop={() => {
                  const process = processes.find(p => p.name === db.name);
                  if (process) stopProcess(process.id);
                }}
              />
            ))}
          </div>
        </TabsContent>

        <TabsContent value="classical">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {dbProcessTypes.CLASSICAL_STORES.map((db) => (
              <ProcessCard
                key={db.name}
                db={db}
                status={processes.find(p => p.name === db.name)?.status || 'idle'}
                onStart={() => startProcess(db.name, {})}
                onStop={() => {
                  const process = processes.find(p => p.name === db.name);
                  if (process) stopProcess(process.id);
                }}
              />
            ))}
          </div>
        </TabsContent>
      </Tabs>
    </div>
  );
};

interface ProcessCardProps {
  db: {
    name: string;
    type: string;
    description: string;
  };
  status: 'active' | 'idle' | 'error';
  onStart: () => void;
  onStop: () => void;
}

const ProcessCard: React.FC<ProcessCardProps> = ({ db, status, onStart, onStop }) => {
  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center justify-between">
          {db.name}
          <Badge
            variant={status === 'active' ? 'default' : status === 'error' ? 'destructive' : 'secondary'}
          >
            {status}
          </Badge>
        </CardTitle>
        <CardDescription>{db.description}</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="flex justify-end space-x-2">
          {status === 'idle' ? (
            <Button onClick={onStart}>Start</Button>
          ) : (
            <Button variant="destructive" onClick={onStop}>Stop</Button>
          )}
        </div>
      </CardContent>
    </Card>
  );
};
