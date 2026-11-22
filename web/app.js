// DarwinCode Live Telemetry & Advanced Multi-Challenge Controller
let isRunning = true;
let currentChallenge = 'parabola';
const fitnessHistory = [];
const maxHistoryPoints = 120;
let lastKnownFameCount = 0;

// Elementos del DOM - Telemetría
const genVal = document.getElementById('genVal');
const fitnessVal = document.getElementById('fitnessVal');
const avgFitnessVal = document.getElementById('avgFitnessVal');
const nodesVal = document.getElementById('nodesVal');
const layersVal = document.getElementById('layersVal');
const formulaVal = document.getElementById('formulaVal');
const toggleBtn = document.getElementById('toggleBtn');
const logStream = document.getElementById('logStream');
const activeChallengeTag = document.getElementById('activeChallengeTag');
const victoryBanner = document.getElementById('victoryBanner');
const hallOfFameList = document.getElementById('hallOfFameList');
const fameCountVal = document.getElementById('fameCountVal');

// Elementos del DOM - Constructor 3D
const renderedBlocksCount = document.getElementById('renderedBlocksCount');
const renderedLayersCount = document.getElementById('renderedLayersCount');
const modeAlgoBtn = document.getElementById('modeAlgoBtn');
const modeVoxelBtn = document.getElementById('modeVoxelBtn');
const autoRotateBtn = document.getElementById('autoRotateBtn');
const resetCamBtn = document.getElementById('resetCamBtn');

// Inicializar Renderizador 3D Universal
const renderer3D = new Darwin3DRenderer('viewport3D');

// Controladores de botones del Constructor 3D
if (modeAlgoBtn && modeVoxelBtn) {
    modeAlgoBtn.addEventListener('click', () => {
        modeAlgoBtn.classList.add('active');
        modeVoxelBtn.classList.remove('active');
        renderer3D.setMode('algorithm');
        addLog('[VISTA 3D] Modo cambiado a: Torre Algorítmica (Computación DAG)');
    });

    modeVoxelBtn.addEventListener('click', () => {
        modeVoxelBtn.classList.add('active');
        modeAlgoBtn.classList.remove('active');
        renderer3D.setMode('voxel');
        addLog('[VISTA 3D] Modo cambiado a: Arquitectura Voxel Estructural');
    });
}

if (autoRotateBtn) {
    autoRotateBtn.addEventListener('click', () => {
        const rotating = renderer3D.toggleAutoRotate();
        autoRotateBtn.classList.toggle('active', rotating);
        autoRotateBtn.style.background = rotating ? '#0e3b43' : '';
        autoRotateBtn.style.borderColor = rotating ? '#06b6d4' : '';
    });
}

if (resetCamBtn) {
    resetCamBtn.addEventListener('click', () => {
        renderer3D.resetCamera();
    });
}

// Selector de Desafíos en Vivo
document.querySelectorAll('.btn-challenge').forEach(btn => {
    btn.addEventListener('click', async () => {
        const challengeId = btn.dataset.id;
        document.querySelectorAll('.btn-challenge').forEach(b => b.classList.remove('active'));
        btn.classList.add('active');

        try {
            await fetch('/api/challenge', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ challenge_id: challengeId }),
            });
            fitnessHistory.length = 0;
            currentChallenge = challengeId;
            addLog(`[DESAFÍO] Cambiando objetivo evolutivo a: ${challengeId}`);
        } catch (err) {
            console.error('Error cambiando desafío:', err);
        }
    });
});

// Control de Velocidad
document.querySelectorAll('.btn-speed').forEach(btn => {
    btn.addEventListener('click', async () => {
        const delayMs = parseInt(btn.dataset.speed, 10);
        document.querySelectorAll('.btn-speed').forEach(b => b.classList.remove('active'));
        btn.classList.add('active');

        try {
            await fetch('/api/speed', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ delay_ms: delayMs }),
            });
            addLog(`[VELOCIDAD] Regulador ajustado a delay de ${delayMs}ms`);
        } catch (err) {
            console.error('Error ajustando velocidad:', err);
        }
    });
});

// Canvas 2D secundarios
const chartCanvas = document.getElementById('chartCanvas');
const chartCtx = chartCanvas.getContext('2d');

const dagCanvas = document.getElementById('dagCanvas');
const dagCtx = dagCanvas.getContext('2d');

function resizeCanvas(canvas) {
    const rect = canvas.getBoundingClientRect();
    const dpr = window.devicePixelRatio || 1;
    canvas.width = rect.width * dpr;
    canvas.height = rect.height * dpr;
    const ctx = canvas.getContext('2d');
    ctx.scale(dpr, dpr);
    return { width: rect.width, height: rect.height };
}

window.addEventListener('resize', () => {
    drawChart();
    if (lastGraph) drawDAG(lastGraph);
});

let lastGraph = null;

// Conexión SSE en tiempo real
const eventSource = new EventSource('/events');

eventSource.onmessage = function (event) {
    try {
        const data = JSON.parse(event.data);
        updateTelemetry(data);
    } catch (err) {
        console.error('Error parseando telemetría SSE:', err);
    }
};

eventSource.onerror = function () {
    addLog('[CONEXIÓN] Flujo SSE reconectando...', false);
};

function updateTelemetry(data) {
    const { stats, best_graph, is_running, spatial_layout, voxels, formula, current_challenge_name, hall_of_fame } = data;
    isRunning = is_running;
    toggleBtn.textContent = isRunning ? 'PAUSAR' : 'REANUDAR';
    toggleBtn.style.background = isRunning ? '#06b6d4' : '#10b981';

    genVal.textContent = stats.generation.toLocaleString();
    fitnessVal.textContent = stats.best_fitness.toFixed(4);
    avgFitnessVal.textContent = stats.avg_fitness.toFixed(4);
    nodesVal.textContent = stats.best_nodes_count;

    if (activeChallengeTag && current_challenge_name) {
        activeChallengeTag.textContent = current_challenge_name;
    }

    if (formula) {
        formulaVal.textContent = formula;
    }

    // Calcular capas de pisos
    let maxLayers = 1;
    if (spatial_layout && spatial_layout.length > 0) {
        maxLayers = spatial_layout.reduce((m, n) => Math.max(m, (n.layer || 0) + 1), 1);
    }
    if (layersVal) layersVal.textContent = maxLayers;
    if (renderedLayersCount) renderedLayersCount.textContent = maxLayers;

    // Actualizar Renderizador 3D Universal bloque a bloque
    if (spatial_layout && voxels) {
        renderer3D.updateData(spatial_layout, voxels);
        if (renderedBlocksCount) {
            renderedBlocksCount.textContent = renderer3D.mode === 'algorithm' ? spatial_layout.length : voxels.length;
        }
    }

    fitnessHistory.push(stats.best_fitness);
    if (fitnessHistory.length > maxHistoryPoints) {
        fitnessHistory.shift();
    }

    drawChart();

    if (best_graph) {
        lastGraph = best_graph;
        drawDAG(best_graph);
    }

    // Actualizar Hall of Fame si hay nuevos trofeos
    if (hall_of_fame) {
        updateHallOfFame(hall_of_fame);
    }

    if (stats.generation % 50 === 0 || stats.generation === 1) {
        addLog(`[Gen ${stats.generation}] Aptitud: ${stats.best_fitness.toFixed(4)} | Nodos: ${stats.best_nodes_count} | Pisos: ${maxLayers}`, stats.best_fitness >= 1.0);
    }
}

function updateHallOfFame(fameList) {
    if (fameCountVal) fameCountVal.textContent = fameList.length;

    // Si hay un descubrimiento nuevo, mostrar banner de victoria
    if (fameList.length > lastKnownFameCount) {
        lastKnownFameCount = fameList.length;
        const latest = fameList[fameList.length - 1];
        showVictoryBanner(latest);
    }

    if (!hallOfFameList) return;

    if (fameList.length === 0) {
        hallOfFameList.innerHTML = '<div class="fame-empty-hint">Evolucionando hacia el primer descubrimiento al 100%...</div>';
        return;
    }

    hallOfFameList.innerHTML = '';
    fameList.forEach(rec => {
        const card = document.createElement('div');
        card.className = 'fame-card-item';
        card.innerHTML = `
            <div class="fame-item-top">
                <span class="fame-badge">🏆 CONQUISTADO</span>
                <span class="fame-gen">Gen ${rec.generation.toLocaleString()}</span>
            </div>
            <div class="fame-item-name">${rec.challenge_name}</div>
            <div class="fame-item-formula">${rec.formula}</div>
            <div class="fame-item-stats">
                <span>Precisión: <strong>${rec.accuracy.toFixed(1)}%</strong></span>
                <span>Complejidad: <strong>${rec.node_count} bloques</strong></span>
            </div>
        `;
        hallOfFameList.appendChild(card);
    });
}

function showVictoryBanner(record) {
    if (!victoryBanner) return;
    const victoryTitle = document.getElementById('victoryTitle');
    const victorySub = document.getElementById('victorySub');
    if (victoryTitle) victoryTitle.textContent = `🏆 ¡LEY CONQUISTADA: ${record.challenge_name}!`;
    if (victorySub) victorySub.textContent = `Descubrimiento exacto: ${record.formula} (Gen ${record.generation.toLocaleString()})`;

    victoryBanner.classList.remove('hidden');
    victoryBanner.classList.add('pulse-victory');
    setTimeout(() => {
        victoryBanner.classList.remove('pulse-victory');
    }, 8000);
}

function addLog(text, highlight = false) {
    const entry = document.createElement('div');
    entry.className = `log-entry ${highlight ? 'highlight-log' : ''}`;
    entry.textContent = text;
    logStream.appendChild(entry);
    if (logStream.children.length > 40) {
        logStream.removeChild(logStream.children[0]);
    }
    logStream.scrollTop = logStream.scrollHeight;
}

// Botón de control de pausa
toggleBtn.addEventListener('click', async () => {
    try {
        const res = await fetch('/api/toggle', { method: 'POST' });
        const data = await res.json();
        isRunning = data.running;
        toggleBtn.textContent = isRunning ? 'PAUSAR' : 'REANUDAR';
        toggleBtn.style.background = isRunning ? '#06b6d4' : '#10b981';
        addLog(`[SISTEMA] Motor ${isRunning ? 'reanudado' : 'pausado'} por el usuario.`);
    } catch (err) {
        console.error('Error enviando comando toggle:', err);
    }
});

// Renderizador del Gráfico de Aptitud en Canvas
function drawChart() {
    const { width, height } = resizeCanvas(chartCanvas);
    chartCtx.clearRect(0, 0, width, height);

    if (fitnessHistory.length < 2) return;

    chartCtx.strokeStyle = '#131b26';
    chartCtx.lineWidth = 1;
    for (let y = 0; y < height; y += 40) {
        chartCtx.beginPath();
        chartCtx.moveTo(0, y);
        chartCtx.lineTo(width, y);
        chartCtx.stroke();
    }

    const min = 0;
    const max = 1.05;

    chartCtx.beginPath();
    chartCtx.strokeStyle = '#10b981';
    chartCtx.lineWidth = 2.5;
    chartCtx.shadowColor = '#10b981';
    chartCtx.shadowBlur = 8;

    const stepX = width / (maxHistoryPoints - 1);
    fitnessHistory.forEach((val, i) => {
        const x = i * stepX;
        const normalizedY = (val - min) / (max - min);
        const y = height - (normalizedY * (height - 20) + 10);
        if (i === 0) chartCtx.moveTo(x, y);
        else chartCtx.lineTo(x, y);
    });
    chartCtx.stroke();
    chartCtx.shadowBlur = 0;
}

// Renderizador del Grafo 2D (DAG)
function drawDAG(graph) {
    const { width, height } = resizeCanvas(dagCanvas);
    dagCtx.clearRect(0, 0, width, height);

    if (!graph.nodes || graph.nodes.length === 0) return;

    const nodes = graph.nodes;
    const nodeCount = nodes.length;

    const nodePositions = new Map();
    const centerX = width / 2;
    const centerY = height / 2;
    const radius = Math.min(width, height) * 0.38;

    nodes.forEach((node, i) => {
        const angle = (i / nodeCount) * Math.PI * 2 - Math.PI / 2;
        const x = centerX + Math.cos(angle) * radius;
        const y = centerY + Math.sin(angle) * radius;
        nodePositions.set(node.id, { x, y, node });
    });

    dagCtx.lineWidth = 1.5;
    dagCtx.strokeStyle = 'rgba(6, 182, 212, 0.4)';
    nodes.forEach(node => {
        if (node.node_type && node.node_type.Operator) {
            const opData = node.node_type.Operator;
            const targetPos = nodePositions.get(node.id);
            if (targetPos && opData.inputs) {
                opData.inputs.forEach(sourceId => {
                    const srcPos = nodePositions.get(sourceId);
                    if (srcPos) {
                        dagCtx.beginPath();
                        dagCtx.moveTo(srcPos.x, srcPos.y);
                        dagCtx.lineTo(targetPos.x, targetPos.y);
                        dagCtx.stroke();
                    }
                });
            }
        }
    });

    nodes.forEach(node => {
        const pos = nodePositions.get(node.id);
        if (!pos) return;

        const isOutput = graph.output_node === node.id;
        const isInput = node.node_type && node.node_type.Input;
        const isConst = node.node_type && node.node_type.Constant !== undefined;

        let fillColor = '#06b6d4';
        let label = 'OP';

        if (isInput) {
            fillColor = '#a855f7';
            label = 'IN';
        } else if (isConst) {
            fillColor = '#64748b';
            label = 'C';
        }

        if (isOutput) {
            fillColor = '#10b981';
            label = 'OUT';
        }

        dagCtx.beginPath();
        dagCtx.arc(pos.x, pos.y, 14, 0, Math.PI * 2);
        dagCtx.fillStyle = '#07090e';
        dagCtx.fill();
        dagCtx.lineWidth = isOutput ? 3 : 2;
        dagCtx.strokeStyle = fillColor;
        dagCtx.stroke();

        dagCtx.fillStyle = '#ffffff';
        dagCtx.font = '10px monospace';
        dagCtx.textAlign = 'center';
        dagCtx.textBaseline = 'middle';
        dagCtx.fillText(label, pos.x, pos.y);
    });
}
