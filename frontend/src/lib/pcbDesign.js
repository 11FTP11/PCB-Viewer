// @ts-nocheck

export class PcbDesign {

        constructor(data) {
            this.#defineConvertUnitsFn(data.asciiHeader.fileUnits);
            this.#defineLayers(data.pcbDesign.layerContents,  data.pcbDesign.layerDefs);
            this.#defineMultilayer(data.pcbDesign.multiLayer, data.library);
        }

        mm2px(value) {
            const scale = 96 / 25.4;
            return value * scale;
        }

        #bakeShape(sh) {
            var path = new Path2D();
            switch (sh.type) {
                case "Ellipse":
                    path.moveTo(sh.rx, 0);
                    path.ellipse(0, 0, sh.rx, sh.ry, 0, 0, Math.PI * 2);
                    break;

                case "Rect":
                    path.rect(-sh.w / 2, -sh.h / 2, sh.w, sh.h);
                    break;

                case "Thrm4_45":
                case "Thrm4_90":
                case "Thrm2_90": {
                    const outR = sh.outRad;
                    const inR = sh.inRad;
                    const sw = sh.spokeW;

                    const gapAngle = sw / (2 * outR); 
                    
                    const numSegments = sh.type === "Thrm2_90" ? 2 : 4;
                    const offset = sh.type === "Thrm4_45" ? Math.PI / 4 : 0;
                    const step = (Math.PI * 2) / numSegments;

                    for (let i = 0; i < numSegments; i++) {
                        const startAngle = (i * step) + offset + gapAngle;
                        const endAngle = ((i + 1) * step) + offset - gapAngle;

                        if (startAngle >= endAngle) continue;

                        path.moveTo(
                            outR * Math.cos(startAngle), 
                            outR * Math.sin(startAngle)
                        );

                        path.arc(0, 0, outR, startAngle, endAngle);

                        path.lineTo(
                            inR * Math.cos(endAngle), 
                            inR * Math.sin(endAngle)
                        );

                        path.arc(0, 0, inR, endAngle, startAngle, true);
                        
                        path.closePath();
                    }
                    break;
                }

                default:
                    console.error(`Unknown shape type: ${sh.type}`);
                    const fallbackRad = sh.rad || sh.rx || 1;
                    path.moveTo(fallbackRad, 0);
                    path.arc(0, 0, fallbackRad, 0, Math.PI * 2);
                    break;
            }

            return path;
        }
    
        #pickShape(sh) {
            const shapeType = sh.padShapeType || sh.viaShapeType;
            const conv = (val) => this.unitsConvertFn(val || 0);

            switch (shapeType) {
                case "Ellipse":
                    return {
                        type: "Ellipse",
                        rx: conv(sh.shapeWidth || sh.outsideDiam) / 2,
                        ry: conv(sh.shapeHeight || sh.outsideDiam) / 2
                    };

                case "Rect":
                    return {
                        type: "Rect",
                        w: conv(sh.shapeWidth),
                        h: conv(sh.shapeHeight)
                    };

                case "Thrm4_45":
                    return {
                        type: "Thrm4_45",
                        outRad: conv(sh.outsideDiam) / 2,
                        inRad: conv(sh.insideDiam) / 2,
                        spokeW: conv(sh.spokeWidth)
                    };

                case "Thrm4_90":
                    return {
                        type: "Thrm4_90",
                        outRad: conv(sh.outsideDiam) / 2,
                        inRad: conv(sh.insideDiam) / 2,
                        spokeW: conv(sh.spokeWidth)
                    };

                case "Thrm2_90":
                    return {
                        type: "Thrm2_90",
                        outRad: conv(sh.outsideDiam) / 2,
                        inRad: conv(sh.insideDiam) / 2,
                        spokeW: conv(sh.spokeWidth)
                    };

                default:
                    return {
                        type: "Unknown",
                        rad: conv(sh.shapeWidth || 0) / 2
                    };
            }
        }

        #getTextStyles(textStyleDef) {
            return textStyleDef.reduce((acc, style) => {
                    const font = style.font.find(f => f.fontType === "TrueType") || style.font[0];

                    acc[style.name] = {
                        name: style.name,
                        height: this.unitsConvertFn(font.fontHeight || 0),
                        strokeWidth: this.unitsConvertFn(font.strokeWidth || 0),
                        family: font.fontFamily || "Arial",
                        type: font.fontType,
                        allowTTF: style.textStyleAllowTType || false
                    };
                    return acc;
                }, 
                {}
            );
        }

        #prepareSilk(layerContents, textStyles, patternRef) {
            layerContents.forEach((layer) => {
                const layerNum = layer.layerRef?.layerNumRef;

                if (layerNum === undefined) 
                    return;

                const currentLayer = this.#ensureLayer(layerNum);

                if (!currentLayer.patterns[patternRef]) {
                    currentLayer.patterns[patternRef] = {
                        footPrint: null,
                        lines: null,
                        arcs: null,
                        holes: null,
                        labels: null,
                        keepOuts: null,
                        flipped: false
                    };
                }

                const target = currentLayer.patterns[patternRef];

                layer.lines?.forEach(line => {
                    const width = this.unitsConvertFn(line.width);
                    const linePath = new Path2D();
                    linePath.moveTo(this.unitsConvertFn(line.start[0]), this.unitsConvertFn(line.start[1]));
                    linePath.lineTo(this.unitsConvertFn(line.end[0]), this.unitsConvertFn(line.end[1]));

                    (target.lines = target.lines || Array()).push({
                        width: width,
                        path: linePath 
                    });
                });

                layer.arcs?.forEach(arc => {
                    const width = this.unitsConvertFn(arc.width);
                    const arcPath = new Path2D();
                    this.#addArcToPath(arcPath, arc);

                    (target.arcs = target.arcs || Array()).push({
                        width: width,
                        path: arcPath
                    });
                });

                layer.polyKeepOuts?.forEach(poly => {
                    const points = poly.pcbPoly?.points;
                    if (points?.length > 0) {
                        if (!target.keepOuts) {
                            target.keepOuts = new Path2D();
                        }
                        target.keepOuts.moveTo(this.unitsConvertFn(points[0][0]), this.unitsConvertFn(points[0][1]));
                        for (let i = 1; i < points.length; i++) {
                            target.keepOuts.lineTo(this.unitsConvertFn(points[i][0]), this.unitsConvertFn(points[i][1]));
                        }
                        target.keepOuts.closePath(); 
                    }
                });


                layer.texts?.forEach(
                    (text) => {
                        const x = this.unitsConvertFn(text.pt?.[0] || 0);
                        const y = this.unitsConvertFn(text.pt?.[1] || 0);
                        const { label, name, textStyleRef } = text;
                        const textStyle = textStyles[textStyleRef];
                        if (!target.labels) {
                            target.labels = {};
                        }

                        target.labels[label] = {
                            text: "",
                            x: x,
                            y: y,
                            font: `${textStyle.height}px ${textStyle.family}`, 
                            styleRef: textStyleRef,
                            flipped: text.isFlipped || false
                        }
                    }
                );

                if (!this.multilayer.patterns[patternRef]) 
                    this.multilayer.patterns[patternRef] = new Set();

                this.multilayer.patterns[patternRef].add(layerNum);
            });
        }


        #addArcToPath(path, arc) {

            const cx = this.unitsConvertFn(arc.startPoint[0]);
            const cy = this.unitsConvertFn(arc.startPoint[1]);
            
            const x1 = this.unitsConvertFn(arc.midPoint[0]);
            const y1 = this.unitsConvertFn(arc.midPoint[1]);
            
            const x2 = this.unitsConvertFn(arc.endPoint[0]);
            const y2 = this.unitsConvertFn(arc.endPoint[1]);

            const radius = Math.hypot(x1 - cx, y1 - cy);
            
            const eps = 0.001;
            if (Math.hypot(x2 - x1, y2 - y1) < eps) {
                path.moveTo(cx + radius, cy);
                path.arc(cx, cy, radius, 0, 2 * Math.PI);
                return;
            }



            const startAngle = Math.atan2(y1 - cy, x1 - cx);
            const endAngle = Math.atan2(y2 - cy, x2 - cx);
            const isccw = (x1 - cx)*(y2 - cy) - (y1 - cy)*(x2 - cx) > 0;

            path.moveTo(x1, y1);
            path.arc(cx, cy, radius, startAngle, endAngle, !isccw);
        }


        #getPadsStyles(padStyleDef) {
            return padStyleDef.reduce(
                (acc, style) => { 
                    const shapes = style.padShape.reduce(
                        (shapeAcc, sh) => { 
                            const shapeGeometry = this.#bakeShape(
                                this.#pickShape(sh)
                            );

                            if (sh.layerRef?.layerNumRef) {
                                shapeAcc.byLayerRef[sh.layerRef.layerNumRef] = shapeGeometry;
                            } else if (sh.layerRef?.layerType) {
                                shapeAcc.byLayerType[sh.layerRef.layerType] = shapeGeometry;
                            }

                            return shapeAcc; 
                        },
                        { byLayerRef: {}, byLayerType: {} }
                    );

                    acc[style.name] = {
                        holeRad: this.unitsConvertFn(style.holeDiam) / 2,
                        shapes: shapes,
                        refRange: { 
                            startRange: style.StartRange, 
                            endRange: style.EndRange 
                        } 
                    };

                    return acc; 
                },
                {}
            );
        }

        #preparePatterns(patternDefExtended, padsStyles, textStyles) {
            const flattenedPads = patternDefExtended.flatMap(
                (patt) => {
                    const gfx = patt.patternGraphicsDef;
                    this.#prepareSilk(gfx.layerContents, textStyles, patt.name);

                    const pads = patt.patternGraphicsDef?.multiLayer.pad;
                    return pads.map(pad => ({
                        patternRef: patt.name,
                        pad: pad
                    }));
                }
            );

            flattenedPads.forEach(
                ({ pad, patternRef }) => {
                    const assembleX = this.unitsConvertFn(pad.pt[0]);
                    const assembleY = this.unitsConvertFn(pad.pt[1]);
                    const { padStyleRef, padNum, defaultPinDes, rotation } = pad;
                    const { shapes, holeRad, refRange } = padsStyles[padStyleRef];

                    const changedSystemCords = new DOMMatrix()
                        .translate(assembleX, assembleY)
                        .rotate(rotation);

                    for (var layerId = refRange.startRange; layerId <= refRange.endRange; ++layerId) {
                        const layer = this.#ensureLayer(layerId);

                        if (!layer) 
                            continue;

                        const baked = shapes.byLayerRef[layerId] 
                            || shapes.byLayerType[layer.layerType];

                        if (baked) {
                            if (!layer.patterns[patternRef]) {
                                layer.patterns[patternRef] = {
                                    footPrint: null,
                                    lines: null,
                                    arcs: null,
                                    holes: null,
                                    labels: null,
                                    keepOuts: null,
                                    flipped: false
                                };
                            }

                            if (!layer.patterns[patternRef].footPrint)
                                layer.patterns[patternRef].footPrint = new Path2D();

                            layer.patterns[patternRef].footPrint.addPath(baked, changedSystemCords)

                            if (!layer.patterns[patternRef].holes)
                                layer.patterns[patternRef].holes = new Path2D();

                            const holePath = new Path2D();
                            holePath.arc(0, 0, holeRad, 0, Math.PI * 2);

                            layer.patterns[patternRef].holes.addPath(holePath, changedSystemCords);

                            if (!this.multilayer.patterns[patternRef]) 
                                this.multilayer.patterns[patternRef] = new Set();

                            this.multilayer.patterns[patternRef].add(layerId);
                        }
                    }
                }
            );
        }

        #defineConvertUnitsFn(fileUnits) {
            switch (fileUnits) {
                case "mm" :
                    this.fileUnit = "mm";
                    this.unitsConvertFn = this.mm2px;
                    break;
                default:
                    this.unitsConvertFn = this.mm2px;
                    break;
            }
        }

        #getViasStyles(library) {
            return library.reduce(
                (acc, style) => { 
                    const shapes = style.viaShape.reduce(
                        (shapeAcc, sh) => { 
                            const shapeGeometry = this.#bakeShape(
                                this.#pickShape(sh)
                            );

                            if (sh.layerRef?.layerNumRef) {
                                shapeAcc.byLayerRef[sh.layerRef.layerNumRef] = shapeGeometry;
                            } else if (sh.layerRef?.layerType) {
                                shapeAcc.byLayerType[sh.layerRef.layerType] = shapeGeometry;
                            }

                            return shapeAcc; 
                        },
                        { byLayerRef: {}, byLayerType: {} }
                    );

                    acc[style.name] = {
                        holeRad: this.unitsConvertFn(style.holeDiam) / 2,
                        shapes: shapes,
                        refRange: { 
                            startRange: style.StartRange, 
                            endRange: style.EndRange 
                        } 
                    };

                    return acc; 
                },
                {}
            );
        }

        #defineVias(vias, viaStyles) {
            vias.forEach(
                (via) => {
                    const globalX = this.unitsConvertFn(via.pt[0]);
                    const globalY = this.unitsConvertFn(via.pt[1]);
                    const { holeRad, shapes, refRange } = viaStyles[via.viaStyleRef];

                    var drillHole = new Path2D();

                    drillHole.moveTo(globalX + holeRad, globalY);
                    drillHole.arc(globalX, globalY, holeRad, 0, 2 * Math.PI);

                    const changedSystemCords = new DOMMatrix().translate(globalX, globalY);

                    for (var layerId = refRange.startRange; layerId <= refRange.endRange; ++layerId) {
                        const layer = this.#ensureLayer(layerId);

                        if (!layer) 
                            continue;

                        if (!layer.vias) {
                            layer.vias = {
                                vias: new Path2D(),
                                holes: new Path2D()
                            }
                        }

                        layer.vias.holes.addPath(drillHole);

                        const baked = shapes.byLayerRef[layerId] 
                            || shapes.byLayerType[layer.layerType];

                        if (baked) {
                            layer.vias.vias.addPath(baked, changedSystemCords);
                            if (!this.multilayer.vias[via.viaStyleRef]) 
                                this.multilayer.vias[via.viaStyleRef] = new Set();

                            this.multilayer.vias[via.viaStyleRef].add(layerId);
                        }
                    }
                }
            );
        }

        #definePatterns(patterns) {
            patterns.forEach(
                ({ patternRef, refDesRef, pt, rotation, isFlipped, patternGraphicsRef}) => {
                    const x = this.unitsConvertFn(pt[0]);
                    const y = this.unitsConvertFn(pt[1]);
                    const rot = rotation || 0;
                    const layers = this.multilayer.patterns[patternRef];

                    const matrix = new DOMMatrix()
                        .translate(x, y)
                        .scale(isFlipped ? -1 : 1, 1) 
                        .rotate(rot);

                    this.multilayer.patterns[patternRef].forEach(
                        (layerId) => {
                            const pattern = this.layers[layerId].patterns[patternRef];

                            var instance = {
                                flipped: isFlipped,
                            }

                            if (pattern.footPrint) {
                                const tmp = new Path2D();
                                tmp.addPath(pattern.footPrint, matrix);
                                instance.footPrint = tmp;
                            }

                            if (pattern.lines) {
                                instance.lines = {};
                                pattern.lines.forEach(line => (
                                    (instance.lines[line.width] = instance.lines[line.width]
                                        || { width: line.width, path: new Path2D() }))
                                            .path.addPath(line.path, matrix)
                                );
                            }

                            if (pattern.arcs) {
                                instance.arcs = {};
                                pattern.arcs.forEach(arc => (
                                    (instance.arcs[arc.width] = instance.arcs[arc.width]
                                        || { width: arc.width, path: new Path2D() }))
                                            .path.addPath(arc.path, matrix)
                                );
                            }

                            if (pattern.keepOuts) {
                                const tmp = new Path2D();
                                tmp.addPath(pattern.keepOuts, matrix);
                                instance.keepOuts = tmp;
                            }

                            if (pattern.holes) {
                                const tmpHoles = new Path2D();
                                tmpHoles.addPath(pattern.holes, matrix);
                                instance.holes = tmpHoles;
                            }

                            if (pattern.labels) {
                                instance.labels = Array();
                                patternGraphicsRef.attributes.forEach((attr) => {
                                    const labelDesc = pattern.labels[attr.label];

                                    const p = new DOMPoint(
                                        attr.pt ? this.unitsConvertFn(attr.pt[0]) : labelDesc.x,
                                        attr.pt ? this.unitsConvertFn(attr.pt[1]) : labelDesc.y
                                    ).matrixTransform(matrix);

                                    instance.labels.push({
                                        text: attr.name || attr.label,
                                        x: p.x,
                                        y: p.y,
                                        font: labelDesc.font,
                                        rotation: rot
                                    });
                                });
                            }
                            this.layers[layerId].instances[refDesRef] = instance;
                        }
                    );
                }
            );
        }


        #defineMultilayer(multilayer, library) {
            this.#defineVias(
                multilayer.vias, 
                this.#getViasStyles(library.viaStyleDef)
            );

            this.#preparePatterns(
                library.patternDefExtended,
                this.#getPadsStyles(
                    library.padStyleDef
                ),
                this.#getTextStyles(
                    library.textStyleDef
                )
            );

            this.#definePatterns(
                multilayer.patterns
            );
        }

        #ensureLayer(layerNum) {
            if (!this.layers[layerNum]) {
                const layerInfo = this.layersBook[layerNum];
                this.layers[layerNum] = {
                    name: layerInfo.layerName,
                    layerType: layerInfo.layerType,
                    patterns: {},
                    instances: {},
                    vias: null,
                    nets: {},
                    visible: true,
                    pours: null
                };

                this.order.push(layerNum);
            }
            return this.layers[layerNum];
        }


        #defineLayers(layerContents, layerDefs) {
            this.bbox = {
                minX: Infinity, 
                minY: Infinity,
                maxY: -Infinity,
                maxX: -Infinity
            };


            this.layersBook = layerDefs.reduce(
                (acc, el) => {
                    acc[el.layerNum] = { layerName: el.name, layerType: el.layerType};
                    return acc;
                },
                {}
            );

            this.order = Array();

            this.layerGroupType = {};
            this.multilayer = {
                silk: new Set(),
                patterns: {},
                vias: {},
                highlight: {},
            };

            this.layers = layerContents.reduce(
                (acc, el, idx) => {
                    const layerRef = el.layerRef.layerNumRef;
                    const { layerName, layerType } = this.layersBook[layerRef];
                    if (layerName == "Board") {
                        this.edges = Array();
                        el.lines.forEach(
                            ({ start, end, width }) => {
                                this.edges.push(
                                    {
                                        start: [this.unitsConvertFn(start[0]), this.unitsConvertFn(start[1])],
                                        end: [this.unitsConvertFn(end[0]), this.unitsConvertFn(end[1])],
                                        width: this.unitsConvertFn(width)
                                    }
                                );

                                this.bbox.minX = Math.min(this.bbox.minX, start[0]);
                                this.bbox.minY = Math.min(this.bbox.minY, start[1]);
                                this.bbox.maxX = Math.max(this.bbox.maxX, start[0]);
                                this.bbox.maxY = Math.max(this.bbox.maxY, start[1]);

                                this.bbox.minX = Math.min(this.bbox.minX, end[0]);
                                this.bbox.minY = Math.min(this.bbox.minY, end[1]);
                                this.bbox.maxX = Math.max(this.bbox.maxX, end[0]);
                                this.bbox.maxY = Math.max(this.bbox.maxY, end[1]);
                            }
                        );
                    } else {
                        acc[layerRef] = {
                            name: layerName,
                            layerType: layerType,
                            vias: null,
                            patterns: {},
                            instances: {},
                            nets: this.#createNetsPath(el.lines),
                            visible: true,
                            pours: el.copperPours.length > 0 ? el.copperPours.reduce(
                                (acc, pour) => {
                                    if (!acc[pour.netNameRef])
                                        acc[pour.netNameRef] = {};
                                    const width = this.unitsConvertFn(pour.width);
                                    acc[pour.netNameRef][width] = {
                                        width: width, 
                                        pour: this.#createZonePath(pour)
                                    }

                                    return acc;
                                },
                                {}
                            ) : null
                        }

                        this.order.push(layerRef);

                        (this.layerGroupType[layerType] = this.layerGroupType[layerType] || Array()).push(layerRef);
                    }

                    return acc;
                },
                {}
            );

            this.bbox.minX = this.unitsConvertFn(this.bbox.minX);
            this.bbox.minY = this.unitsConvertFn(this.bbox.minY);
            this.bbox.maxX = this.unitsConvertFn(this.bbox.maxX);
            this.bbox.maxY = this.unitsConvertFn(this.bbox.maxY);
        }

        #createNetsPath(line) {
            return line.reduce(
                (acc, el) => {
                    const width = this.unitsConvertFn(el.width);
                    acc[el.netNameRef] = acc[el.netNameRef] || {};
                    acc[el.netNameRef][width] = ( 
                        acc[el.netNameRef][width] || {
                            width: width, path: new Path2D()
                        }
                    );
                    
                    const path = acc[el.netNameRef][width].path;
                    path.moveTo(this.unitsConvertFn(el.start[0]), this.unitsConvertFn(el.start[1]));
                    path.lineTo(this.unitsConvertFn(el.end[0]), this.unitsConvertFn(el.end[1]));
                    
                    return acc; 
                },
                {}
            );
        }

        #createZonePath(pour) {
            return pour.islands.reduce(
                (acc, island) => {
                    const iPts = island.islandOutline;
                    if (iPts.length < 3) return;

                    acc.copper.moveTo(this.unitsConvertFn(iPts[0][0]), this.unitsConvertFn(iPts[0][1]));
                    for (let i = 1; i < iPts.length; ++i) {
                        acc.copper.lineTo(this.unitsConvertFn(iPts[i][0]), this.unitsConvertFn(iPts[i][1]));
                    }
                    acc.copper.closePath();

                    const tLines = island.thermals;
                    if (tLines && tLines.length > 0) {
                        tLines.forEach(thermal => {
                            const w = this.unitsConvertFn(thermal.thermalWidth);
                            (!acc.thermals[w]) && (acc.thermals[w] = { path: new Path2D(), width: w });

                            acc.thermals[w].path.moveTo(this.unitsConvertFn(thermal.start[0]), this.unitsConvertFn(thermal.start[1]));
                            acc.thermals[w].path.lineTo(this.unitsConvertFn(thermal.end[0]), this.unitsConvertFn(thermal.end[1]));
                        });
                    }

                    const cPts = island.cutout?.cutoutOutline;
                    if (cPts && cPts.length >= 3) {
                        acc.copper.moveTo(this.unitsConvertFn(cPts[0][0]), this.unitsConvertFn(cPts[0][1]));
                        for (let i = 1; i < cPts.length; ++i) {
                            acc.copper.lineTo(this.unitsConvertFn(cPts[i][0]), this.unitsConvertFn(cPts[i][1]));
                        }
                        acc.copper.closePath();
                    }

                    return acc;
                },
                {
                    copper: new Path2D(),
                    thermals: {}
                }
            );
        }
    }