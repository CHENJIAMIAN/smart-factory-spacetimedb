import React, { useEffect, useMemo, useState } from 'react';
import { createRoot } from 'react-dom/client';
import './style.css';

type Machine = {
  id: number;
  name: string;
  status: 'running' | 'idle' | 'fault' | 'warning';
  oee: number;
  temperature: number;
  vibration: number;
  last_updated: number;
};

type Alert = {
  level: 'warning' | 'critical' | string;
  message: string;
  timestamp: number;
};

type LivePayload = {
  machine: Machine;
  alerts: Alert[];
};

const fallback: LivePayload = {
  machine: {
    id: 1,
    name: 'CNC-01',
    status: 'idle',
    oee: 0,
    temperature: 0,
    vibration: 0,
    last_updated: Date.now(),
  },
  alerts: [],
};

const API_BASE = (import.meta.env.VITE_API_BASE as string | undefined)?.replace(/\/$/, '') || '';

function App() {
  const [data, setData] = useState<LivePayload>(fallback);
  const [online, setOnline] = useState(false);

  useEffect(() => {
    let alive = true;

    const load = async () => {
      try {
        const url = API_BASE ? `${API_BASE}/api/live?t=${Date.now()}` : `/live.json?t=${Date.now()}`;
        const res = await fetch(url, { cache: 'no-store' });
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        const json = (await res.json()) as LivePayload;
        if (alive) {
          setData(json);
          setOnline(true);
        }
      } catch {
        if (alive) setOnline(false);
      }
    };

    load();
    const timer = setInterval(load, 1000);
    return () => {
      alive = false;
      clearInterval(timer);
    };
  }, []);

  const statusClass = useMemo(() => `badge ${data.machine.status}`, [data.machine.status]);

  return (
    <div className="page">
      <h1>Smart Factory Digital Twin</h1>
      <p className="sub">
        Realtime MVP Dashboard · <span className={online ? 'ok' : 'down'}>{online ? 'LIVE' : 'OFFLINE'}</span>
        {API_BASE ? ` · backend: ${API_BASE}` : ' · backend: local live.json'}
      </p>

      <div className="grid">
        <div className="card">
          <h3>{data.machine.name}</h3>
          <p>Status: <span className={statusClass}>{data.machine.status}</span></p>
          <p>OEE: <b>{data.machine.oee.toFixed(1)}%</b></p>
          <p>Temp: <b>{data.machine.temperature.toFixed(2)}°C</b></p>
          <p>Vibration: <b>{data.machine.vibration.toFixed(2)}</b></p>
          <p className="muted">{new Date(data.machine.last_updated).toLocaleTimeString()}</p>
        </div>

        <div className="card">
          <h3>Alerts</h3>
          {data.alerts.length === 0 ? (
            <p className="muted">No alerts</p>
          ) : (
            <ul className="alerts">
              {data.alerts.map((a, idx) => (
                <li key={`${a.timestamp}-${idx}`}>
                  <span className={`level ${a.level}`}>{a.level}</span>
                  <span>{a.message}</span>
                </li>
              ))}
            </ul>
          )}
        </div>
      </div>
    </div>
  );
}

createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
