<script>
// @ts-nocheck
    import logo from "$lib/assets/logo.svg";
    import loading from "$lib/assets/loading.svg"
    import done from "$lib/assets/done.svg";
    import copy from "$lib/assets/copy.svg";
    import save from "$lib/assets/save.svg";
    import { goto } from "$app/navigation";
    import Gutter from "./gutter.svelte";
    import Split from "split.js";
    import { onMount } from "svelte";
    import { slide, scale } from "svelte/transition";
    import { browser } from '$app/environment';
    import { draggable, hoverable } from "$lib/attachments";
    import { render } from "svelte/server";
    import { PcbDesign } from "$lib/pcbDesign";
    import eye from "$lib/assets/eye.svg";
    import eyeOff from "$lib/assets/eye-off.svg";
    import { updateBoard } from "$lib";
    import { tick } from 'svelte';

    class BillOfMaterials {
        constructor(data, savedData) {
            const pcbPatternsBook = new Map(
                data.pcbDesign.multiLayer.patterns.map(p => [p.refDesRef, p.patternRef])
            );


            const compDefsBook = new Map(
                data.library.compDef.map((item, idx) => [item.name, idx])    
            );

            const installedSet = new Set(savedData?.installed || []);
            const compInst = data.netlist.compInst;

            this.components = $state(compInst.map(component => {
                const libIdx = compDefsBook.get(component.compRef);
                return {
                    refDes: component.ref_des, 
                    compRef: component.compRef, 
                    compValue: this.defineCompValue(component, libIdx, data),
                    patternName: pcbPatternsBook.get(component.ref_des)|| data.library.compDef[libIdx].attachedPattern.patternName,
                    installed: installedSet.has(component.ref_des) 
                };
            }));

            this.grouped = $derived.by(() => {
                const book = new Map();
                this.components.forEach(comp => {
                    const groupKey = `${comp.patternName}|${comp.compValue}`;
                    if (!book.has(groupKey)) {
                        book.set(groupKey, {
                            patternName: comp.patternName,
                            compValue: comp.compValue,
                            refs: []
                        });
                    }
                    book.get(groupKey).refs.push(comp);
                });

                return Array.from(book.values()).map(group => {
                    return {
                        ...group,
                        get isInstalled() {
                            return this.refs.every(r => r.installed);
                        },
                        get isIndeterminate() {
                            let hasInstalled = false, hasUninstalled = false;
                            for (const r of this.refs) {
                                if (r.installed) hasInstalled = true;
                                else hasUninstalled = true;
                                if (hasInstalled && hasUninstalled) return true; 
                            }
                            return false;
                        }
                    };
                });
            });
        }

        toggleGroup(group, status) {
            group.refs.forEach(comp => {
                comp.installed = status;
            });
        }

        reorderItems(from, to, arrName) {
            if (from === to) return;  
            const arr = [...this[arrName]];
            const { 0: item } = arr.splice(from, 1);
            arr.splice(to, 0, item);
            this[arrName] = arr;
        }

        defineCompValue(component, libIdx, data) {
            if (component.compValue) return component.compValue;
            var libDef = data.library.compDef[libIdx];
            return libDef ? libDef.originalName : component.originalName;
        }
    }

    class PcbRenderer {
        constructor(sidesConfiguration, pcbData) {
            this.sidesConfiguration = sidesConfiguration;
            this.pcbData = pcbData;

            this.settings = {
                silk: true,
                footPrint: true,
                net: true,
                holes: true,
                keepOuts: true,
                silkWidth: 0.2
            }

            this.sidesConfiguration.front.order = [...this.pcbData.order];
            this.sidesConfiguration.back.order = [...this.pcbData.order].reverse();

            this.sidesConfiguration.front.visibility = {};
            this.sidesConfiguration.back.visibility = {};

            this.pcbData.order.forEach(layerRef => {
                const isVisible = this.pcbData.layers[layerRef].visible;
                this.sidesConfiguration.front.visibility[layerRef] = isVisible;
                this.sidesConfiguration.back.visibility[layerRef] = isVisible;
            });

            this.sidesConfiguration.front.pointersData = {};
            this.sidesConfiguration.back.pointersData = {};
            this.layerStyles = {};

            this.initHandlers(this.sidesConfiguration.front);
            this.initHandlers(this.sidesConfiguration.back);
            this.highlighted = new Set();
            this.installed = new Set();
            this.selectedTrack = null;
        }

        initHandlers(sideConfiguration) {
            const parentDiv = sideConfiguration.sideParent;
            parentDiv.addEventListener('pointerdown', (e) => this.onPanStart(e, sideConfiguration));
            window.addEventListener('pointermove', (e) => this.onPanMove(e, sideConfiguration));
            window.addEventListener('pointerup', (e) => this.onPanEnd(e, sideConfiguration));
            parentDiv.addEventListener("pointerleave", (e) => this.leavePointer(e, sideConfiguration))
            parentDiv.addEventListener("pointercancel", (e) => this.leavePointer(e, sideConfiguration))
            parentDiv.addEventListener("pointerout", (e) => this.leavePointer(e, sideConfiguration))
            parentDiv.addEventListener('wheel', (e) => this.onZoomWheel(e, sideConfiguration)); 
        }

        setLayerVisibility(sideName, layerRef, isVisible) {
            const sideConfig = this.sidesConfiguration[sideName];
            if (sideConfig) {
                sideConfig.visibility[layerRef] = isVisible;
                this.render(false); 
            }
        }

        setLayerOrder(sideName, newOrder) {
            const sideConfig = this.sidesConfiguration[sideName];
            if (sideConfig) {
                sideConfig.order = newOrder;
                this.render(false);
            }
        }


        getLayerStyle(layerRef) {
            if (this.layerStyles[layerRef]) {
                return this.layerStyles[layerRef];
            }

            const index = this.pcbData.order.indexOf(layerRef);
            
            let hue = 540 + ((index * 137.5) % 40); 
            const saturation = 20 + (index % 3) * 5;  
            const lightness = 80 - (index % 5) * 2;   
            const netColor = `hsl(${hue}, ${saturation}%, ${lightness}%)`; 
            const zoneColor = `hsl(${hue}, ${saturation + 8}%, ${lightness - 5}%)`; 

            this.layerStyles[layerRef] = {
                color: zoneColor, 
                alpha: 0.9,       
                netColor: netColor 
            };

            return this.layerStyles[layerRef];
        }

        onZoomWheel(e, sideConfiguration) {
            e.preventDefault();
            e.stopPropagation();

            var deltaY = e.deltaY;

            switch (e.deltaMode) {
                case WheelEvent.DOM_DELTA_LINE:
                    deltaY *= 30;
                    break;
                case WheelEvent.DOM_DELTA_PAGE:
                    deltaY *= 300; 
                    break;
            }

            var scaleFactor = Math.min(
                Math.max(Math.pow(1.1, -deltaY / 40), 0.5), 
                2
            );


            var devicePixelRatio = window.devicePixelRatio;

            const canvasWidth = sideConfiguration.sideParent.clientWidth * devicePixelRatio;
            const canvasHeight = sideConfiguration.sideParent.clientHeight * devicePixelRatio;

            const dx = (devicePixelRatio * e.offsetX - (canvasWidth / 2) - sideConfiguration.panX);
            const dy = (devicePixelRatio * e.offsetY - (canvasHeight / 2) - sideConfiguration.panY);

            sideConfiguration.zoom *= scaleFactor;
            sideConfiguration.panX += dx * (1 - scaleFactor);
            sideConfiguration.panY += dy * (1 - scaleFactor);
            this.redrawSide(sideConfiguration);
        }

        leavePointer(event, sideConfiguration) {
            event.preventDefault();
            event.stopPropagation();

            this.redrawSide(sideConfiguration);

            sideConfiguration.sideParent.style.cursor = "crosshair";
            if (event && sideConfiguration.pointersData) {
                delete sideConfiguration.pointersData[event.pointerId];
            }
        }

        syncInstalled(refDes, installed) {
            if (installed) {
                this.installed.add(refDes);
            } else {
                this.installed.delete(refDes);
            }
            this.render();
        }

        syncGroupInstalled(refs, installed) {
            const refDesList = refs.map(r => r.refDes);
            if (installed) {
                refDesList.forEach(ref => this.installed.add(ref));
            } else {
                refDesList.forEach(ref => this.installed.delete(ref));
            }
            this.render();
        }

        onPanStart(event, sideConfiguration) {
            sideConfiguration.sideParent.style.cursor = "grabbing";
            if (event.button != 0 && event.button != 1) {
                return;
            }

            event.preventDefault();
            event.stopPropagation();

            const { offsetX, offsetY } = event;
            sideConfiguration.pointersData[event.pointerId] = {
                lastX: offsetX,
                lastY: offsetY,
                startY: offsetY,
                startX: offsetX,
                travelled: 0
            }
        }

        onPanMove(event, sideConfiguration) {
            const thisPtr = sideConfiguration.pointersData[event.pointerId];
            
            if (!thisPtr) 
                return;

            const dx = event.offsetX - thisPtr.lastX;
            const dy = event.offsetY - thisPtr.lastY;

            if (Object.keys(sideConfiguration.pointersData).length === 1) {
                sideConfiguration.panX += dx * window.devicePixelRatio;
                sideConfiguration.panY += dy * window.devicePixelRatio;
            }

            thisPtr.lastX = event.offsetX;
            thisPtr.lastY = event.offsetY;

            this.redrawSide(sideConfiguration);
        }

        onPanEnd(event, sideConfiguration) {
            event.preventDefault();
            event.stopPropagation();
            const ptr = sideConfiguration.pointersData[event.pointerId];
            if (ptr && Math.hypot(event.offsetX - ptr.startX, event.offsetY - ptr.startY) < 5) {
                this.handleCanvasClick(event, sideConfiguration);
            }

            sideConfiguration.sideParent.style.cursor = "crosshair";
            if (event && sideConfiguration.pointersData) {
                delete sideConfiguration.pointersData[event.pointerId];
            }
        }

        observeCanvases() {
            var resizer = new ResizeObserver(entries => {
                entries.forEach(entry => {
                    const target = entry.target;
                    if (target === this.sidesConfiguration.front.sideParent) {
                        this.resizeSide(this.sidesConfiguration.front);
                    } else if (target === this.sidesConfiguration.back.sideParent) {
                        this.resizeSide(this.sidesConfiguration.back);
                    }
                });
            });
            
            resizer.observe(this.sidesConfiguration.front.sideParent);
            resizer.observe(this.sidesConfiguration.back.sideParent);
            
            this.resizer = resizer;
        }

        handleCanvasClick({ offsetX, offsetY }, sideConfiguration) {
            const devicePixelRatio = window.devicePixelRatio;
            const canvasX = offsetX * devicePixelRatio;
            const canvasY = offsetY * devicePixelRatio;

            const ctxContent = sideConfiguration.contexts.content;
            let foundRefDes = null;
            
            for (var layerRef of this.pcbData.order) {

                let layer = this.pcbData.layers[layerRef];
                if (sideConfiguration.visibility?.[layerRef] === false) 
                    continue;

                if (!layer.instances) continue;
                
                for (const { 0: refDes, 1: instance } of Object.entries(layer.instances)) {
                    if (
                        instance.footPrint && ctxContent.isPointInPath(instance.footPrint, canvasX, canvasY)
                        || instance.keepOuts && ctxContent.isPointInPath(instance.keepOuts, canvasX, canvasY)
                    ) {
                        foundRefDes = refDes;
                        break;
                    }
                }
                if (foundRefDes) break;
            }
            
            if (foundRefDes) {
                this.selectedTrack = null; 
                if (!this.onSelectNet instanceof Function) 
                    this.onSelectNet(null);

                if (this.highlighted.has(foundRefDes)) {
                    this.highlighted.delete(foundRefDes);
                } else {
                    this.highlighted.clear();
                    this.highlightComp(foundRefDes);
                }
                if (this.onSelect instanceof Function) 
                    this.onSelect(foundRefDes);
            } else {
                const trackData = this.getTrackAtPoint(canvasX, canvasY, sideConfiguration);
                if (trackData) {
                    if (this.selectedTrack?.path === trackData.path) {
                        if (this.onSelectNet instanceof Function) 
                            this.onSelectNet(null);
                        this.selectedTrack = null;
                    } else {
                        this.selectedTrack = trackData;
                        if (this.onSelectNet instanceof Function) 
                            this.onSelectNet(trackData.net);

                        this.highlighted.clear();
                    }
                } else {
                    this.selectedTrack = null;
                    if (this.onSelectNet instanceof Function) 
                        this.onSelectNet(null);
                    this.highlighted.clear();
                }
            }

            this.redrawSide(this.sidesConfiguration.front);
            this.redrawSide(this.sidesConfiguration.back);
        }

        redrawWithTransform(sideConfiguration) {
            const ctx = sideConfiguration.contexts.content;
            const hCtx = sideConfiguration.contexts.highlight;
            
            this.reloadContext(ctx, sideConfiguration);
            this.clearContext(ctx); 
            
            this.drawEdges(ctx);
            this.pcbData.order.forEach((layerRef) => {
                var layer = this.pcbData[layerRef];
                if (this.settings.net) {
                    this.drawNets(ctx, layer, layerRef);
                    this.drawVias(ctx, layer, sideConfiguration); 
                    this.drawZones(ctx, layer, sideConfiguration); 
                }
                this.drawInstances(ctx, layer, sideConfiguration);
            });

            this.redrawHighlightLayer(sideConfiguration);
        }

        getTrackAtPoint(canvasX, canvasY, sideConfiguration) {
            const ctx = sideConfiguration.contexts.content;

            ctx.save(); 
            this.reloadContext(ctx, sideConfiguration);

            const matrix = ctx.getTransform();
            const invMatrix = matrix.invertSelf();
            
            const pcbX = canvasX * invMatrix.a + canvasY * invMatrix.c + invMatrix.e;
            const pcbY = canvasX * invMatrix.b + canvasY * invMatrix.d + invMatrix.f;

            ctx.setTransform(1, 0, 0, 1, 0, 0);

            for (const layerRef of this.pcbData.order) {
                const layer = this.pcbData.layers[layerRef];
                if (!layer || !layer.visible) continue;

                if (layer.nets) {
                    for (const [netName, widths] of Object.entries(layer.nets)) {
                        for (const { width, path } of Object.values(widths)) {
                            if (ctx.isPointInStroke(path, pcbX, pcbY)) {
                                ctx.restore(); 
                                return {
                                    net: netName,
                                    path: path,
                                    width: width,
                                    isPour: false 
                                }; 
                            }
                        }
                    }
                }

                if (layer.pours) {
                    for (const [netName, widths] of Object.entries(layer.pours)) {
                        for (const { pour } of Object.values(widths)) {
                            if (pour && pour.copper && ctx.isPointInPath(pour.copper, pcbX, pcbY)) {
                                ctx.restore();
                                console.log("Найден полигон:", netName);
                                return {
                                    net: netName,
                                    path: pour.copper,
                                    width: null,
                                    isPour: true
                                }; 
                            }
                        }
                    }
                }
            }
            
            ctx.restore(); 
            return null;
        }

        render(forceFullRedraw) {
            (!forceFullRedraw) && (forceFullRedraw = true);
            if (forceFullRedraw) {
                this.resizeSide(this.sidesConfiguration.front);
                this.redrawSide(this.sidesConfiguration.front);
                this.resizeSide(this.sidesConfiguration.back);
                this.redrawSide(this.sidesConfiguration.back);
            } else {
                this.redrawWithTransform(this.sidesConfiguration.front);
                this.redrawWithTransform(this.sidesConfiguration.back);
            }
        }

        toggleInstalled(refDes) {
            (this.installed.has(refDes))
                ? this.installed.delete(refDes)
                : this.installed.add(refDes);
            
            this.redrawHighlightLayer(this.sidesConfiguration.front);
            this.redrawHighlightLayer(this.sidesConfiguration.back);
        }

        toggleInstalledMultiple(refDesList, checked) {
            (!checked)
                ? refDesList.forEach(item => this.installed.delete(item.refDes))
                : refDesList.forEach(item => this.installed.add(item.refDes));
            
            this.redrawHighlightLayer(this.sidesConfiguration.front);
            this.redrawHighlightLayer(this.sidesConfiguration.back);
        }


        deg2rad(angle) {
            return angle * (Math.PI / 180);
        }

        clearSide(sideConfiguration) {
            var ctxs = sideConfiguration.contexts
            this.clearContext(ctxs.context);
            // this.clearContext(ctxs.fab);
        }
        
        drawNets(context, layer, layerRef) {
            const style = this.getLayerStyle(layer);
            context.lineCap = "round";
            context.lineJoin = "round";

            Object.values(layer.nets).forEach(netWidths => {
                Object.values(netWidths).forEach(({ path, width }) => {
                    context.strokeStyle = style.netColor || '#555'; 
                    context.lineWidth = width;
                    context.stroke(path);
                });
            });
        }

        redrawHighlightLayer(sideConfiguration) {
            const hCtx = sideConfiguration.contexts.highlight;
            
            this.clearContext(hCtx);
            
            if (this.highlighted.size === 0 && this.installed.size === 0 && !this.selectedTrack) 
                return;

            this.reloadContext(hCtx, sideConfiguration);
            if (this.selectedTrack) { 
                hCtx.save();
                hCtx.lineCap = "round";
                hCtx.lineJoin = "round";

                if (this.selectedTrack.isPour) {
                    hCtx.fillStyle = "rgba(255, 179, 0, 0.4)";
                    hCtx.strokeStyle = "rgba(255, 179, 0, 0.4)";
                    hCtx.lineWidth = this.pcbData.mm2px(0.2);
                    hCtx.fill(this.selectedTrack.path, "evenodd");
                    hCtx.stroke(this.selectedTrack.path);
                } else {
                    hCtx.strokeStyle = "rgba(255, 179, 0, 0.4)"; 

                    const targetNetName = this.selectedTrack.net;

                    this.pcbData.order.forEach(layerRef => {
                        const layer = this.pcbData.layers[layerRef];
                        if (sideConfiguration.visibility?.[layerRef] === false) 
                            return;
                        
                        if (layer && layer.nets && layer.nets[targetNetName]) {
                            
                            Object.values(layer.nets[targetNetName]).forEach(({ path, width }) => {
                                hCtx.lineWidth = width + this.pcbData.mm2px(0.3); 
                                hCtx.stroke(path);
                            });
                        }
                    });
                }
                hCtx.restore();
            }
            
            this.pcbData.order.forEach(layerRef => {
                var layer = this.pcbData.layers[layerRef];
                if (sideConfiguration.visibility?.[layerRef] === false) 
                    return;

                if (!layer.instances) 
                    return;

                this.installed.forEach(key => {
                    const instance = layer.instances[key];
                    if (!instance) return;
                    
                    hCtx.save();
                    
                    if (this.settings.footPrint && instance.footPrint) {
                        hCtx.fillStyle = "#00AA44";      
                        hCtx.fill(instance.footPrint);
                    }

                    if (this.settings.silk) {
                        // hCtx.strokeStyle = "#83D03F";    
                        hCtx.strokeStyle = "#00CC55";    
                        hCtx.lineCap = "round";
                        hCtx.lineJoin = "round";
                        
                        if (instance.lines) 
                            Object.values(instance.lines).forEach(
                                a => {
                                    hCtx.lineWidth = a.width;
                                    hCtx.stroke(a.path);
                                }
                            );
                        if (instance.arcs) 
                            Object.values(instance.arcs).forEach(
                                a => {
                                    hCtx.lineWidth = a.width;
                                    hCtx.stroke(a.path);
                                }
                            );
                        if (instance.silk) 
                            hCtx.stroke(instance.silk);
                    }
                    
                    hCtx.restore();
                });

                this.highlighted.forEach(key => {
                    
                    const instance = layer.instances[key];
                    if (!instance) return;
                    
                    hCtx.save();
                    
                    if (this.settings.footPrint && instance.footPrint) {
                        hCtx.fillStyle = this.installed.has(key) 
                            ? "#A5D6A7"   
                            : "#FFD700";  
                        hCtx.fill(instance.footPrint);
                    }

                    if (this.settings.silk) {
                        hCtx.strokeStyle = this.installed.has(key)
                            ? "#2E7D32"   
                            : "#FF4444";  
                        hCtx.lineWidth = this.pcbData.mm2px(0.4);
                        hCtx.lineCap = "round";
                        hCtx.lineJoin = "round";
                        
                        if (instance.lines) 
                            Object.values(instance.lines).forEach(l => hCtx.stroke(l.path));
                        if (instance.arcs) 
                            Object.values(instance.arcs).forEach(a => hCtx.stroke(a.path));
                        if (instance.silk) 
                            hCtx.stroke(instance.silk);
                    }
                    
                    hCtx.restore();
                });
            });
        }

        drawEdges(context) {
            if (!this.pcbData.edges?.length) return;
            
            context.strokeStyle = "#000000";
            context.lineWidth = this.pcbData.mm2px(0.2);
            context.lineCap = "round";
            
            this.pcbData.edges.forEach(edge => {
                context.beginPath();
                context.moveTo(edge.start[0], edge.start[1]);
                context.lineTo(edge.end[0], edge.end[1]);
                context.stroke();
            });
        }

        drawInstances(context, layer, sideConfiguration) {
            const isFlipped = sideConfiguration.flipped;
            
            Object.entries(layer.instances).forEach(
                ({ 0: key, 1: instance}) => {

                if (this.settings.footPrint && instance.footPrint) {
                    context.save();
                    context.fillStyle = "#878787"; 
                    context.fill(instance.footPrint);
                    context.restore();

                    if (instance.holes) {
                        context.save();
                        context.fillStyle = "#CCCCCC"; 
                        context.fill(instance.holes);
                        context.restore();
                    }
                }

                if (this.settings.silk) {
                    context.save();
                    // context.strokeStyle = "#c2a842";
                    context.strokeStyle = "#c20042";
                    context.lineWidth = this.pcbData.mm2px(this.settings.silkWidth || 0.2);
                    context.lineCap = "round";
                    context.lineJoin = "round";

                    if (instance.lines) {
                        for (const lineData of Object.values(instance.lines)) {
                            context.stroke(lineData.path);
                        }
                    }
                    if (instance.arcs) {
                        for (const arcData of Object.values(instance.arcs)) {
                            context.stroke(arcData.path);
                        }
                    }

                    if (instance.silk) {
                        context.stroke(instance.silk);
                    }
                    context.restore();
                }

                if (!this.installed.has(key) && this.settings.keepOuts && instance.keepOuts) {
                    context.save();
                    context.fillStyle = "rgba(255, 120, 80, 0.15)"; 
                    context.strokeStyle = "rgba(255, 80, 40, 0.6)";
                    context.lineWidth = this.pcbData.mm2px(0.1);
                    
                    context.fill(instance.keepOuts, "evenodd");
                    context.stroke(instance.keepOuts);
                    context.restore();
                }
            });
        }

        drawZones(context, layer, sideConfiguration) {
            if (!layer.pours) return;

            const isFlipped = sideConfiguration.flipped;
            const style = this.getLayerStyle(layer);

            Object.values(layer.pours).forEach(netWidths => {
                Object.values(netWidths).forEach(({ pour, width }) => {
                    if (pour && pour.copper) {
                        context.fillStyle = style.color; 
                        context.fill(pour.copper, "evenodd");
                        context.lineWidth = width || this.pcbData.mm2px(0.2); 
                    }
                });
            });
        }

        drawVias(context, layer, sideConfiguration) {
            if (!layer.vias || !layer.vias.vias) return;
            
            context.save();
            context.fillStyle = "#757575"; 
            context.fill(layer.vias.vias);

            context.globalCompositeOperation = "destination-out"; 
            if (layer.vias.holes) {
                context.fill(layer.vias.holes);
            }
            
            context.restore();
        }

        clearContext(context) {
            context.save();
            context.resetTransform();
            context.clearRect(0, 0, context.canvas.width, context.canvas.height);
            context.restore();
        }

        resizeSide(sideConfiguration) {
            this.updateSideScale(sideConfiguration);
            this.redrawSide(sideConfiguration);
        }

        redrawSide(sideConfiguration) {
            const ctx = sideConfiguration.contexts.content;
            this.reloadContext(ctx, sideConfiguration);
            this.clearContext(ctx);

            this.drawEdges(sideConfiguration.contexts.content);
            const layersOrder = sideConfiguration.order || this.pcbData.order;

            layersOrder.forEach((layerRef) => {
                var layer = this.pcbData.layers[layerRef];
                if (!layer) return;
                if (sideConfiguration.visibility[layerRef] === false) return;

                if (this.settings.net) {
                    this.drawZones(ctx, layer, layerRef, sideConfiguration); 
                    this.drawNets(ctx, layer, layerRef); 
                    this.drawVias(ctx, layer, sideConfiguration); 
                }

                this.drawInstances(ctx, layer, sideConfiguration);
            });

            this.redrawHighlightLayer(sideConfiguration);
        }


        updateSideScale(sideConfiguration) {
            const { clientWidth, clientHeight } = sideConfiguration.sideParent;
            const height = clientHeight * window.devicePixelRatio;
            const width = clientWidth * window.devicePixelRatio;

            var bbox = this.recalcBbox(sideConfiguration);
            const bboxXlen = (bbox.maxX - bbox.minX);
            const bboxYlen = (bbox.maxY - bbox.minY);

            var scaleFactor = 0.98 * Math.min(
                (width / bboxXlen),
                (height / bboxYlen)
            ); 

            if (scaleFactor < 0.1) {
                return;
            }

            sideConfiguration.scale = scaleFactor;
            sideConfiguration.x = sideConfiguration.flipped
                ? (width + (bboxXlen * scaleFactor)) / 2
                : (width - (bboxXlen * scaleFactor)) / 2;
            sideConfiguration.y = (height - (bboxYlen * scaleFactor)) / 2;
            Object.values(sideConfiguration.contexts).forEach(ctx => {
                ctx.canvas.width = width;
                ctx.canvas.height = height;
            });
        }

        recalcBbox(sideConfiguration) {
            var { rotation } = sideConfiguration;
            var bbox = this.pcbData.bbox;
            var corners = [
                [bbox.minX, bbox.minY],
                [bbox.minX, bbox.maxY],
                [bbox.maxX, bbox.maxY],
                [bbox.maxX, bbox.minY],
            ]

            var calculated = corners.map((vector) => this.rotateVec(vector, rotation));
            return calculated.reduce(
                (acc, el) => {
                    acc.minX = Math.min(acc.minX, el[0]);
                    acc.minY = Math.min(acc.minY, el[1]);
                    acc.maxX = Math.max(acc.maxX, el[0]);
                    acc.maxY = Math.max(acc.maxY, el[1]);

                    return acc;
                },
                {
                    minX: Infinity, 
                    minY: Infinity,
                    maxX: -Infinity,
                    maxY: -Infinity
                }
            );
        }

        highlightComp(refDes) {
            this.highlighted.add(refDes);
            this.redrawHighlightLayer(this.sidesConfiguration.front);
            this.redrawHighlightLayer(this.sidesConfiguration.back);

            return () => { 
                this.highlighted.delete(refDes);
                this.redrawHighlightLayer(this.sidesConfiguration.front);
                this.redrawHighlightLayer(this.sidesConfiguration.back);
            }
        }

        highlightGroup(refDefs) {
            var cleaners = refDefs.map(ref => this.highlightComp(ref.refDes));
            return () => {
                cleaners.forEach(fn => fn());
            };
        }

        highlight() {
            this.redrawSide(this.sidesConfiguration.front);
            this.redrawSide(this.sidesConfiguration.back);
        }

        rotateVec(vector, angle) {
            const radian = this.deg2rad(angle)
            return [
                vector[0] * Math.cos(radian) - vector[1] * Math.sin(radian),
                vector[0] * Math.sin(radian) + vector[1] * Math.cos(radian),
            ]
        }

        reloadContext(context, { zoom, panX, panY, rotation, scale, flipped, sideParent }) {  
            const canvasWidth = sideParent.clientWidth * window.devicePixelRatio;
            const canvasHeight = sideParent.clientHeight * window.devicePixelRatio;
            
            const finalZoom = zoom * (scale);
            const bbox = this.pcbData.bbox;

            context.setTransform(1, 0, 0, 1, 0, 0);
            context.translate(canvasWidth / 2, canvasHeight / 2);
            context.translate(panX, panY);

            context.scale( flipped ? finalZoom : -finalZoom, finalZoom);

            context.rotate(this.deg2rad(rotation + 180));

            const pcbCenterX = (bbox.minX + bbox.maxX) / 2;
            const pcbCenterY = (bbox.minY + bbox.maxY) / 2;
            
            context.translate(-pcbCenterX, -pcbCenterY);
        }
    }


    var getStyleVal = (
        (htmlEl, cssProp) => window.getComputedStyle(htmlEl, null).getPropertyValue(cssProp)
    );

    var colPaddingDiff = (
        (htmlCol) => {
            if (getStyleVal(htmlCol, 'box-sizing') == "border-box") {
                return 0;
            }

            return parseInt(getStyleVal(htmlCol, "padding-left")) 
                + parseInt(getStyleVal(htmlCol, "padding-right"));
        }
    );

    var resizable = (
        (content) => {
            var startX, curCol, nxtCol, curColWidth, nxtColWidth, rowWidth;

            var handleMouseDown = (
                (event) => {
                    event.preventDefault();
                    event.stopPropagation();

                    curCol = event.target.parentElement;
                    nxtCol = curCol.nextElementSibling;
                    startX = event.pageX;

                    var padding = colPaddingDiff(curCol);

                    rowWidth = curCol.parentElement.offsetWidth;
                    curColWidth = curCol.clientWidth - padding;
                    nxtColWidth = nxtCol.clientWidth - padding;

                    window.addEventListener("mousemove", handleMouseMove);
                    window.addEventListener("mouseup", handleMouseUp);
                }
            );
            
            var handleMouseMove = (
                (event) => {
                    if (startX) {
                        var diffX = event.pageX - startX;
                        diffX = -Math.min(-diffX, curColWidth - 20);
                        diffX = Math.min(diffX, nxtColWidth - 20);

                        curCol.style.width = `${((curColWidth + diffX) / rowWidth) * 100}%`
                        nxtCol.style.width = `${((nxtColWidth - diffX) / rowWidth) * 100}%`
                    }
                }
            )

            var handleMouseUp = (
                () => {
                    curCol = undefined;
                    nxtCol = undefined;
                    startX = undefined;
                    nxtColWidth = undefined;
                    curColWidth = undefined

                    window.removeEventListener("mousemove", handleMouseMove);
                    window.removeEventListener("mouseup", handleMouseUp);
                }
            );

            content.addEventListener("mousedown", handleMouseDown);
            

            return {
                destroy() {
                    content.removeEventListener("mousedown", handleMouseDown);
                    window.removeEventListener("mousemove", handleMouseMove);
                    window.removeEventListener("mouseup", handleMouseUp);
                }
            };
        }
    );

    var handleReorder = (fromIndex, toIndex) => {
        if (viewMode == 0) {
            bom.reorderItems(fromIndex, toIndex, "components");
            bom.components = bom.components;
        }
        else {
            bom.reorderItems(fromIndex, toIndex, "grouped");
            bom.grouped = bom.grouped;
        }

    }

    var { data } = $props();

    var isGuest = data.guest; 

    var viewMode = $state(0);

    var rotation = $state(0);
    var pcbBoard = data.serverData.data;
    var pcbSave = pcbBoard.save;
    var publicToken = pcbBoard.public_token;
    var updatedAt = $state(pcbBoard.updated_at);
    var firstName = pcbBoard.first_name;
    var lastName = pcbBoard.first_name;

    var bom = new BillOfMaterials(pcbBoard.content);
    
    var design;
    if (browser) {
        design = new PcbDesign(pcbBoard.content);
    }
    // console.log("bom:", bom);
    console.log(design);

    // console.log(bom.components)
    // console.log(bom.grouped)

    var renderer = null;
    var layerOrder = $state([]);
    var selectedNetName = $state(null);


    onMount(
        () => {

            var frontCanvas = document.getElementById("front-content");
            var frontHlCanvas = document.getElementById("front-hl");
            var frontCanvasParent = frontCanvas.parentElement;

            var backCanvas = document.getElementById("back-content");
            var backHlCanvas = document.getElementById("back-hl");
            var backCanvasParent = backCanvas.parentElement;

            var sidesConfigs = {
                front: {
                    flipped: false,       
                    rotation: 0,
                    panX: 0,
                    panY: 0,
                    zoom: 1,
                    scale: 1,
                    sideParent: frontCanvasParent,     
                    contexts: {
                        content: frontCanvas.getContext("2d"),
                        highlight: frontHlCanvas.getContext("2d")
                    }
                },
                back: {
                    flipped: true,       
                    rotation: 0,
                    panX: 0,
                    panY: 0,
                    zoom: 1,
                    scale: 1,
                    sideParent: backCanvasParent,     
                    contexts: {
                        content: backCanvas.getContext("2d"),
                        highlight: backHlCanvas.getContext("2d")
                    }
                }
            };

            $effect(() => {
                sidesConfigs.front.rotation = rotation;
                sidesConfigs.back.rotation = rotation;
                renderer?.render(false);
            });

            renderer = new PcbRenderer(sidesConfigs, design);
            renderer.onSelect = (refDes) => {
                document.querySelectorAll('tbody tr.tr-hover').forEach(row => {
                    row.classList.remove('tr-hover');
                });

                if (viewMode === 0) {
                    const row = document.getElementById(`row-${refDes}`);
                    if (row) {
                        row.classList.add('tr-hover');
                        row.scrollIntoView({ behavior: 'smooth', block: 'center' });
                    }
                } else if (viewMode === 1) {
                    const group = bom.grouped.find(g => g.refs.some(r => r.refDes === refDes));
                    
                    if (group) {
                        const groupId = `row-group-${group.patternName}-${group.compValue}`;
                        const row = document.getElementById(groupId);
                        
                        if (row) {
                            row.focus();
                            row.classList.add('tr-hover');
                            row.scrollIntoView({ behavior: 'smooth', block: 'center' });
                        }
                    }
                }
            }


            if (pcbSave) {
                console.log('Applying saved settings:', pcbSave);
                
                if (pcbSave.ref_defs?.length) {
                    bom.components.forEach(comp => {
                        if (pcbSave.ref_defs.includes(comp.refDes)) {
                            comp.installed = true;
                            renderer.installed.add(comp.refDes);
                        }
                    });
                }
                
                if (pcbSave.common_side_order?.length) {
                    const newFrontOrder = pcbSave.common_side_order.map(meta => meta.layer_ref.toString());
                    frontLayers = newFrontOrder;
                    
                    if (renderer?.sidesConfiguration?.front) {
                        renderer.sidesConfiguration.front.order = newFrontOrder;
                    }
                    
                    pcbSave.common_side_order.forEach(meta => {
                        const layerRef = meta.layer_ref.toString();
                        if (renderer?.sidesConfiguration?.front?.visibility) {
                            renderer.sidesConfiguration.front.visibility[layerRef] = meta.visible;
                        }
                    });
                }
                
                if (pcbSave.mirror_side_order?.length) {
                    const newBackOrder = pcbSave.mirror_side_order.map(meta => meta.layer_ref.toString());
                    backLayers = newBackOrder;
                    
                    if (renderer?.sidesConfiguration?.back) {
                        renderer.sidesConfiguration.back.order = newBackOrder;
                    }
                    
                    pcbSave.mirror_side_order.forEach(meta => {
                        const layerRef = meta.layer_ref.toString();
                        if (renderer?.sidesConfiguration?.back?.visibility) {
                            renderer.sidesConfiguration.back.visibility[layerRef] = meta.visible;
                        }
                    });
                }
                
                if (pcbSave.rotation !== undefined) {
                    rotation = pcbSave.rotation;
                }
                
                renderer?.render();
                console.log('Saved settings applied');
            }


            if (bom?.components?.length) {
                bom.components.forEach(comp => {
                    if (comp.installed) {
                        renderer.installed.add(comp.refDes);
                    }
                });
                renderer.redrawHighlightLayer(sidesConfigs.front);
                renderer.redrawHighlightLayer(sidesConfigs.back);
            }

            var rafId = null;

            layerOrder = [...renderer.pcbData.order];
            frontLayers = [...renderer.sidesConfiguration.front.order];
            backLayers = [...renderer.sidesConfiguration.back.order];
            renderer.onSelectNet = (netName) => {
                selectedNetName = netName; 
            };

            Split(
                ["#table-container", "#canvases-boards-container"],
                {
                    direction: 'horizontal', 
                    sizes: [50, 50],       
                    gutterSize: 6,
                    minSize: 0,
                    snapOffset: 30,
                }
            );

            Split(
                ["#front-canvas", "#back-canvas"],
                {
                    direction: 'vertical', 
                    sizes: [50, 50],       
                    gutterSize: 6,
                    snapOffset: 30,
                    minSize: 0,
                }
            );
            
            renderer.render();
            renderer.observeCanvases();
        }
    );

    var handleRowKeydown = async (e, component, index) => {
        const rows = document.querySelectorAll('#table-container tbody tr[tabindex="0"]');
    
        switch (e.key) {
            case 'ArrowDown':
                e.preventDefault();
                rows[index + 1]?.focus();
                break;
                
            case 'ArrowUp':
                e.preventDefault();
                rows[index - 1]?.focus();
                break;
                
            case 'Enter':
            case ' ':  
                if (isGuest) {
                    e.preventDefault();
                    e.stopPropagation();
                    return;
                }
                e.preventDefault();
                component.installed = !component.installed;
                renderer?.syncInstalled?.(component.refDes, component.installed);
                
                await tick();
                
                const freshRows = document.querySelectorAll('#table-container tbody tr[tabindex="0"]');
                const nextRow = freshRows[index + 1];

                if (nextRow) {
                    nextRow.focus();
                    const nextComponent = bom.components[index + 1];
                    if (nextComponent) {
                        renderer?.highlighted.clear();
                        renderer?.highlightComp(nextComponent.refDes);
                    }
                }
                break;
        }
    };

    var handleGroupKeydown = async (e, group, index) => {
        switch (e.key) {
            case 'ArrowDown':
                e.preventDefault();
                document.querySelectorAll('#table-container tbody tr[tabindex="0"]')[index + 1]?.focus();
                break;
            case 'ArrowUp':
                e.preventDefault();
                document.querySelectorAll('#table-container tbody tr[tabindex="0"]')[index - 1]?.focus();
                break;
            case 'Enter':
            case ' ':
                if (isGuest) {
                    e.preventDefault();
                    e.stopPropagation();
                    return;
                }
                e.preventDefault();
                const newVal = !group.isInstalled;
                bom.toggleGroup(group, newVal);
                renderer?.syncGroupInstalled?.(group.refs, newVal);
                
                await tick();
                
                const nextRow = document.querySelectorAll('#table-container tbody tr[tabindex="0"]')[index + 1];
                if (nextRow) {
                    nextRow.focus();
                    const nextGroup = bom.grouped[index + 1];
                    if (nextGroup) {
                        renderer?.highlighted.clear();
                        renderer?.highlightGroup(nextGroup.refs);
                    }
                }
                break;
        }
    };

    var focusTable = () => {
        tick().then(() => {
            const firstRow = document.querySelector('#table-container tbody tr[tabindex="0"]');
            firstRow?.focus();
        }); 
    };

    var handleGlobalKeydown = ((e) => {
        const triggerKeys = ['q'];
        
        if (e.target.closest('table') || e.target.tagName === 'SELECT' || e.target.tagName === 'INPUT') {
            return;
        }

        if (triggerKeys.includes(e.key)) {
            e.preventDefault();
            focusTable();
        }
    });

    var layersDropdownOpen = $state({
        front: false,
        back: false
    });

    var frontLayers = $state([]);
    var backLayers = $state([]);

    var toggleLayerVisibility = ((side, layerRef, visible) => {
        if (!renderer?.sidesConfiguration?.[side]) return;
        
        if (renderer.sidesConfiguration[side].visibility) {
            renderer.sidesConfiguration[side].visibility[layerRef] = visible;
        }

        var layer = renderer?.pcbData?.layers?.[layerRef];
        if (layer) {
            layer.visible = visible;
        }

        renderer?.render();
        renderer = renderer; 
    });

    var handleLayerReorder = ((side, fromIndex, toIndex) => {
        if (fromIndex === toIndex || !renderer?.sidesConfiguration?.[side]) return;
        
        const orderArray = renderer.sidesConfiguration[side].order;
        if (!orderArray) return;

        const [item] = orderArray.splice(fromIndex, 1);
        orderArray.splice(toIndex, 0, item);
        
        renderer.sidesConfiguration[side].order = [...orderArray];
        if (side === 'front') {
            frontLayers = [...orderArray];
        } else {
            backLayers = [...orderArray];
        }
        renderer?.render();
        renderer = renderer; 
    });

    var copyPublicLink = (() => {
        var url = `${window.location.origin}/public/boards/${publicToken}`;
        navigator.clipboard.writeText(url)
            .then(() => {
                const btn = document.getElementById('copy-link-btn');
                if (btn) {
                    const originalHTML = btn.innerHTML;
                    btn.disabled = true
                    btn.innerHTML = `<img src="${done}" alt="done"/> Скопировано!`;
                    setTimeout(() => { 
                        btn.innerHTML = originalHTML; 
                        btn.disabled = false;
                    }, 1500);
                }
            })
            .catch(err => {
                    console.error('Не удалось скопировать ссылку:', err);
            });
    });

    var saveBoardSettings = (() => {
        if (isGuest || isSaving) {
            return;
        }

        isSaving = true;
        
        var btn = document.getElementById('save-btn');
        let originalHTML = ''; 

        if (btn) {
            originalHTML = btn.innerHTML; 
            btn.disabled = true;
            btn.innerHTML = `<img src="${loading}" alt="loading"/> Сохранение...`;
            btn.style.cursor = 'wait';
        }

        const refDefs = bom.components
            .filter(comp => comp.installed)
            .map(comp => comp.refDes);

        const commonSideOrder = (frontLayers || renderer?.sidesConfiguration?.front?.order || [])
            .map(layerRef => ({
                layer_ref: parseInt(layerRef),  
                visible: renderer?.sidesConfiguration?.front?.visibility?.[layerRef] !== false
            }));
        
        const mirrorSideOrder = (backLayers || renderer?.sidesConfiguration?.back?.order || [])
            .map(layerRef => ({
                layer_ref: parseInt(layerRef),
                visible: renderer?.sidesConfiguration?.back?.visibility?.[layerRef] !== false
            }));
        
        const rot = parseInt(rotation) || 0;
        
        const saveData = {
            refDefs: refDefs,
            commonSideOrder: commonSideOrder,
            mirrorSideOrder: mirrorSideOrder,
            rotation: rot 
        };
        
        updateBoard(saveData, pcbBoard.id)
            .then((info) => {
                updatedAt = info.updatedAt;
                if (btn) {
                    btn.innerHTML = `<img src="${done}" alt="done"/> Сохранено!`;
                    
                    setTimeout(() => { 
                        btn.innerHTML = originalHTML; 
                        btn.disabled = false;
                        btn.style.cursor = '';
                    }, 2000);
                }
            })
            .catch((error) => {
                if (btn) {
                    btn.innerHTML = `<span style="color:#dc2626">Ошибка</span>`;
                    
                    setTimeout(() => { 
                        btn.innerHTML = originalHTML; 
                        btn.disabled = false;
                        btn.style.cursor = '';
                    }, 2000);
                }
                
                const errorMsg = error?.message || error?.error || 'Неизвестная ошибка';
                console.error(errorMsg);
                alert(`Не удалось сохранить: ${errorMsg}`);
            })
            .finally(() => {
                isSaving = false;
            });
    });

    var isSaving = $state(false);
    var handleSaveHotkey = ((e) => {
        if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
            e.preventDefault(); 
            
            if (isGuest || isSaving) {
                if (!isGuest && isSaving) {
                    const btn = document.getElementById('save-btn');
                    if (btn) {
                        btn.style.transform = 'scale(0.98)';
                        setTimeout(() => { btn.style.transform = ''; }, 100);
                    }
                }
                return;
            }
            
            saveBoardSettings();
        }
    });
    

</script>

<svelte:window 
    onkeydown={(e) => (handleGlobalKeydown(e), handleSaveHotkey(e))}
/>

<div id="content">
    <header>
        <button id="logo-button" onclick={() => goto("/boards")}>
            <img id="logo" src={logo} alt="logo"/>
        </button>
        <div id="title">
            <div id="creator">
                <span>{pcbBoard.title}</span>
                {#if isGuest}
                    <span>/ от { firstName + " " + lastName }</span>
                {/if}
            </div>
            <span id="added-date">Обновлено: {new Date(updatedAt).toLocaleString("ru-Ru")}</span>
        </div>
        <div id="select-table-view">
            Таблица:
            <select bind:value={viewMode}>
                <option value={0}>По компонентам</option>
                <option value={1}>Сгруппированный</option>
            </select> 
        </div>
        <div id="select-board-angle">
           Поворот ({rotation}&#176;) 
           <input type="range" min="0", max="180" bind:value={rotation}/>
        </div>
        {#if !isGuest}
            <button id="save-btn" onclick={saveBoardSettings} class="icon-button save-btn" title="Сохранить изменения">
                <img src={save} alt="save"/>
                <span>Сохранить прогресс</span>
            </button>
            <button id="copy-link-btn" class="icon-button" onclick={copyPublicLink} title="Копировать публичную ссылку">
                <img src={copy} alt="copy"/>
                <span>Cсылка на схему</span>
            </button>
        {/if}
    </header>

    <main>
        <div id="table-container">
            <table>
                <thead>
                    <tr>
                        <th>
                            Экземпляр 
                            <div class="resize-handle" {@attach resizable}></div>
                        </th>
                        <th>
                            Значение
                            <div class="resize-handle" {@attach resizable}></div>
                        </th>
                        <th>
                            Посадочное место
                            <div class="resize-handle" {@attach resizable}></div>
                        </th>
                        {#if viewMode == 1}
                            <th>
                                Количество
                                <div class="resize-handle" {@attach resizable}></div>
                            </th>
                        {/if}
                        <th>
                            Установлен
                            <div class="resize-handle" {@attach resizable}></div>
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {#if viewMode === 0}
                        {#each bom.components as component, index (component.refDes)}
                            <tr { @attach (node) => draggable(node, { handleReorder })}
                                { @attach (node) => hoverable(node, { item: component.refDes, withCallbackFn: (id) => renderer?.highlightComp(id)})}
                                id="row-{component.refDes}"
                                tabindex="0"
                                onkeydown={(e) => handleRowKeydown(e, component, index)}
                                onfocus={() => {
                                    renderer?.highlighted.clear(); 
                                    renderer?.highlightComp(component.refDes);
                                }}
                            >
                                <td class="refdes-cell">{component.refDes}</td>
                                <td>{component.compValue}</td>
                                <td>{component.patternName}</td>
                                <td>
                                    <input 
                                        type="checkbox"
                                        bind:checked={component.installed}
                                        onchange={() => renderer?.syncInstalled(component.refDes, component.installed)}
                                        tabindex="-1"
                                        autocomplete="off"
                                        disabled={isGuest}
                                    />
                                </td>
                            </tr>
                        {/each}
                    {:else}
                        {#each bom.grouped as group, index (group.patternName + group.compValue)}
                            <tr { @attach (node) => draggable(node, { handleReorder })}
                                { @attach (node) => hoverable(node, { item: group.refs, withCallbackFn: (rs) => renderer?.highlightGroup(rs)})}
                                id="row-group-{group.patternName}-{group.compValue}"
                                tabindex="0"
                                onkeydown={(e) => handleGroupKeydown(e, group, index)}
                                onfocus={() => {
                                    renderer?.highlighted.clear(); 
                                    renderer?.highlightGroup(group.refs);
                                }}
                            >
                                <td class="refdes-cell">{group.refs.map(el => el.refDes).join(", ")}</td>
                                <td>{group.compValue}</td>
                                <td>{group.patternName}</td>
                                <td>{group.refs.length}</td>
                                <td>
                                    <input 
                                        type="checkbox"
                                        checked={group.isInstalled}
                                        indeterminate={group.isIndeterminate}
                                        autocomplete="off"
                                        onchange={(e) => {
                                            const val = e.target.checked;
                                            bom.toggleGroup(group, val);
                                            renderer?.syncGroupInstalled(group.refs, val);
                                        }}
                                        tabindex="-1"
                                        disabled={isGuest}
                                    />
                                </td>
                            </tr>
                        {/each}
                    {/if}
                </tbody>
            </table>
        </div>

        <div id="canvases-boards-container">
            <div id="front-canvas" class="board-canvas">
                <div class="canvas-layers">
                    <canvas id="front-content"></canvas>
                    <canvas id="front-hl"></canvas>
                </div>

                <div class="canvas-layer-controls dropdown">
                    <button 
                        class="dropdown-toggle" 
                        onclick={() => layersDropdownOpen.front = !layersDropdownOpen.front}
                        aria-expanded={layersDropdownOpen.front}
                    >
                        <span>Слои</span>
                        <span class="dropdown-arrow" class:open={layersDropdownOpen.front}>&#8595</span>
                    </button>
                    
                    {#if layersDropdownOpen.front}
                        <div class="dropdown-content" transition:slide={{ duration: 200 }}>
                            <div class="layer-list">
                                {#each frontLayers as layerRef (layerRef)}
                                    {@const layer = renderer?.pcbData?.layers?.[layerRef]}
                                    {@const isVisible = renderer?.sidesConfiguration['front']?.visibility[layerRef] !== false}
                                    <div 
                                        class="layer-item" 
                                        class:invisible={!isVisible}
                                        {@attach (node) => draggable(node, { handleReorder: (oldIdx, newIdx) => handleLayerReorder('front', oldIdx, newIdx) })}
                                    >
                                        <span class="layer-name" title="{layer?.name || `Layer ${layerRef}`}">
                                            {layer?.name || `Layer ${layerRef}`}
                                        </span>
                                        <div class="layer-actions">
                                            <label class="visibility-toggle">
                                                <input 
                                                    type="checkbox"
                                                    checked={isVisible}
                                                    onchange={(e) => toggleLayerVisibility('front', layerRef, e.target.checked)}
                                                    class="visible-checkbox"
                                                />
                                                <img src={eye} alt="visible" class="icon-eye" class:hidden={!isVisible}/>
                                                <img src={eyeOff} alt="hidden" class="icon-eye-off" class:hidden={isVisible}/>
                                            </label>
                                        </div>
                                    </div>
                                {/each}
                            </div>
                        </div>
                    {/if}
                </div>

                <div class="net-hud-badge">
                    <span style="align-self: center;">Обычный</span>
                    {#if selectedNetName}
                        <div>
                            <span class="label">Сеть:</span>
                            <span class="value">{selectedNetName}</span>
                        </div>
                    {/if}
                </div>
            </div>

            <div id="back-canvas" class="board-canvas">
                <div class="canvas-layers">
                    <canvas id="back-content"></canvas>
                    <canvas id="back-hl"></canvas>
                </div>

                <div class="canvas-layer-controls dropdown">
                    <button 
                        class="dropdown-toggle" 
                        onclick={() => layersDropdownOpen.back = !layersDropdownOpen.back}
                        aria-expanded={layersDropdownOpen.back}
                    >
                        <span>Слои</span>
                        <span class="dropdown-arrow" class:open={layersDropdownOpen.back}>&#8595</span>
                    </button>
                    
                    {#if layersDropdownOpen.back}
                        <div class="dropdown-content" transition:slide={{ duration: 200 }}>
                            <div class="layer-list">
                                {#each backLayers as layerRef (layerRef)}
                                    {@const layer = renderer?.pcbData?.layers?.[layerRef]}
                                    {@const isVisible = renderer?.sidesConfiguration['back']?.visibility[layerRef] !== false}
                                    <div 
                                        class="layer-item" 
                                        class:invisible={!isVisible}
                                        {@attach (node) => draggable(node, { handleReorder: (oldIdx, newIdx) => handleLayerReorder('back', oldIdx, newIdx) })}
                                    >
                                        <span class="layer-name" title="{layer?.name || `Layer ${layerRef}`}">
                                            {layer?.name || `Layer ${layerRef}`}
                                        </span>
                                        <div class="layer-actions">
                                            <label class="visibility-toggle">
                                                <input 
                                                    type="checkbox"
                                                    checked={isVisible}
                                                    onchange={(e) => toggleLayerVisibility('back', layerRef, e.target.checked)}
                                                    class="visible-checkbox"
                                                />
                                                <img src={eye} alt="visible" class="icon-eye" class:hidden={!isVisible}/>
                                                <img src={eyeOff} alt="hidden" class="icon-eye-off" class:hidden={isVisible}/>
                                            </label>
                                        </div>
                                    </div>
                                {/each}
                            </div>
                        </div>
                    {/if}
                </div>

                <div class="net-hud-badge">
                    <span style="align-self: center;">Зеркальный</span>
                    {#if selectedNetName}
                        <div>
                            <span class="label">Сеть:</span>
                            <span class="value">{selectedNetName}</span>
                        </div>
                    {/if}
                </div>
            </div>
        </div>
    </main>
</div>

<style>


    input[type="checkbox"]:disabled:checked,
    input[type="checkbox"]:disabled:indeterminate {
        accent-color: #83D03F !important;
    }

    input[type="checkbox"]:disabled:not(:checked):not(:indeterminate) {
        display: none !important;
    }

    :global(tbody tr:has(input[type="checkbox"]:disabled:checked)) {
        background-color: rgba(131, 208, 63, 0.06);
    }

    :global(td:has(input[type="checkbox"]:disabled:not(:checked):not(:indeterminate))) {
        padding: 6px 8px; 
    }

    #copy-link-btn, #save-btn {
        display: flex;
        align-items: center;
        gap: 0.4rem;
        padding: 0.5rem 1rem;
        background-color: #f3f4f6;
        border: 1px solid #d1d5db;
        border-radius: 0.5rem;
        cursor: pointer;
        font-size: 0.9rem;
        color: #374151;
        transition: all 0.2s;
        white-space: nowrap;
    }

    #copy-link-btn:hover, #save-btn:hover {
        background-color: #83D03F;
        border-color: #83D03F;
        color: white;
    }

    #copy-link-btn:active , #save-btn:active {
        transform: translateY(1px);
    }

    :global(#copy-link-btn:hover img, #save-btn:hover img) {
        color: white;
    }

    :global(#copy-link-btn img, #save-btn img) {
        width: 1rem;
        height: 1rem;
        color: #374151;
        flex-shrink: 0;
    }

    .canvas-layer-controls {
        position: absolute;
        top: -100%;
        left: 0;
        margin: 0.3rem;
        z-index: 100; 
    }

    .canvas-layer-controls .dropdown-content {
        left: 0;
        right: auto;
        z-index: 100;
    }

    .net-hud-badge {
        position: absolute;
        top: 0.1rem;
        display: flex;
        flex-direction: column;
        right: 0.1rem;
        backdrop-filter: blur(4px);
        font-family: monospace;
        pointer-events: none; 

        .label { 
            color: #888; 
            margin-right: 6px; 
        }
        .value { 
            color: #000000; 
            font-weight: bold; 
        }
    }

    :global(.layer-item[draggable="true"]) {
        cursor: grab;
        user-select: none;
        transition: background-color 0.15s, opacity 0.15s;
    }

    :global(.layer-item[draggable="true"]:active) {
        cursor: grabbing;
    }

    :global(.layer-item.dragging) {
        opacity: 0.5;
        background-color: #f0f0f0 !important;
        transform: scale(1.02);
        z-index: 10;
    }

    :global(.layer-item.drag-over) {
        border-top: 3px solid #83D03F;
        transition: border-top 0.1s;
    }

    :global(.layer-item.drag-over:hover) {
        background-color: transparent !important;
    }

    .visible-checkbox {
        display: none;
        cursor: pointer;
    }

    .invisible {
        background: whitesmoke;
    }

    .icon-eye,
    .icon-eye-off {
        display: none;
        width: 1.2rem;
        height: 1.2rem;
        padding: 0.2rem;
    }

    .icon-eye:hover {
        background-color: #d97d5e;
        transition: all 0.2s;
        border-radius: 0.2rem;
    }

    .icon-eye-off:hover {
        background-color: #d97d5e;
        transition: all 0.2s;
        border-radius: 0.2rem;
    }

    .visible-checkbox:not(:checked) ~ .icon-eye-off {
        display: inline-block;
    }

    .visible-checkbox:checked ~ .icon-eye {
        display: inline-block;
    }

    .dropdown {
        position: relative; 
        display: inline-block; 
    }

    .dropdown-toggle {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 8px;
        padding: 6px 12px;
        background-color: #f3f4f6;
        border: 1px solid #d1d5db;
        border-radius: 4px;
        cursor: pointer;
        font-size: 14px;
    }

    .dropdown-arrow {
        transition: transform 0.2s ease;
        font-size: 10px;
    }
    .dropdown-arrow.open {
        transform: rotate(180deg);
    }

    .dropdown-content {
        position: absolute; 
        top: 100%; 
        right: 0; 
        margin-top: 4px; 
        
        background-color: rgb(255, 255, 255);
        border: 1px solid #ccc;
        border-radius: 0.5rem;
        box-shadow: 0 8px 16px rgba(0, 0, 0, 0.15); 
        z-index: 1000; 
        
        min-width: 260px;
        max-height: 400px; 
        overflow-y: auto;
    }

    .layer-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.5rem 1rem;
        border-bottom: 0.1rem solid #eee;
    }

    .layer-item:hover {
        background-color: #fb9c7d;
    }

    .layer-item:last-child {
        border-bottom: none;
    }

    .layer-item.hidden {
        opacity: 0.5; 
    }

    .layer-name {
        flex-grow: 1;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        margin-right: 12px;
    }

    .layer-actions {
        display: flex;
        gap: 0.2rem;
        align-items: center;
    }

    .layer-actions label {
        padding: 2px 6px;
        cursor: pointer;
    }

    input[type="checkbox"]:indeterminate {
        accent-color: #83D03F;  
    }

    :global(tr:has(input[type="checkbox"]:checked)) {
        accent-color: #83D03F;  
    }

    :global(tr:has(input[type="checkbox"]:indeterminate)) {
        background-color: rgba(255, 165, 0, 0.08);
    }

    :global(tbody tr[draggable="true"]) {
        user-select: none;
    }

    :global(tbody tr.dragging) {
        opacity: 0.5;
        background-color: #f5f5f5 !important;
        transition: opacity 0.15s;
    }

    :global(tbody tr.drag-over) {
        border-top: 3px solid #83D03F;
        transition: border-top 0.1s;
    }

    :global(body), :global(html) {
        margin: 0;
        padding: 0;
        height: 100%;
        overflow: hidden; 
    }

    #content {
        display: flex;
        flex-direction: column;
        height: 100vh;
    }

    #select-board-angle {
        display: flex;
        flex-direction: column;
        input {
            accent-color: #83D03F;  
        }
    }

    :global(tbody tr:focus-visible) {
        background-color: #fb9c7d !important;
        outline: none; 
    }

    :global(tbody tr:focus-visible:hover) {
        background-color: #ffc8b5 !important;
        outline: none; 
    }
    
    header {
        background-color: white;
        margin-bottom: 0.5rem;
        border-radius: 0 0 1.5rem 1.5rem;
        display: flex;
        align-items: center;
        box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
        /* border: 1px solid rgba(0, 0, 0, 0.1); */
        border: 2px solid #fb9c7d; 
        padding-left: 2rem;
        padding-right: 2rem;
        gap: 1rem;
    }


    main {
        display: flex;
        flex-direction: row;
        width: 100%;
        overflow: hidden; 
        min-height: 0;   
    }


    #canvases-boards-container {
        height: 100%;
        display: flex;
        flex-direction: column;
        overflow: visible ;
        /* border: 2px solid black; */
    }

    .board-canvas {
        position: relative;
        overflow: visible ;
    }

    .canvas-layers {
        position: relative;
        cursor: crosshair;
        width: 100%;
        height: 100%;
        overflow: hidden;

        canvas {
            position: absolute;
            top: 0px;
            left: 0px;
            width: 100%;
            height: 100%;
            display: block;
            border: 2px solid #8080807d; 
        }
    }

    td, th {
        word-wrap: break-word;
        word-break: break-word;
        white-space: normal;
        overflow-wrap: break-word;
    }


    .resize-handle {
        position: absolute;
        right: 0;
        top: 0;
        bottom: 0;
        width: 5px;
        cursor: col-resize;
        background: transparent;
        transition: background 0.2s;
        z-index: 20;
    }

    .resize-handle:hover { 
        background: #83D03F;
    }

    #select-table-view {
        display: flex;
        flex-direction: column;
    }

    #title {
        display: flex;
        flex-direction: column;
    }


    #logo-button {
        background: transparent;
        border: none;
        padding: 0;
        margin: 0;
        cursor: pointer;
        display: flex; 
    }

    #logo {
        width: 12rem; 
        display: block;
    }
    #added-date {
        font-size: 10pt;
        color: gray;
    }

    #table-container {
        overflow-y: auto;
        height: 100%;
        text-align: center;
    }

    table {
        width: 100%;
        font-family: sans-serif;
        border-collapse: collapse;
        resize: both;
        table-layout: fixed;
        overflow: auto;
    }
    th {
        position: sticky;
        top: 0;
        /* border: 2px solid #fb9c7d;  */
        background: #eee;
        padding: 8px;
    }
    td {
        border: 2px solid whitesmoke; 
        padding: 6px 8px;
        border-bottom: 1px solid #ddd;
    }

    :global(.tr-hover){
        background-color: #fb9c7d !important;
        cursor: pointer;
    }

    .refdes-cell {
        font-weight: bold;
        color: #2c3e50;
    }
    .footprint-label {
        font-family: monospace;
        font-size: 0.9em;
    }

</style>