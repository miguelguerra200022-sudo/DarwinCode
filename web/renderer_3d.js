// DarwinCode 3D Universal Block & Voxel Engine
// Supports both WebGL Three.js (Hardware-Accelerated) and 2.5D Isometric Fallback

class Darwin3DRenderer {
    constructor(canvasContainerId) {
        this.container = document.getElementById(canvasContainerId);
        this.mode = 'algorithm'; // 'algorithm' | 'voxel'
        this.currentLayout = [];
        this.currentVoxels = [];
        this.hasThree = typeof THREE !== 'undefined';

        // Orbit / Camera state
        this.rotX = 0.38;
        this.rotY = -0.65;
        this.zoom = 30;
        this.targetRotX = 0.38;
        this.targetRotY = -0.65;
        this.targetZoom = 30;
        this.isDragging = false;
        this.lastMouseX = 0;
        this.lastMouseY = 0;
        this.pulseTime = 0;
        this.autoRotate = false;

        // Texture Cache & Active Mesh Pool for zero-garbage 60 FPS animation
        this.textureCache = new Map();
        this.activeBlockMeshes = new Map(); // key -> { mesh, targetPos, targetScale, isDying }

        if (this.hasThree) {
            this.initThreeJS();
        } else {
            this.initFallbackCanvas();
        }

        this.setupInteractions();
        this.animate = this.animate.bind(this);
        requestAnimationFrame(this.animate);
    }

    setMode(newMode) {
        if (this.mode === newMode) return;
        this.mode = newMode;
        // Limpiar bloques activos suavemente
        this.clearAllMeshes();
        if (this.hasThree) {
            this.updateThreeData(this.currentLayout, this.currentVoxels);
        }
    }

    toggleAutoRotate() {
        this.autoRotate = !this.autoRotate;
        return this.autoRotate;
    }

    resetCamera() {
        this.targetRotX = 0.38;
        this.targetRotY = -0.65;
        this.targetZoom = 30;
    }

    initThreeJS() {
        this.scene = new THREE.Scene();
        this.scene.background = new THREE.Color(0x020408);

        const width = this.container.clientWidth || 600;
        const height = this.container.clientHeight || 400;
        const aspect = width / height;

        this.camera = new THREE.PerspectiveCamera(45, aspect, 0.1, 1000);
        this.camera.position.set(0, 16, 32);
        this.camera.lookAt(0, 4, 0);

        this.webglRenderer = new THREE.WebGLRenderer({ antialias: true, alpha: true, powerPreference: "high-performance" });
        this.webglRenderer.setSize(width, height);
        this.webglRenderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
        this.container.appendChild(this.webglRenderer.domElement);

        // Iluminación ambiental y direccional
        const ambientLight = new THREE.AmbientLight(0xffffff, 0.85);
        this.scene.add(ambientLight);

        const dirLight1 = new THREE.DirectionalLight(0x06b6d4, 1.4);
        dirLight1.position.set(20, 35, 25);
        this.scene.add(dirLight1);

        const dirLight2 = new THREE.DirectionalLight(0xa855f7, 0.8);
        dirLight2.position.set(-20, -10, -20);
        this.scene.add(dirLight2);

        // Cuadrícula cyberpunk en el suelo
        this.gridHelper = new THREE.GridHelper(36, 36, 0x06b6d4, 0x131b26);
        this.gridHelper.position.y = -0.01;
        this.scene.add(this.gridHelper);

        // Grupos de escena
        this.blocksGroup = new THREE.Group();
        this.beamsGroup = new THREE.Group();
        this.scene.add(this.blocksGroup);
        this.scene.add(this.beamsGroup);

        // Geometrías compartidas
        this.algoBoxGeom = new THREE.BoxGeometry(1.6, 1.1, 1.6);
        this.algoEdgesGeom = new THREE.EdgesGeometry(this.algoBoxGeom);
        this.voxelBoxGeom = new THREE.BoxGeometry(1.0, 1.0, 1.0);
        this.voxelEdgesGeom = new THREE.EdgesGeometry(this.voxelBoxGeom);

        // Listener de redimensión de ventana
        window.addEventListener('resize', () => {
            if (!this.container || !this.webglRenderer) return;
            const w = this.container.clientWidth;
            const h = this.container.clientHeight;
            if (w === 0 || h === 0) return;
            this.camera.aspect = w / h;
            this.camera.updateProjectionMatrix();
            this.webglRenderer.setSize(w, h);
        });
    }

    initFallbackCanvas() {
        this.canvas = document.createElement('canvas');
        this.canvas.style.width = '100%';
        this.canvas.style.height = '100%';
        this.canvas.style.display = 'block';
        this.container.appendChild(this.canvas);
        this.ctx = this.canvas.getContext('2d');
    }

    // Generador y caché de texturas procedurales para los bloques 3D
    getOrCreateTexture(label, colorHex) {
        const key = `${label}_${colorHex}`;
        if (this.textureCache.has(key)) {
            return this.textureCache.get(key);
        }

        const cvs = document.createElement('canvas');
        cvs.width = 128;
        cvs.height = 128;
        const c = cvs.getContext('2d');

        // Fondo oscuro de bloque
        c.fillStyle = '#080d15';
        c.fillRect(0, 0, 128, 128);

        // Marco cibernético iluminado
        c.strokeStyle = colorHex;
        c.lineWidth = 6;
        c.strokeRect(6, 6, 116, 116);

        // Círculo central o cruz estética
        c.strokeStyle = 'rgba(255, 255, 255, 0.15)';
        c.lineWidth = 1;
        c.beginPath();
        c.arc(64, 64, 44, 0, Math.PI * 2);
        c.stroke();

        // Texto / Símbolo del operador o constante
        c.fillStyle = colorHex;
        c.textAlign = 'center';
        c.textBaseline = 'middle';
        
        let fontSize = 28;
        if (label.length > 5) fontSize = 20;
        if (label.length > 8) fontSize = 16;
        c.font = `bold ${fontSize}px monospace`;
        c.fillText(label, 64, 64);

        const tex = new THREE.CanvasTexture(cvs);
        this.textureCache.set(key, tex);
        return tex;
    }

    clearAllMeshes() {
        for (const [key, entry] of this.activeBlockMeshes.entries()) {
            this.blocksGroup.remove(entry.mesh);
        }
        this.activeBlockMeshes.clear();
        while (this.beamsGroup.children.length > 0) {
            this.beamsGroup.remove(this.beamsGroup.children[0]);
        }
    }

    updateData(spatialLayout, voxels) {
        this.currentLayout = spatialLayout || [];
        this.currentVoxels = voxels || [];

        if (this.hasThree) {
            this.updateThreeData(this.currentLayout, this.currentVoxels);
        }
    }

    updateThreeData(spatialLayout, voxels) {
        const seenKeys = new Set();

        if (this.mode === 'algorithm') {
            const nodeMap = new Map();

            // Sincronizar bloques de la Torre de ADN / Algoritmo
            spatialLayout.forEach(node => {
                const key = `node_${node.id}`;
                seenKeys.add(key);

                let colorHex = '#06b6d4'; // Cian (Operador)
                let colorInt = 0x06b6d4;
                if (node.color_tag === 'input') {
                    colorHex = '#a855f7';
                    colorInt = 0xa855f7;
                } else if (node.color_tag === 'constant') {
                    colorHex = '#64748b';
                    colorInt = 0x64748b;
                } else if (node.color_tag === 'output') {
                    colorHex = '#10b981';
                    colorInt = 0x10b981;
                }

                const targetX = node.x;
                const targetY = node.y + 0.55;
                const targetZ = node.z;

                if (this.activeBlockMeshes.has(key)) {
                    // Bloque existente: actualizar posición objetivo
                    const entry = this.activeBlockMeshes.get(key);
                    entry.targetPos.set(targetX, targetY, targetZ);
                    entry.targetScale.set(1.0, 1.0, 1.0);
                    entry.isDying = false;
                    nodeMap.set(node.id, entry.mesh.position);
                } else {
                    // ¡NUEVO BLOQUE ENSAMBLADO!
                    // Efecto de construcción: Cae desde arriba y crece con rebote
                    const tex = this.getOrCreateTexture(node.label, colorHex);
                    const matSides = new THREE.MeshStandardMaterial({
                        color: colorInt,
                        metalness: 0.8,
                        roughness: 0.2,
                        transparent: true,
                        opacity: 0.92
                    });
                    const matText = new THREE.MeshStandardMaterial({
                        map: tex,
                        metalness: 0.5,
                        roughness: 0.3
                    });

                    // 6 caras: poner textura en frontal, superior y trasera
                    const materials = [matSides, matSides, matText, matSides, matText, matText];
                    const mesh = new THREE.Mesh(this.algoBoxGeom, materials);

                    // Aparece arriba cayendo hacia su base de construcción
                    mesh.position.set(targetX, targetY + 8.0, targetZ);
                    mesh.scale.set(0.1, 0.1, 0.1);

                    // Bordes brillantes
                    const wireframe = new THREE.LineSegments(
                        this.algoEdgesGeom,
                        new THREE.LineBasicMaterial({ color: 0xffffff, transparent: true, opacity: 0.6 })
                    );
                    mesh.add(wireframe);

                    this.blocksGroup.add(mesh);
                    this.activeBlockMeshes.set(key, {
                        mesh,
                        targetPos: new THREE.Vector3(targetX, targetY, targetZ),
                        targetScale: new THREE.Vector3(1.0, 1.0, 1.0),
                        isDying: false
                    });
                    nodeMap.set(node.id, mesh.position);
                }
            });

            // Trazar rayos de energía láser entre bloques conectados
            while (this.beamsGroup.children.length > 0) {
                this.beamsGroup.remove(this.beamsGroup.children[0]);
            }

            spatialLayout.forEach(node => {
                const targetPos = nodeMap.get(node.id);
                if (!targetPos || !node.inputs) return;

                node.inputs.forEach(srcId => {
                    const srcPos = nodeMap.get(srcId);
                    if (srcPos) {
                        const points = [srcPos, targetPos];
                        const lineGeom = new THREE.BufferGeometry().setFromPoints(points);
                        const lineMat = new THREE.LineBasicMaterial({
                            color: 0x06b6d4,
                            transparent: true,
                            opacity: 0.7,
                            linewidth: 2
                        });
                        const line = new THREE.Line(lineGeom, lineMat);
                        this.beamsGroup.add(line);
                    }
                });
            });

        } else {
            // Modo Voxel: Construcción estructural 3D bloque a bloque (casa / ciudad)
            voxels.forEach(([x, y, z, type]) => {
                const key = `voxel_${x}_${y}_${z}`;
                seenKeys.add(key);

                let colorInt = 0xd97706; // Dorado canela / Golden fur por defecto
                let label = 'PELAJE';
                if (type === 1) { colorInt = 0xd97706; label = 'PELAJE'; } // Marrón dorado cálido
                if (type === 2) { colorInt = 0xfef08a; label = 'PEZUÑAS'; } // Blanco crema / Pezuñas
                if (type === 3) { colorInt = 0xb45309; label = 'CABEZA'; } // Caramelo / Cabeza y orejas
                if (type === 4) { colorInt = 0xfde68a; label = 'PECHO'; } // Pecho y vientre claro
                if (type === 5) { colorInt = 0x78350f; label = 'LOMO'; } // Lomo
                if (type === 6) { colorInt = 0x0f172a; label = 'TRUFA/OJOS'; } // Negro azabache
                if (type === 7) { colorInt = 0xef4444; label = 'COLLAR'; } // Rojo collar

                const targetX = (x - 7.5) * 1.1;
                const targetY = y * 1.1 + 0.5;
                const targetZ = (z - 2.0) * 1.1;

                if (this.activeBlockMeshes.has(key)) {
                    const entry = this.activeBlockMeshes.get(key);
                    entry.targetPos.set(targetX, targetY, targetZ);
                    entry.targetScale.set(1.0, 1.0, 1.0);
                    entry.isDying = false;
                } else {
                    // Nuevo voxel que sube bloque por bloque desde el nivel inferior
                    const mat = new THREE.MeshStandardMaterial({
                        color: colorInt,
                        metalness: 0.6,
                        roughness: 0.3
                    });
                    const mesh = new THREE.Mesh(this.voxelBoxGeom, mat);
                    
                    // Empieza en el suelo y sube hasta su piso
                    mesh.position.set(targetX, 0, targetZ);
                    mesh.scale.set(0.1, 0.1, 0.1);

                    const wireframe = new THREE.LineSegments(
                        this.voxelEdgesGeom,
                        new THREE.LineBasicMaterial({ color: 0x000000, linewidth: 1 })
                    );
                    mesh.add(wireframe);

                    this.blocksGroup.add(mesh);
                    this.activeBlockMeshes.set(key, {
                        mesh,
                        targetPos: new THREE.Vector3(targetX, targetY, targetZ),
                        targetScale: new THREE.Vector3(1.0, 1.0, 1.0),
                        isDying: false
                    });
                }
            });

            // Limpiar rayos en modo voxel
            while (this.beamsGroup.children.length > 0) {
                this.beamsGroup.remove(this.beamsGroup.children[0]);
            }
        }

        // Marcar bloques desaparecidos para animación de desvanecimiento
        for (const [key, entry] of this.activeBlockMeshes.entries()) {
            if (!seenKeys.has(key)) {
                entry.targetScale.set(0.01, 0.01, 0.01);
                entry.isDying = true;
            }
        }
    }

    setupInteractions() {
        const target = this.hasThree ? this.webglRenderer.domElement : this.canvas;

        const onDown = (clientX, clientY) => {
            this.isDragging = true;
            this.lastMouseX = clientX;
            this.lastMouseY = clientY;
        };

        const onMove = (clientX, clientY) => {
            if (!this.isDragging) return;
            const dx = clientX - this.lastMouseX;
            const dy = clientY - this.lastMouseY;
            this.targetRotY += dx * 0.009;
            this.targetRotX = Math.max(-1.3, Math.min(1.3, this.targetRotX + dy * 0.009));
            this.lastMouseX = clientX;
            this.lastMouseY = clientY;
        };

        const onUp = () => {
            this.isDragging = false;
        };

        target.addEventListener('mousedown', e => onDown(e.clientX, e.clientY));
        window.addEventListener('mousemove', e => onMove(e.clientX, e.clientY));
        window.addEventListener('mouseup', onUp);

        // Soporte táctil móvil (1 dedo: rotación 360°, 2 dedos: zoom fluido)
        let initialPinchDist = 0;
        target.addEventListener('touchstart', e => {
            if (e.touches.length === 1) {
                onDown(e.touches[0].clientX, e.touches[0].clientY);
            } else if (e.touches.length === 2) {
                const dx = e.touches[0].clientX - e.touches[1].clientX;
                const dy = e.touches[0].clientY - e.touches[1].clientY;
                initialPinchDist = Math.hypot(dx, dy);
            }
        }, { passive: false });

        target.addEventListener('touchmove', e => {
            if (e.touches.length === 1) {
                onMove(e.touches[0].clientX, e.touches[0].clientY);
            } else if (e.touches.length === 2) {
                const dx = e.touches[0].clientX - e.touches[1].clientX;
                const dy = e.touches[0].clientY - e.touches[1].clientY;
                const dist = Math.hypot(dx, dy);
                const diff = dist - initialPinchDist;
                this.targetZoom = Math.max(10, Math.min(65, this.targetZoom - diff * 0.08));
                initialPinchDist = dist;
            }
            e.preventDefault();
        }, { passive: false });

        target.addEventListener('touchend', onUp);

        target.addEventListener('wheel', e => {
            this.targetZoom = Math.max(10, Math.min(65, this.targetZoom + e.deltaY * 0.03));
            e.preventDefault();
        }, { passive: false });
    }

    animate() {
        requestAnimationFrame(this.animate);
        this.pulseTime += 0.05;

        // Auto-rotación si está activada
        if (this.autoRotate) {
            this.targetRotY += 0.006;
        }

        // Suavizado cinemático de cámara (LERP suave)
        this.rotX += (this.targetRotX - this.rotX) * 0.12;
        this.rotY += (this.targetRotY - this.rotY) * 0.12;
        this.zoom += (this.targetZoom - this.zoom) * 0.12;

        if (this.hasThree) {
            // Posicionamiento de cámara en coordenadas esféricas
            const radius = this.zoom;
            this.camera.position.x = radius * Math.sin(this.rotY) * Math.cos(this.rotX);
            this.camera.position.y = radius * Math.sin(this.rotX) + 5.5;
            this.camera.position.z = radius * Math.cos(this.rotY) * Math.cos(this.rotX);
            this.camera.lookAt(0, 4.5, 0);

            // Interpolación física de cada bloque (efecto de ensamble bloque por bloque)
            for (const [key, entry] of this.activeBlockMeshes.entries()) {
                entry.mesh.position.lerp(entry.targetPos, 0.18);
                entry.mesh.scale.lerp(entry.targetScale, 0.20);

                // Si ha muerto y ya se redujo a casi 0, eliminar de la escena
                if (entry.isDying && entry.mesh.scale.x < 0.05) {
                    this.blocksGroup.remove(entry.mesh);
                    this.activeBlockMeshes.delete(key);
                }
            }

            // Pulsación de energía en haces láser
            this.beamsGroup.children.forEach((beam, idx) => {
                if (beam.material) {
                    beam.material.opacity = 0.35 + 0.35 * Math.sin(this.pulseTime * 2.5 + idx);
                }
            });

            this.webglRenderer.render(this.scene, this.camera);
        } else {
            this.renderFallback2D();
        }
    }

    renderFallback2D() {
        if (!this.ctx || !this.container) return;
        const rect = this.container.getBoundingClientRect();
        if (this.canvas.width !== rect.width || this.canvas.height !== rect.height) {
            this.canvas.width = rect.width;
            this.canvas.height = rect.height;
        }

        const w = rect.width;
        const h = rect.height;

        this.ctx.fillStyle = '#020408';
        this.ctx.fillRect(0, 0, w, h);

        const cx = w / 2;
        const cy = h / 2 + 20;

        // Proyección isométrica 2.5D
        const iso = (x, y, z) => {
            const rx = x * Math.cos(this.rotY) - z * Math.sin(this.rotY);
            const rz = x * Math.sin(this.rotY) + z * Math.cos(this.rotY);
            const screenX = cx + (rx - rz) * (this.zoom * 0.7);
            const screenY = cy + (rx + rz) * (this.zoom * 0.35) - y * (this.zoom * 0.85);
            return { x: screenX, y: screenY };
        };

        // Suelo de cuadrícula isométrica
        this.ctx.strokeStyle = '#131b26';
        this.ctx.lineWidth = 1;
        for (let gx = -4; gx <= 4; gx += 2) {
            const p1 = iso(gx, 0, -4);
            const p2 = iso(gx, 0, 4);
            this.ctx.beginPath();
            this.ctx.moveTo(p1.x, p1.y);
            this.ctx.lineTo(p2.x, p2.y);
            this.ctx.stroke();
        }

        // Dibujar bloques según el modo
        if (this.mode === 'algorithm') {
            this.currentLayout.forEach(node => {
                const pos = iso(node.x, node.y, node.z);
                let color = '#06b6d4';
                if (node.color_tag === 'input') color = '#a855f7';
                if (node.color_tag === 'constant') color = '#64748b';
                if (node.color_tag === 'output') color = '#10b981';

                this.ctx.fillStyle = color;
                this.ctx.strokeStyle = '#ffffff';
                this.ctx.lineWidth = 1.5;
                this.ctx.fillRect(pos.x - 22, pos.y - 14, 44, 28);
                this.ctx.strokeRect(pos.x - 22, pos.y - 14, 44, 28);

                this.ctx.fillStyle = '#ffffff';
                this.ctx.font = 'bold 10px monospace';
                this.ctx.textAlign = 'center';
                this.ctx.fillText(node.label, pos.x, pos.y + 4);
            });
        } else {
            this.currentVoxels.forEach(([x, y, z, type]) => {
                const pos = iso(x * 1.1, y * 1.1, z * 1.1);
                let color = '#3b82f6';
                if (type === 2) color = '#94a3b8';
                if (type === 3) color = '#06b6d4';
                if (type === 4) color = '#f59e0b';

                this.ctx.fillStyle = color;
                this.ctx.strokeStyle = '#020408';
                this.ctx.lineWidth = 1;
                this.ctx.fillRect(pos.x - 12, pos.y - 12, 24, 24);
                this.ctx.strokeRect(pos.x - 12, pos.y - 12, 24, 24);
            });
        }
    }
}
