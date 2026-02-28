import express from 'express';
import cors from 'cors';

const app = express();
const port = Number(process.env.PORT || 3000);

const machineId = Number(process.env.MACHINE_ID || 1);
const machineName = process.env.MACHINE_NAME || `CNC-${String(machineId).padStart(2, '0')}`;

const corsOrigin = process.env.CORS_ORIGIN || '*';
app.use(cors({ origin: corsOrigin }));

function buildPayload() {
  const ts = Date.now();
  const temperature = 55 + Math.random() * 40;
  const vibration = 10 + Math.random() * 70;

  let status = 'running';
  const alerts = [];

  if (temperature > 85) {
    status = 'fault';
    alerts.push({
      level: 'critical',
      message: `Machine ${machineId} overheat: ${temperature.toFixed(2)}°C`,
      timestamp: ts,
    });
  } else if (vibration > 60) {
    status = 'warning';
    alerts.push({
      level: 'warning',
      message: `Machine ${machineId} vibration high: ${vibration.toFixed(2)}`,
      timestamp: ts,
    });
  }

  const tempPenalty = Math.min(Math.max(temperature - 60, 0) * 0.9, 45);
  const vibPenalty = Math.min(vibration * 0.7, 45);
  const oee = Math.max(0, Math.min(100, 100 - tempPenalty - vibPenalty));

  return {
    machine: {
      id: machineId,
      name: machineName,
      status,
      oee,
      temperature,
      vibration,
      last_updated: ts,
    },
    alerts,
  };
}

app.get('/health', (_req, res) => {
  res.json({ ok: true, service: 'smart-factory-render-backend' });
});

app.get('/api/live', (_req, res) => {
  res.json(buildPayload());
});

app.listen(port, () => {
  console.log(`smart-factory-render-backend running on :${port}`);
});
