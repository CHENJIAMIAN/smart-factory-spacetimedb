import React from 'react';
import { createRoot } from 'react-dom/client';
import './style.css';

type Machine = {
  id: number;
  name: string;
  status: 'running' | 'idle' | 'fault' | 'warning';
  oee: number;
  lastUpdated: number;
};

const mockMachines: Machine[] = [
  { id: 1, name: 'CNC-01', status: 'running', oee: 87.2, lastUpdated: Date.now() },
  { id: 2, name: 'PRESS-02', status: 'idle', oee: 76.5, lastUpdated: Date.now() },
  { id: 3, name: 'WELD-03', status: 'warning', oee: 69.1, lastUpdated: Date.now() },
];

function App() {
  return (
    <div className="page">
      <h1>Smart Factory Digital Twin</h1>
      <p className="sub">MVP Dashboard (next: bind SpacetimeDB subscriptions)</p>

      <div className="grid">
        {mockMachines.map((m) => (
          <div key={m.id} className="card">
            <h3>{m.name}</h3>
            <p>Status: <b>{m.status}</b></p>
            <p>OEE: <b>{m.oee.toFixed(1)}%</b></p>
            <p className="muted">{new Date(m.lastUpdated).toLocaleTimeString()}</p>
          </div>
        ))}
      </div>
    </div>
  );
}

createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
