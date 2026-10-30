use serde::{Deserialize, Serialize};
use crate::pcad_parser::{PcadDeserialize, PcadParser, ParserError, Wrapped, TokenKind, TokenType};


#[derive(Debug, Serialize, Deserialize)]
pub struct PcbSpec {
    #[serde(rename = "asciiHeader")]
    pub ascii_header: AsciiHeader,
    pub library: Library,
    pub netlist: Netlist,
    #[serde(rename = "pcbDesign")]
    pub pcb_design: PcbDesign
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "asciiHeader")]
pub struct AsciiHeader {
    #[serde(rename = "asciiVersion")]
    pub ascii_version: (u32, u32),
    #[serde(rename = "timeStamp")]
    pub time_stamp: (u32, u32, u32, u32, u32, u32),
    #[serde(rename = "program")]
    pub program: (String, String),
    #[serde(rename = "copyright")]
    pub copyright: String,
    #[serde(rename = "fileAuthor")]
    pub file_author: String,
    #[serde(rename = "headerString")]
    pub header_string: String,
    #[serde(rename = "fileUnits")]
    pub file_units: String,
    #[serde(rename = "guidString")]
    pub guid_string: String,
}

impl PcadDeserialize for AsciiHeader {
    const TAG: &'static [u8] = b"asciiHeader";

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"asciiVersion" => {
                self.ascii_version = parser.parse_field(|p| {
                    let v1 = p.parse_u32()?;
                    let v2 = p.parse_u32()?;
                    Ok((v1, v2))
                })?;
                Ok(true)
            },
            b"timeStamp" => {
                self.time_stamp = parser.parse_field(|p| {
                    let y = p.parse_u32()?;
                    let m = p.parse_u32()?;
                    let d = p.parse_u32()?;
                    let h = p.parse_u32()?;
                    let min = p.parse_u32()?;
                    let s = p.parse_u32()?;
                    Ok((y, m, d, h, min, s))
                })?;
                Ok(true)
            },
            b"program" => {
                self.program = parser.parse_field(|p| {
                    let p1 = p.parse_string()?;
                    let p2 = p.parse_string()?;
                    Ok((p1, p2))
                })?;
                Ok(true)
            },
            b"copyright" => {
                self.copyright = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            },
            b"fileAuthor" => {
                self.file_author = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            },
            b"headerString" => {
                self.header_string = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            },
            b"fileUnits" => {
                self.file_units = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            },
            b"guidString" => {
                self.guid_string = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            },
            _ => {
                Ok(false)
            }
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Library {
    pub name: String,
    #[serde(rename = "padStyleDef", default)]
    pub pad_style_defs: Vec<PadStyleDef>,
    #[serde(rename = "viaStyleDef", default)]
    pub via_style_defs: Vec<ViaStyleDef>,
    #[serde(rename = "textStyleDef", default)]
    pub text_style_defs: Vec<TextStyleDef>,
    #[serde(rename = "patternDefExtended", default)]
    pub pattern_def_extendeds: Vec<PatternDefExtended>,
    #[serde(rename = "compDef", default)]
    pub comp_defs: Vec<CompDef>,
}

impl PcadDeserialize for Library {
    const TAG: &'static [u8] = b"library";
    const HAS_INTERNAL_NAME: bool = true;

    fn load_internal_name(&mut self, name: &[u8]) {
        self.name = String::from_utf8_lossy(name).into_owned();
    }

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"padStyleDef" => {
                let pad_style = parser.parse_common_struct::<PadStyleDef>(Wrapped::In)?;
                self.pad_style_defs.push(pad_style);
                Ok(true)
            }
            b"viaStyleDef" => {
                let via_style = parser.parse_common_struct::<ViaStyleDef>(Wrapped::In)?;
                self.via_style_defs.push(via_style);
                Ok(true)
            }
            b"textStyleDef" => {
                let text_style = parser.parse_common_struct::<TextStyleDef>(Wrapped::In)?;
                self.text_style_defs.push(text_style);
                Ok(true)
            }
            b"patternDefExtended" => {
                let pattern = parser.parse_common_struct::<PatternDefExtended>(Wrapped::In)?;
                self.pattern_def_extendeds.push(pattern);
                Ok(true)
            }
            b"compDef" => {
                let comp = parser.parse_common_struct::<CompDef>(Wrapped::In)?;
                self.comp_defs.push(comp);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PadStyleDef {
    pub name: String,
    #[serde(rename = "holeDiam")]
    pub hole_diam: f64,
    #[serde(rename = "isHolePlated", default)]
    pub is_hole_plated: Option<bool>,
    #[serde(rename = "StartRange")]
    pub start_range: u32,
    #[serde(rename = "EndRange")]
    pub end_range: u32,
    #[serde(rename = "padShape", default)]
    pub pad_shapes: Vec<PadShape>,
}

impl PcadDeserialize for PadStyleDef {
    const TAG: &'static [u8] = b"padStyleDef";
    const HAS_INTERNAL_NAME: bool = true;

    fn load_internal_name(&mut self, name: &[u8]) {
        self.name = String::from_utf8_lossy(name).into_owned();
    }

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"holeDiam" => {
                self.hole_diam = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"isHolePlated" => {
                self.is_hole_plated = Some(parser.parse_field(|p| p.parse_bool())?);
                Ok(true)
            }
            b"StartRange" => {
                self.start_range = parser.parse_field(|p| p.parse_u32())?;
                Ok(true)
            }
            b"EndRange" => {
                self.end_range = parser.parse_field(|p| p.parse_u32())?;
                Ok(true)
            }
            b"padShape" => {
                let pad_shape = parser.parse_common_struct::<PadShape>(Wrapped::In)?;
                self.pad_shapes.push(pad_shape);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PadShape {
    #[serde(rename = "layerRef")]
    pub layer_ref: LayerRef,
    #[serde(rename = "padShapeType")]
    pub shape_type: PadShapeType,
    #[serde(rename = "shapeWidth")]
    pub shape_width: f64,
    #[serde(rename = "shapeHeight")]
    pub shape_height: f64,
    #[serde(rename = "outsideDiam", default)]
    pub outside_diam: Option<f64>,
    #[serde(rename = "insideDiam", default)]
    pub inside_diam: Option<f64>,
    #[serde(rename = "spokeWidth", default)]
    pub spoke_width: Option<f64>,
}

impl PcadDeserialize for PadShape {
    const TAG: &'static [u8] = b"padShape";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"layerNumRef" => {
                let layer_num = parser.parse_field(|p| p.parse_u32())?;
                self.layer_ref = LayerRef::Num(layer_num);
                Ok(true)
            }
            b"layerType" => {
                let layer_type_str = parser.parse_field(|p| p.parse_string())?;
                let layer_type = match layer_type_str.as_str() {
                    "Signal" => LayerType::Signal,
                    "NonSignal" => LayerType::NonSignal,
                    "Plane" => LayerType::Plane,
                    _ => return Err(ParserError::GeneralError("Unknown layer type")),
                };
                self.layer_ref = LayerRef::Type(layer_type);
                Ok(true)
            }
            b"padShapeType" => {
                let shape_type_str = parser.parse_field(|p| p.parse_string())?;
                self.shape_type = match shape_type_str.as_str() {
                    "Ellipse" => PadShapeType::Ellipse,
                    "Rect" => PadShapeType::Rect,
                    "Thrm4_45" => PadShapeType::Thrm4_45,
                    _ => return Err(ParserError::GeneralError("Unknown shape type")),
                };
                Ok(true)
            }
            b"shapeWidth" => {
                self.shape_width = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"shapeHeight" => {
                self.shape_height = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"outsideDiam" => {
                self.outside_diam = Some(parser.parse_field(|p| p.parse_f64())?);
                Ok(true)
            }
            b"insideDiam" => {
                self.inside_diam = Some(parser.parse_field(|p| p.parse_f64())?);
                Ok(true)
            }
            b"spokeWidth" => {
                self.spoke_width = Some(parser.parse_field(|p| p.parse_f64())?);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LayerRef {
    #[serde(rename = "layerNumRef")]
    Num(u32),
    #[serde(rename = "layerType")]
    Type(LayerType),
}

impl Default for LayerRef {
    fn default() -> Self {
        LayerRef::Num(1)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum PadShapeType {
    #[default]
    #[serde(rename = "Ellipse")]
    Ellipse,
    #[serde(rename = "Rect")]
    Rect,
    #[serde(rename = "Thrm4_45")]
    Thrm4_45,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ViaStyleDef {
    pub name: String,
    #[serde(rename = "holeDiam")]
    pub hole_diam: f64,
    #[serde(rename = "StartRange")]
    pub start_range: u32,
    #[serde(rename = "EndRange")]
    pub end_range: u32,
    #[serde(rename = "viaShape", default)]
    pub via_shapes: Vec<ViaShape>,
}

impl PcadDeserialize for ViaStyleDef {
    const TAG: &'static [u8] = b"viaStyleDef";
    const HAS_INTERNAL_NAME: bool = true;

    fn load_internal_name(&mut self, name: &[u8]) {
        self.name = String::from_utf8_lossy(name).into_owned();
    }

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"holeDiam" => {
                self.hole_diam = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"StartRange" => {
                self.start_range = parser.parse_field(|p| p.parse_u32())?;
                Ok(true)
            }
            b"EndRange" => {
                self.end_range = parser.parse_field(|p| p.parse_u32())?;
                Ok(true)
            }
            b"viaShape" => {
                let via_shape = parser.parse_common_struct::<ViaShape>(Wrapped::In)?;
                self.via_shapes.push(via_shape);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ViaShape {
    #[serde(rename = "layerRef")]
    pub layer_ref: LayerRef,
    #[serde(rename = "viaShapeType")]
    pub shape_type: PadShapeType, 
    #[serde(rename = "shapeWidth")]
    pub shape_width: f64,
    #[serde(rename = "shapeHeight")]
    pub shape_height: f64,
    #[serde(rename = "outsideDiam", default)]
    pub outside_diam: Option<f64>,
    #[serde(rename = "insideDiam", default)]
    pub inside_diam: Option<f64>,
    #[serde(rename = "spokeWidth", default)]
    pub spoke_width: Option<f64>,
}

impl PcadDeserialize for ViaShape {
    const TAG: &'static [u8] = b"viaShape";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"layerNumRef" => {
                let layer_num = parser.parse_field(|p| p.parse_u32())?;
                self.layer_ref = LayerRef::Num(layer_num); 
                Ok(true)
            }
            b"layerType" => {
                let layer_type_str = parser.parse_field(|p| p.parse_string())?;
                let layer_type = match layer_type_str.as_str() {
                    "Signal" => LayerType::Signal,
                    "NonSignal" => LayerType::NonSignal,
                    "Plane" => LayerType::Plane, 
                    _ => return Err(ParserError::GeneralError("Unknown layer type")),
                };
                self.layer_ref = LayerRef::Type(layer_type); 
                Ok(true)
            }
            b"viaShapeType" => {
                let shape_type_str = parser.parse_field(|p| p.parse_string())?;
                self.shape_type = match shape_type_str.as_str() {
                    "Ellipse" => PadShapeType::Ellipse,
                    "Rect" => PadShapeType::Rect,
                    "Thrm4_45" => PadShapeType::Thrm4_45,
                    _ => return Err(ParserError::GeneralError("Unknown shape type")),
                };
                Ok(true)
            }
            b"shapeWidth" => {
                self.shape_width = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"shapeHeight" => {
                self.shape_height = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"outsideDiam" => {
                self.outside_diam = Some(parser.parse_field(|p| p.parse_f64())?);
                Ok(true)
            }
            b"insideDiam" => {
                self.inside_diam = Some(parser.parse_field(|p| p.parse_f64())?);
                Ok(true)
            }
            b"spokeWidth" => {
                self.spoke_width = Some(parser.parse_field(|p| p.parse_f64())?);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TextStyleDef {
    pub name: String,
    #[serde(rename = "font", default)]
    pub fonts: Vec<FontDef>,
    #[serde(rename = "textStyleAllowTType")]
    pub allow_ttype: bool,
    #[serde(rename = "textStyleDisplayTType")]
    pub display_ttype: bool,
}

impl PcadDeserialize for TextStyleDef {
    const TAG: &'static [u8] = b"textStyleDef";
    const HAS_INTERNAL_NAME: bool = true;

    fn load_internal_name(&mut self, name: &[u8]) {
        self.name = String::from_utf8_lossy(name).into_owned();
    }

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"font" => {
                let font = parser.parse_common_struct::<FontDef>(Wrapped::In)?;
                self.fonts.push(font);
                Ok(true)
            }
            b"textStyleAllowTType" => {
                self.allow_ttype = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"textStyleDisplayTType" => {
                self.display_ttype = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FontDef {
    #[serde(rename = "fontType")]
    pub font_type: FontType,
    #[serde(rename = "fontFamily")]
    pub font_family: String,
    #[serde(rename = "fontFace")]
    pub font_face: String,
    #[serde(rename = "fontHeight")]
    pub font_height: f64,
    #[serde(rename = "strokeWidth")]
    pub stroke_width: f64,
    #[serde(rename = "fontWeight", default)]
    pub font_weight: Option<u32>,
    #[serde(rename = "fontCharSet", default)]
    pub font_char_set: Option<u32>,
    #[serde(rename = "fontOutPrecision", default)]
    pub font_out_precision: Option<u32>,
    #[serde(rename = "fontClipPrecision", default)]
    pub font_clip_precision: Option<u32>,
    #[serde(rename = "fontQuality", default)]
    pub font_quality: Option<u32>,
    #[serde(rename = "fontPitchAndFamily", default)]
    pub font_pitch_and_family: Option<u32>,
}

impl PcadDeserialize for FontDef {
    const TAG: &'static [u8] = b"font";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"fontType" => {
                let font_type_str = parser.parse_field(|p| p.parse_string())?;
                self.font_type = match font_type_str.as_str() {
                    "Stroke" => FontType::Stroke,
                    "TrueType" => FontType::TrueType,
                    _ => return Err(ParserError::GeneralError("Unknown font type")),
                };
                Ok(true)
            }
            b"fontFamily" => {
                self.font_family = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"fontFace" => {
                self.font_face = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"fontHeight" => {
                self.font_height = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"strokeWidth" => {
                self.stroke_width = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"fontWeight" => {
                self.font_weight = Some(parser.parse_field(|p| p.parse_u32())?);
                Ok(true)
            }
            b"fontCharSet" => {
                self.font_char_set = Some(parser.parse_field(|p| p.parse_u32())?);
                Ok(true)
            }
            b"fontOutPrecision" => {
                self.font_out_precision = Some(parser.parse_field(|p| p.parse_u32())?);
                Ok(true)
            }
            b"fontClipPrecision" => {
                self.font_clip_precision = Some(parser.parse_field(|p| p.parse_u32())?);
                Ok(true)
            }
            b"fontQuality" => {
                self.font_quality = Some(parser.parse_field(|p| p.parse_u32())?);
                Ok(true)
            }
            b"fontPitchAndFamily" => {
                self.font_pitch_and_family = Some(parser.parse_field(|p| p.parse_u32())?);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum FontType {
    #[default]
    #[serde(rename = "Stroke")]
    Stroke,
    #[serde(rename = "TrueType")]
    TrueType,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PatternDefExtended {
    pub name: String,
    #[serde(rename = "originalName")]
    pub original_name: String,
    #[serde(rename = "patternGraphicsNameRef")]
    pub pattern_graphics_name_ref: String,
    #[serde(rename = "patternGraphicsDef")]
    pub pattern_graphics_def: PatternGraphicsDef,
}

impl PcadDeserialize for PatternDefExtended {
    const TAG: &'static [u8] = b"patternDefExtended";
    const HAS_INTERNAL_NAME: bool = true;

    fn load_internal_name(&mut self, name: &[u8]) {
        self.name = String::from_utf8_lossy(name).into_owned();
    }

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"originalName" => {
                self.original_name = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"patternGraphicsNameRef" => {
                self.pattern_graphics_name_ref = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"patternGraphicsDef" => {
                self.pattern_graphics_def = parser.parse_common_struct::<PatternGraphicsDef>(Wrapped::In)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PatternGraphicsDef {
    #[serde(rename = "patternGraphicsNameDef")]
    pub name_def: String,
    #[serde(rename = "multiLayer")]
    pub multi_layer: MultiLayerContents,
    #[serde(rename = "layerContents", default)]
    pub layer_contents: Vec<LayerContents>,
}

impl PcadDeserialize for PatternGraphicsDef {
    const TAG: &'static [u8] = b"patternGraphicsDef";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"patternGraphicsNameDef" => {
                self.name_def = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"multiLayer" => {
                self.multi_layer = parser.parse_common_struct::<MultiLayerContents>(Wrapped::In)?;
                Ok(true)
            }
            b"layerContents" => {
                let layer_contents = parser.parse_common_struct::<LayerContents>(Wrapped::In)?;
                self.layer_contents.push(layer_contents);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MultiLayerContents {
    #[serde(rename = "pad", default)]
    pub pads: Vec<Pad>,
    #[serde(rename = "gluepoint", default)]
    pub gluepoints: Vec<GluepointDef>,
    #[serde(rename = "pickpoint", default)]
    pub pickpoints: Vec<PickpointDef>,
}

impl PcadDeserialize for MultiLayerContents {
    const TAG: &'static [u8] = b"multiLayer";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"pad" => {
                let pad = parser.parse_common_struct::<Pad>(Wrapped::In)?;
                self.pads.push(pad);
                Ok(true)
            }
            b"gluepoint" => {
                let gluepoint = parser.parse_common_struct::<GluepointDef>(Wrapped::In)?;
                self.gluepoints.push(gluepoint);
                Ok(true)
            }
            b"pickpoint" => {
                let pickpoint = parser.parse_common_struct::<PickpointDef>(Wrapped::In)?;
                self.pickpoints.push(pickpoint);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Pad {
    #[serde(rename = "padNum")]
    pub pad_num: u32,
    #[serde(rename = "padStyleRef")]
    pub pad_style_ref: String,
    #[serde(rename = "pt")]
    pub point: (f64, f64),
    #[serde(rename = "rotation", default)]
    pub rotation: Option<f64>,
    #[serde(rename = "defaultPinDes", default)]
    pub default_pin_des: Option<String>,
}

impl PcadDeserialize for Pad {
    const TAG: &'static [u8] = b"pad";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"padNum" => {
                self.pad_num = parser.parse_field(|p| p.parse_u32())?;
                Ok(true)
            }
            b"padStyleRef" => {
                self.pad_style_ref = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"pt" => {
                let x = parser.parse_f64()?;
                let y = parser.parse_f64()?;
                self.point = (x, y);
                parser.expect_token(TokenKind::CloseParen)?;
                Ok(true)
            }
            b"rotation" => {
                self.rotation = Some(parser.parse_field(|p| p.parse_f64())?);
                Ok(true)
            }
            b"defaultPinDes" => {
                self.default_pin_des = Some(parser.parse_field(|p| p.parse_string())?);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GluepointDef {
    #[serde(rename = "pt")]
    pub point: (f64, f64),
}

impl PcadDeserialize for GluepointDef {
    const TAG: &'static [u8] = b"gluepoint";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"pt" => {
                let x = parser.parse_f64()?;
                let y = parser.parse_f64()?;
                self.point = (x, y);
                parser.expect_token(TokenKind::CloseParen)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PickpointDef {
    #[serde(rename = "pt")]
    pub point: (f64, f64),
}

impl PcadDeserialize for PickpointDef {
    const TAG: &'static [u8] = b"pickpoint";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"pt" => {
                let x = parser.parse_f64()?;
                let y = parser.parse_f64()?;
                self.point = (x, y);
                parser.expect_token(TokenKind::CloseParen)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompDef {
    pub name: String,
    #[serde(rename = "originalName")]
    pub original_name: String,
    #[serde(rename = "compHeader")]
    pub comp_header: CompHeader,
    #[serde(rename = "compPin", default)]
    pub comp_pins: Vec<CompPin>,
    #[serde(rename = "attachedSymbol")]
    pub attached_symbol: AttachedSymbol,
    #[serde(rename = "attachedPattern")]
    pub attached_pattern: AttachedPattern,
}

impl PcadDeserialize for CompDef {
    const TAG: &'static [u8] = b"compDef";
    const HAS_INTERNAL_NAME: bool = true;

    fn load_internal_name(&mut self, name: &[u8]) {
        self.name = String::from_utf8_lossy(name).into_owned();
    }

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"originalName" => {
                self.original_name = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"compHeader" => {
                self.comp_header = parser.parse_common_struct::<CompHeader>(Wrapped::In)?;
                Ok(true)
            }
            b"compPin" => {
                let comp_pin = parser.parse_common_struct::<CompPin>(Wrapped::In)?;
                self.comp_pins.push(comp_pin);
                Ok(true)
            }
            b"attachedSymbol" => {
                self.attached_symbol = parser.parse_common_struct::<AttachedSymbol>(Wrapped::In)?;
                Ok(true)
            }
            b"attachedPattern" => {
                self.attached_pattern = parser.parse_common_struct::<AttachedPattern>(Wrapped::In)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompHeader {
    #[serde(rename = "sourceLibrary")]
    pub source_library: String,
    #[serde(rename = "numPins")]
    pub num_pins: u32,
    #[serde(rename = "numParts")]
    pub num_parts: u32,
    #[serde(rename = "alts")]
    pub alts: Alts,
    #[serde(rename = "refDesPrefix")]
    pub ref_des_prefix: String,
    #[serde(rename = "numType", default)]
    pub num_type: Option<PinNumberingType>,
}

impl PcadDeserialize for CompHeader {
    const TAG: &'static [u8] = b"compHeader";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"sourceLibrary" => {
                self.source_library = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"numPins" => {
                self.num_pins = parser.parse_field(|p| p.parse_u32())?;
                Ok(true)
            }
            b"numParts" => {
                self.num_parts = parser.parse_field(|p| p.parse_u32())?;
                Ok(true)
            }
            b"alts" => {
                self.alts = parser.parse_common_struct::<Alts>(Wrapped::In)?;
                Ok(true)
            }
            b"refDesPrefix" => {
                self.ref_des_prefix = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"numType" => {
                let num_type_str = parser.parse_field(|p| p.parse_string())?;
                self.num_type = Some(match num_type_str.as_str() {
                    "Numeric" => PinNumberingType::Numeric,
                    "Alphabetic" => PinNumberingType::Alphabetic,
                    _ => return Err(ParserError::GeneralError("Unknown pin numbering type")),
                });
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Alts {
    #[serde(rename = "ieeeAlt")]
    pub ieee_alt: bool,
    #[serde(rename = "deMorganAlt")]
    pub de_morgan_alt: bool,
}

impl PcadDeserialize for Alts {
    const TAG: &'static [u8] = b"alts";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"ieeeAlt" => {
                self.ieee_alt = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"deMorganAlt" => {
                self.de_morgan_alt = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PinNumberingType {
    #[serde(rename = "Numeric")]
    Numeric,
    #[serde(rename = "Alphabetic")]
    Alphabetic,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompPin {
    pub name: String,
    #[serde(rename = "pinName", default)]
    pub pin_name: Option<String>,
    #[serde(rename = "partNum")]
    pub part_num: u32,
    #[serde(rename = "symPinNum")]
    pub sym_pin_num: u32,
    #[serde(rename = "gateEq")]
    pub gate_eq: u32,
    #[serde(rename = "pinEq")]
    pub pin_eq: u32,
    #[serde(rename = "pinType", default)]
    pub pin_type: Option<PinType>,
}

impl PcadDeserialize for CompPin {
    const TAG: &'static [u8] = b"compPin";
    const HAS_INTERNAL_NAME: bool = true;

    fn load_internal_name(&mut self, name: &[u8]) {
        self.name = String::from_utf8_lossy(name).into_owned();
    }

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"pinName" => {
                self.pin_name = Some(parser.parse_field(|p| p.parse_string())?);
                Ok(true)
            }
            b"partNum" => {
                self.part_num = parser.parse_field(|p| p.parse_u32())?;
                Ok(true)
            }
            b"symPinNum" => {
                self.sym_pin_num = parser.parse_field(|p| p.parse_u32())?;
                Ok(true)
            }
            b"gateEq" => {
                self.gate_eq = parser.parse_field(|p| p.parse_u32())?;
                Ok(true)
            }
            b"pinEq" => {
                self.pin_eq = parser.parse_field(|p| p.parse_u32())?;
                Ok(true)
            }
            b"pinType" => {
                let pin_type_str = parser.parse_field(|p| p.parse_string())?;
                self.pin_type = Some(match pin_type_str.as_str() {
                    "Passive" => PinType::Passive,
                    "Input" => PinType::Input,
                    "Output" => PinType::Output,
                    _ => return Err(ParserError::GeneralError("Unknown pin type")),
                });
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PinType {
    #[serde(rename = "Passive")]
    Passive,
    #[serde(rename = "Input")]
    Input,
    #[serde(rename = "Output")]
    Output,
    //TODO: add other variants
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AttachedSymbol {
    #[serde(rename = "partNum")]
    pub part_num: u32,
    #[serde(rename = "altType")]
    pub alt_type: AltType,
    #[serde(rename = "symbolName")]
    pub symbol_name: String,
}

impl PcadDeserialize for AttachedSymbol {
    const TAG: &'static [u8] = b"attachedSymbol";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"partNum" => {
                self.part_num = parser.parse_field(|p| p.parse_u32())?;
                Ok(true)
            }
            b"altType" => {
                let alt_type_str = parser.parse_field(|p| p.parse_string())?;
                self.alt_type = match alt_type_str.as_str() {
                    "Normal" => AltType::Normal,
                    _ => return Err(ParserError::GeneralError("Unknown alt type")),
                };
                Ok(true)
            }
            b"symbolName" => {
                self.symbol_name = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum AltType {
    #[default]
    #[serde(rename = "Normal")]
    Normal,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AttachedPattern {
    #[serde(rename = "patternNum")]
    pub pattern_num: u32,
    #[serde(rename = "patternName")]
    pub pattern_name: String,
    #[serde(rename = "numPads")]
    pub num_pads: u32,
    #[serde(rename = "padPinMap", default)]
    pub pad_pin_map: Vec<PadPinMap>,
}

impl PcadDeserialize for AttachedPattern {
    const TAG: &'static [u8] = b"attachedPattern";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"patternNum" => {
                self.pattern_num = parser.parse_field(|p| p.parse_u32())?;
                Ok(true)
            }
            b"patternName" => {
                self.pattern_name = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"numPads" => {
                self.num_pads = parser.parse_field(|p| p.parse_u32())?;
                Ok(true)
            }
             b"padPinMap" => {
                let entry = parser.parse_common_struct::<PadPinMap>(Wrapped::In)?;
                self.pad_pin_map.push(entry);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PadPinMap {
    #[serde(rename = "padNum")]
    pub pad_num: u32,
    #[serde(rename = "compPinRef")]
    pub comp_pin_ref: String,
}

impl PcadDeserialize for PadPinMap {
    const TAG: &'static [u8] = b"padPinMap"; 
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"padNum" => {
                self.pad_num = parser.parse_field(|p| p.parse_u32())?;
                Ok(true)
            }
            b"compPinRef" => {
                self.comp_pin_ref = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Netlist {
    pub name: String,
    #[serde(rename = "globalAttrs", default)]
    pub global_attrs: Vec<Attr>,
    #[serde(rename = "compInst", default)]
    pub comp_insts: Vec<CompInst>,
    #[serde(rename = "net", default)]
    pub nets: Vec<Net>,
}

impl PcadDeserialize for Netlist {
    const TAG: &'static [u8] = b"netlist";
    const HAS_INTERNAL_NAME: bool = true;

    fn load_internal_name(&mut self, name: &[u8]) {
        self.name = String::from_utf8_lossy(name).into_owned();
    }

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"globalAttrs" => {
                loop {
                    if matches!(parser.peek_token_type(), Some(TokenType::CloseParen)) {
                        parser.expect_token(TokenKind::CloseParen)?;
                        break;
                    }


                    let attr = parser.parse_attr_struct(Wrapped::Out)?;
                    self.global_attrs.push(attr);
                }
                Ok(true)
            }
            b"compInst" => {
                let comp_inst = parser.parse_common_struct::<CompInst>(Wrapped::In)?;
                self.comp_insts.push(comp_inst);
                Ok(true)
            }
            b"net" => {
                let net = parser.parse_common_struct::<Net>(Wrapped::In)?;
                self.nets.push(net);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CompInst {
    pub ref_des: String,
    #[serde(rename = "compRef")]
    pub comp_ref: String,
    #[serde(rename = "originalName")]
    pub original_name: String,
    #[serde(rename = "compValue", default)]
    pub comp_value: Option<String>,
}

impl PcadDeserialize for CompInst {
    const TAG: &'static [u8] = b"compInst";
    const HAS_INTERNAL_NAME: bool = true;

    fn load_internal_name(&mut self, ref_des: &[u8]) {
        self.ref_des = String::from_utf8_lossy(ref_des).into_owned();
    }

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"compRef" => {
                self.comp_ref = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"originalName" => {
                self.original_name = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"compValue" => {
                self.comp_value = Some(parser.parse_field(|p| p.parse_string())?);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Net {
    pub name: String,
    #[serde(rename = "node", default)]
    pub nodes: Vec<Node>,
}

impl PcadDeserialize for Net {
    const TAG: &'static [u8] = b"net";
    const HAS_INTERNAL_NAME: bool = true;

    fn load_internal_name(&mut self, name: &[u8]) {
        self.name = String::from_utf8_lossy(name).into_owned();
    }

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"node" => {
                let ref_des = parser.parse_string()?;
                let pin = parser.parse_string()?;
                parser.expect_token(TokenKind::CloseParen)?;
                self.nodes.push(Node {
                    ref_des,
                    pin,
                });
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Node {
    pub ref_des: String,
    pub pin: String,
}


#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "pcbDesign")]
pub struct PcbDesign {
    pub name: String,
    #[serde(rename = "pcbDesignHeader")]
    pub pcb_design_header: PcbDesignHeader,
    #[serde(rename = "layerDefs")]
    pub layer_defs: Vec<LayerDef>,
    #[serde(rename = "multiLayer")]
    pub multi_layer: MultiLayer,
    #[serde(rename = "layerContents")]
    pub layer_contents: Vec<LayerContents>,
    #[serde(rename = "pcbPrintSettings")]
    pub pcb_print_settings: PcbPrintSettings,
    #[serde(rename = "drillSymSettings")]
    pub drill_sym_settings: DrillSymSettings,
    #[serde(rename = "gerberSettings")]
    pub gerber_settings: GerberSettings,
    #[serde(rename = "ncDrillSettings")]
    pub nc_drill_settings: NcDrillSettings,
    #[serde(rename = "programState")]
    pub program_state: ProgramState,
    #[serde(rename = "layerSets")]
    pub layer_sets: LayerSets,
    #[serde(rename = "displayDrawOrder")]
    pub display_draw_order: DisplayDrawOrder,
    #[serde(rename = "reportSettings")]
    pub report_settings: ReportSettings,
    #[serde(rename = "odbSettings")]
    pub odb_settings: OdbSettings,
    #[serde(rename = "layersStackup")]
    pub layers_stackup: LayersStackup,
}

impl PcadDeserialize for PcbDesign {
    const TAG: &'static [u8] = b"pcbDesign";
    const HAS_INTERNAL_NAME: bool = true;

    fn load_internal_name(&mut self, name: &[u8]) {
        self.name = String::from_utf8_lossy(name).into_owned();
    }

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"pcbDesignHeader" => {
                self.pcb_design_header = parser.parse_common_struct::<PcbDesignHeader>(Wrapped::In)?;
                Ok(true)
            }
            b"layerDef" => {
                let layer_def = parser.parse_common_struct::<LayerDef>(Wrapped::In)?;
                self.layer_defs.push(layer_def);
                Ok(true)
            }
            b"multiLayer" => {
                self.multi_layer = parser.parse_common_struct::<MultiLayer>(Wrapped::In)?;
                Ok(true)
            }
            b"layerContents" => {
                let layer_contents = parser.parse_common_struct::<LayerContents>(Wrapped::In)?;
                self.layer_contents.push(layer_contents);
                Ok(true)
            }
            b"pcbPrintSettings" => {
                self.pcb_print_settings = parser.parse_common_struct::<PcbPrintSettings>(Wrapped::In)?;
                Ok(true)
            }
            b"drillSymSettings" => {
                self.drill_sym_settings = parser.parse_common_struct::<DrillSymSettings>(Wrapped::In)?;
                Ok(true)
            }
            b"gerberSettings" => {
                self.gerber_settings = parser.parse_common_struct::<GerberSettings>(Wrapped::In)?;
                Ok(true)
            }
            b"ncDrillSettings" => {
                self.nc_drill_settings = parser.parse_common_struct::<NcDrillSettings>(Wrapped::In)?;
                Ok(true)
            }
            b"programState" => {
                self.program_state = parser.parse_common_struct::<ProgramState>(Wrapped::In)?;
                Ok(true)
            }
            b"layerSets" => {
                self.layer_sets = parser.parse_common_struct::<LayerSets>(Wrapped::In)?;
                Ok(true)
            }
            b"displayDrawOrder" => {
                self.display_draw_order = parser.parse_common_struct::<DisplayDrawOrder>(Wrapped::In)?;
                Ok(true)
            }
            b"reportSettings" => {
                self.report_settings = parser.parse_common_struct::<ReportSettings>(Wrapped::In)?;
                Ok(true)
            }
            b"odbSettings" => {
                self.odb_settings = parser.parse_common_struct::<OdbSettings>(Wrapped::In)?;
                Ok(true)
            }
            b"layersStackup" => {
                self.layers_stackup = parser.parse_common_struct::<LayersStackup>(Wrapped::In)?;
                Ok(true)
            }
                _ => Ok(false),
            }
        }
    }


    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct PcbDesignHeader {
        #[serde(rename = "workSpaceSize")]
        pub workspace_size: (f64, f64),
        #[serde(rename = "gridDfns")]
        pub grid_dfns: GridDfns,
        #[serde(rename = "designInfo")]
        pub design_info: DesignInfo,
        #[serde(rename = "solderSwell")]
        pub solder_swell: f64,
        #[serde(rename = "pasteSwell")]
        pub paste_swell: f64,
        #[serde(rename = "planeSwell")]
        pub plane_swell: f64,
        #[serde(rename = "refPointSize")]
        pub ref_point_size: f64,
        #[serde(rename = "infoPointSize")]
        pub info_point_size: f64,
        #[serde(rename = "gluePointSize")]
        pub glue_point_size: f64,
        #[serde(rename = "pickPointSize")]
        pub pick_point_size: f64,
        #[serde(rename = "testPointSize")]
        pub test_point_size: f64,
        #[serde(rename = "refPointSizePrint")]
        pub ref_point_size_print: f64,
        #[serde(rename = "infoPointSizePrint")]
        pub info_point_size_print: f64,
        #[serde(rename = "gluePointSizePrint")]
        pub glue_point_size_print: f64,
        #[serde(rename = "pickPointSizePrint")]
        pub pick_point_size_print: f64,
        #[serde(rename = "testPointSizePrint")]
        pub test_point_size_print: f64,
        #[serde(rename = "solderFlowDirection")]
        pub solder_flow_direction: SolderFlowDirection,
        #[serde(rename = "globalCopperPourCutoutBackoffFlag")]
        pub global_copper_pour_cutout_backoff_flag: bool,
    }

    impl PcadDeserialize for PcbDesignHeader {
        const TAG: &'static [u8] = b"pcbDesignHeader";
        const HAS_INTERNAL_NAME: bool = false;

        fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
            match field_name {
                b"workspaceSize" => {
                    let w = parser.parse_f64()?;
                    let h = parser.parse_f64()?;
                    self.workspace_size = (w, h);
                    parser.expect_token(TokenKind::CloseParen)?;
                    Ok(true)
                }
                b"gridDfns" => {
                    self.grid_dfns = parser.parse_common_struct::<GridDfns>(Wrapped::In)?;
                    Ok(true)
                }
                b"designInfo" => {
                    self.design_info = parser.parse_common_struct::<DesignInfo>(Wrapped::In)?;
                    Ok(true)
                }
                b"solderSwell" => {
                    self.solder_swell = parser.parse_field(|p| p.parse_f64())?;
                    Ok(true)
                }
                b"pasteSwell" => {
                    self.paste_swell = parser.parse_field(|p| p.parse_f64())?;
                    Ok(true)
                }
                b"planeSwell" => {
                    self.plane_swell = parser.parse_field(|p| p.parse_f64())?;
                    Ok(true)
                }
                b"refPointSize" => {
                    self.ref_point_size = parser.parse_field(|p| p.parse_f64())?;
                    Ok(true)
                }
                b"infoPointSize" => {
                    self.info_point_size = parser.parse_field(|p| p.parse_f64())?;
                    Ok(true)
                }
                b"gluePointSize" => {
                    self.glue_point_size = parser.parse_field(|p| p.parse_f64())?;
                    Ok(true)
                }
                b"pickPointSize" => {
                    self.pick_point_size = parser.parse_field(|p| p.parse_f64())?;
                    Ok(true)
                }
                b"testPointSize" => {
                    self.test_point_size = parser.parse_field(|p| p.parse_f64())?;
                    Ok(true)
                }
                b"refPointSizePrint" => {
                    self.ref_point_size_print = parser.parse_field(|p| p.parse_f64())?;
                    Ok(true)
                }
                b"infoPointSizePrint" => {
                    self.info_point_size_print = parser.parse_field(|p| p.parse_f64())?;
                    Ok(true)
                }
                b"gluePointSizePrint" => {
                    self.glue_point_size_print = parser.parse_field(|p| p.parse_f64())?;
                    Ok(true)
                }
                b"pickPointSizePrint" => {
                    self.pick_point_size_print = parser.parse_field(|p| p.parse_f64())?;
                    Ok(true)
                }
                b"testPointSizePrint" => {
                    self.test_point_size_print = parser.parse_field(|p| p.parse_f64())?;
                    Ok(true)
                }
                b"solderFlowDirection" => {
                    let dir_str = parser.parse_field(|p| p.parse_string())?;
                    self.solder_flow_direction = match dir_str.as_str() {
                        "solderFlowTopToBottom" => SolderFlowDirection::SolderFlowTopToBottom,
                        _ => return Err(ParserError::GeneralError("Unknown solder flow direction")),
                    };
                    Ok(true)
                }
                b"globalCopperPourCutoutBackoffFlag" => {
                    self.global_copper_pour_cutout_backoff_flag = parser.parse_field(|p| p.parse_bool())?;
                    Ok(true)
                }
                _ => Ok(false),
            }
        }
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct GridDfns {
        pub grids: Vec<String>,
    }

    impl PcadDeserialize for GridDfns {
        const TAG: &'static [u8] = b"gridDfns";
        const HAS_INTERNAL_NAME: bool = false;

        fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
            match field_name {
                b"grid" => {
                    let grid = parser.parse_field(|p| Ok(p.parse_string()?))?;
                    self.grids.push(grid);
                    Ok(true)
                }
                _ => Ok(false),
            }
        }
    }


    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct DesignInfo {
        #[serde(rename = "fieldSet")]
        pub field_set: FieldSet,
    }

    impl PcadDeserialize for DesignInfo {
        const TAG: &'static [u8] = b"designInfo";
        const HAS_INTERNAL_NAME: bool = false;

        fn load_internal_name(&mut self, _name: &[u8]) {}

        fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
            match field_name {
                b"fieldSet" => {
                    self.field_set = parser.parse_common_struct::<FieldSet>(Wrapped::In)?;
                    Ok(true)
                }
                _ => Ok(false),
            }
        }
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct FieldSet {
        pub name: String,
        #[serde(rename = "fieldDefs")]
        pub field_defs: Vec<FieldDef>,
    }

    impl PcadDeserialize for FieldSet {
        const TAG: &'static [u8] = b"fieldSet";
        const HAS_INTERNAL_NAME: bool = true;

        fn load_internal_name(&mut self, name: &[u8]) {
            self.name = String::from_utf8_lossy(name).into_owned();
        }

        fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
            match field_name {
                b"fieldDef" => {
                    let name = parser.parse_string()?;
                    let value = parser.parse_string()?;
                    parser.expect_token(TokenKind::CloseParen)?;
                    self.field_defs.push(FieldDef { name, value });
                    Ok(true)
                }
                _ => Ok(false)
            }
        }
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct FieldDef {
        pub name: String,
        pub value: String,
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub enum SolderFlowDirection {
        #[default]
        SolderFlowTopToBottom,
        // TODO: add other variants
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct LayerDef {
        pub name: String,
        #[serde(rename = "layerNum")]
        pub layer_num: i32,
        #[serde(rename = "layerType")]
        pub layer_type: LayerType,
        #[serde(rename = "attributes")]
        pub attributes: Vec<Attr>,
        #[serde(rename = "fieldSetRef")]
        pub field_set_ref: String,
    }

    impl PcadDeserialize for LayerDef {
        const TAG: &'static [u8] = b"layerDef";
        const HAS_INTERNAL_NAME: bool = true;

        fn load_internal_name(&mut self, name: &[u8]) {
            self.name = String::from_utf8_lossy(name).into_owned();
        }

        fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
            match field_name {
                b"layerNum" => {
                    self.layer_num = parser.parse_field(|p| p.parse_i32())?;
                    Ok(true)
                }
                b"layerType" => {
                    let layer_type_str = parser.parse_field(|p| p.parse_string())?;
                    self.layer_type = match layer_type_str.as_str() {
                        "Signal" => LayerType::Signal,
                        "NonSignal" => LayerType::NonSignal,
                        "Plane" => LayerType::Plane,
                        _ => return Err(ParserError::GeneralError("Unknown layer type")),
                    };
                    Ok(true)
                }
                b"attr" => {
                    let attr = parser.parse_attr_struct(Wrapped::In)?;
                    self.attributes.push(attr);
                    Ok(true)
                }
                b"fieldSetRef" => {
                    self.field_set_ref = parser.parse_field(|p| p.parse_string())?;
                    Ok(true)
                }
                _ => Ok(false),
            }
        }
    }

    #[derive(Debug, Default, Serialize, Deserialize)]
    pub enum LayerType {
        #[default]
        Signal,
        NonSignal,
        Plane
    }


    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct Attr {
        pub label: String,           
        pub name: String,          

        #[serde(rename = "pt", skip_serializing_if = "Option::is_none")]
        pub point: Option<(f64, f64)>,
        #[serde(rename = "textStyleRef", skip_serializing_if = "Option::is_none")]
        pub text_style_ref: Option<String>,
        #[serde(rename = "isFlipped", skip_serializing_if = "Option::is_none")]
        pub is_flipped: Option<bool>,
        #[serde(rename = "rotation", skip_serializing_if = "Option::is_none")]
        pub rotation: Option<f64>,
        #[serde(rename = "constraintUnits", skip_serializing_if = "Option::is_none")]
        pub constraint_units: Option<ConstraintUnits>,
    }

    impl PcadDeserialize for Attr {
        const TAG: &'static [u8] = b"attr";
        const HAS_INTERNAL_NAME: bool = false;

        fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
            match field_name {
            b"pt" => {
                let x = parser.parse_f64()?;
                let y = parser.parse_f64()?;
                self.point = Some((x, y));
                parser.expect_token(TokenKind::CloseParen)?;
                Ok(true)
            }
            b"textStyleRef" => {
                self.text_style_ref = Some(parser.parse_field(|p| p.parse_string())?);
                Ok(true)
            }
            b"isFlipped" => {
                self.is_flipped = Some(parser.parse_field(|p| p.parse_bool())?);
                Ok(true)
            }
            b"rotation" => { 
                self.rotation = Some(parser.parse_field(|p| p.parse_f64())?);
                Ok(true)
            }
            b"constraintUnits" => {
                let units_str = parser.parse_field(|p| p.parse_string())?;
                self.constraint_units = Some(match units_str.as_str() {
                    "mil" => ConstraintUnits::Mil,
                    "mm" => ConstraintUnits::Mm,
                    _ => return Err(ParserError::GeneralError("Unknown constraint units")),
                });
                Ok(true)
            }
            _ => {
                Ok(false)
            }
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum ConstraintUnits {
    #[default]
    Mil,
    Mm
    // TODO: add other variants 
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MultiLayer {
    pub vias: Vec<Via>,
    pub patterns: Vec<Pattern>,
    #[serde(rename = "fromTos")]
    pub from_tos: Vec<FromTo>,
}

impl PcadDeserialize for MultiLayer {
    const TAG: &'static [u8] = b"multiLayer";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_internal_name(&mut self, _name: &[u8]) {}

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"via" => {
                let via = parser.parse_common_struct::<Via>(Wrapped::In)?;
                self.vias.push(via);
                Ok(true)
            }
            b"pattern" => {
                let pattern = parser.parse_common_struct::<Pattern>(Wrapped::In)?;
                self.patterns.push(pattern);
                Ok(true)
            }
            b"fromTo" => {
                let from_to = parser.parse_common_struct::<FromTo>(Wrapped::In)?;
                self.from_tos.push(from_to);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Via {
    #[serde(rename = "viaStyleRef")]
    pub via_style_ref: String,
    #[serde(rename = "pt")]
    pub point: (f64, f64),
    #[serde(rename = "netNameRef")]
    pub net_name_ref: String,
}

impl PcadDeserialize for Via {
    const TAG: &'static [u8] = b"via";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"viaStyleRef" => {
                self.via_style_ref = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"pt" => {
                self.point = parser.parse_field(|p| {
                    let x = p.parse_f64()?;
                    let y = p.parse_f64()?;
                    Ok((x, y))
                })?;
                Ok(true)
            }
            b"netNameRef" => {
                self.net_name_ref = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Pattern {
    #[serde(rename = "patternRef")]
    pub pattern_ref: String,
    #[serde(rename = "refDesRef")]
    pub ref_des_ref: String,
    #[serde(rename = "pt")]
    pub point: (f64, f64),
    pub rotation: Option<f64>,
    #[serde(rename = "isFlipped", default)]
    pub is_flipped: bool,
    #[serde(rename = "PatternGraphicsNameRef")]
    pub pattern_graphics_name_ref: String,
    #[serde(rename = "patternGraphicsRef")]
    pub pattern_graphics_ref: PatternGraphicsRef,
}

impl PcadDeserialize for Pattern {
    const TAG: &'static [u8] = b"pattern";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"patternRef" => {
                self.pattern_ref = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"refDesRef" => {
                self.ref_des_ref = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"pt" => {
                self.point = parser.parse_field(|p| {
                    let x = p.parse_f64()?;
                    let y = p.parse_f64()?;
                    Ok((x, y))
                })?;
                Ok(true)
            }
            b"rotation" => {
                self.rotation = Some(parser.parse_field(|p| p.parse_f64())?);
                Ok(true)
            }
            b"isFlipped" => {
                self.is_flipped = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"patternGraphicsNameRef" => {
                self.pattern_graphics_name_ref = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"patternGraphicsRef" => {
                self.pattern_graphics_ref = parser.parse_common_struct::<PatternGraphicsRef>(Wrapped::In)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PatternGraphicsRef {
    #[serde(rename = "PatternGraphicsNameRef")]
    pub pattern_graphics_name_ref: String,
    pub attributes: Vec<Attr>,
}

impl PcadDeserialize for PatternGraphicsRef {
    const TAG: &'static [u8] = b"patternGraphicsRef";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"patternGraphicsNameRef" => {
                self.pattern_graphics_name_ref = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"attr" => {
                let attr = parser.parse_attr_struct(Wrapped::In)?;
                self.attributes.push(attr);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FromTo {
    #[serde(rename = "netNameRef")]
    pub net_name_ref: String,
    #[serde(rename = "pt")]
    pub points: Vec<(f64, f64)>,
}

impl PcadDeserialize for FromTo {
    const TAG: &'static [u8] = b"fromTo";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"netNameRef" => {
                self.net_name_ref = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"pt" => {
                let point = parser.parse_field(|p| {
                    let x = p.parse_f64()?;
                    let y = p.parse_f64()?;
                    Ok((x, y))
                })?;

                self.points.push(point);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PolyKeepOut {
    #[serde(rename = "pcbPoly")]
    pub pcb_poly: PcbPoly,
}

impl PcadDeserialize for PolyKeepOut {
    const TAG: &'static [u8] = b"polyKeepOut";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"pcbPoly" => {
                self.pcb_poly = parser.parse_common_struct::<PcbPoly>(Wrapped::In)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LayerContents {
    #[serde(rename = "layerRef")]
    pub layer_ref: LayerRef,

    pub lines: Vec<Line>,
    #[serde(rename = "copperPours")]
    pub copper_pours: Vec<CopperPour>,
    pub dimensions: Vec<Dimension>,

    pub texts: Vec<Attr>,           
    #[serde(rename = "polyKeepOuts")]
    pub poly_keep_outs: Vec<PolyKeepOut>, 
    pub arcs: Vec<TriplePointArc>,  
    // TODO: add other fields
}


impl PcadDeserialize for LayerContents {
    const TAG: &'static [u8] = b"layerContents";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        if field_name == b"layerNumRef" {
            let layer_num = parser.parse_u32()?;
            self.layer_ref = LayerRef::Num(layer_num);
            parser.expect_token(TokenKind::CloseParen)?;
            Ok(true)
        } else if field_name == b"layerType" {
            let layer_type_str = parser.parse_string()?;
            let layer_type = match layer_type_str.as_str() {
                "Signal" => LayerType::Signal,
                "NonSignal" => LayerType::NonSignal,
                "Plane" => LayerType::Plane,
                _ => return Err(ParserError::GeneralError("Unknown layer type")),
            };
            self.layer_ref = LayerRef::Type(layer_type);
            parser.expect_token(TokenKind::CloseParen)?;
            Ok(true)
        } else {
            match field_name {
                b"line" => {
                    let line = parser.parse_common_struct::<Line>(Wrapped::In)?;
                    self.lines.push(line);
                    Ok(true)
                }
                b"copperPour95" => {
                    let copper_pour = parser.parse_common_struct::<CopperPour>(Wrapped::In)?;
                    self.copper_pours.push(copper_pour);
                    Ok(true)
                }
                b"dimension" => {
                    let dimension = parser.parse_common_struct::<Dimension>(Wrapped::In)?;
                    self.dimensions.push(dimension);
                    Ok(true)
                }
                b"attr" => {
                    let attr = parser.parse_attr_struct(Wrapped::In)?;
                    self.texts.push(attr);
                    Ok(true)
                }
                b"polyKeepOut" => {
                    let poly_keep_out = parser.parse_common_struct::<PolyKeepOut>(Wrapped::In)?;
                    self.poly_keep_outs.push(poly_keep_out);
                    Ok(true)
                }
                b"triplePointArc" => {
                    let arc = parser.parse_common_struct::<TriplePointArc>(Wrapped::In)?;
                    self.arcs.push(arc);
                    Ok(true)
                }
                _ => Ok(false),
            }
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TriplePointArc {
    #[serde(rename = "startPoint")]
    pub start_point: (f64, f64),
    #[serde(rename = "midPoint")]
    pub mid_point: (f64, f64),
    #[serde(rename = "endPoint")]
    pub end_point: (f64, f64),
    pub width: f64,
}

impl PcadDeserialize for TriplePointArc {
    const TAG: &'static [u8] = b"triplePointArc";
    const HAS_INTERNAL_NAME: bool = false;


    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"pt" => {
                let x = parser.parse_f64()?;
                let y = parser.parse_f64()?;
                parser.expect_token(TokenKind::CloseParen)?;

                if self.start_point == (0.0, 0.0) {
                    self.start_point = (x, y);
                }

                else if self.mid_point == (0.0, 0.0) {
                    self.mid_point = (x, y);
                }

                else {
                    self.end_point = (x, y);
                }
                Ok(true)
            }
            b"width" => {
                self.width = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Line {
    pub start: (f64, f64),
    pub end: (f64, f64),
    pub width: f64,
    #[serde(rename = "netNameRef")]
    pub net_name_ref: String,
    #[serde(rename = "dimensionRefs", default)]
    pub dimension_refs: Vec<(i32, i32)> 
}

impl PcadDeserialize for Line {
    const TAG: &'static [u8] = b"line";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"pt" => {
                let x = parser.parse_f64()?;
                let y = parser.parse_f64()?;
                parser.expect_token(TokenKind::CloseParen)?;

                if self.start == (0.0, 0.0) {
                    self.start = (x, y);
                } else {
                    self.end = (x, y);
                }
                Ok(true)
            }
            b"width" => {
                self.width = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"netNameRef" => {
                self.net_name_ref = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"dimensionRef" => {
                let dim1 = parser.parse_i32()?;
                let dim2 = parser.parse_i32()?;
                self.dimension_refs.push((dim1, dim2));
                parser.expect_token(TokenKind::CloseParen)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CopperPour {
    #[serde(rename = "pourType")]
    pub pour_type: PourType,
    #[serde(rename = "netNameRef")]
    pub net_name_ref: String,
    pub width: f64,
    #[serde(rename = "pourSpacing")]
    pub pour_spacing: f64,
    #[serde(rename = "pourBackoff")]
    pub pour_backoff: f64,
    #[serde(rename = "useDesignRules")]
    pub use_design_rules: bool,
    #[serde(rename = "pourSmoothness")]
    pub pour_smoothness: i32,
    #[serde(rename = "thermalType")]
    pub thermal_type: ThermalType,
    #[serde(rename = "thermalWidth")]
    pub thermal_width: f64,
    #[serde(rename = "thermalSpokes")]
    pub thermal_spokes: i32,
    #[serde(rename = "islandRemoval")]
    pub island_removal: IslandRemoval,
    #[serde(rename = "viaThermalType")]
    pub via_thermal_type: ThermalType,
    #[serde(rename = "viaThermalWidth")]
    pub via_thermal_width: f64,
    #[serde(rename = "viaThermalSpokes")]
    pub via_thermal_spokes: i32,
    #[serde(rename = "pcbPoly")]
    pub pcb_poly: PcbPoly,
    pub islands: Vec<Island>,
}

impl PcadDeserialize for CopperPour {
    const TAG: &'static [u8] = b"copperPour";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"pourType" => {
                let pour_type_str = parser.parse_field(|p| p.parse_string())?;
                self.pour_type = match pour_type_str.as_str() {
                    "SolidPour" => PourType::SolidPour,
                    "Hatch" => PourType::Hatch,
                    _ => return Err(ParserError::GeneralError("Unknown pour type")),
                };
                Ok(true)
            }
            b"netNameRef" => {
                self.net_name_ref = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"width" => {
                self.width = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"pourSpacing" => {
                self.pour_spacing = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"pourBackoff" => {
                self.pour_backoff = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"useDesignRules" => {
                self.use_design_rules = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"pourSmoothness" => {
                self.pour_smoothness = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"thermalType" => {
                let thermal_type_str = parser.parse_field(|p| p.parse_string())?;
                self.thermal_type = match thermal_type_str.as_str() {
                    "Therm45" => ThermalType::Therm45,
                    _ => return Err(ParserError::GeneralError("Unknown thermal type")),
                };
                Ok(true)
            }
            b"thermalWidth" => {
                self.thermal_width = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"thermalSpokes" => {
                self.thermal_spokes = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"islandRemoval" => {
                self.island_removal = parser.parse_common_struct::<IslandRemoval>(Wrapped::In)?;
                Ok(true)
            }
            b"viaThermalType" => {
                let via_thermal_type_str = parser.parse_field(|p| p.parse_string())?;
                self.via_thermal_type = match via_thermal_type_str.as_str() {
                    "Therm45" => ThermalType::Therm45,
                    _ => return Err(ParserError::GeneralError("Unknown via thermal type")),
                };
                Ok(true)
            }
            b"viaThermalWidth" => {
                self.via_thermal_width = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"viaThermalSpokes" => {
                self.via_thermal_spokes = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"pcbPoly" => {
                self.pcb_poly = parser.parse_common_struct::<PcbPoly>(Wrapped::In)?;
                Ok(true)
            }
            b"island" => {
                let island = parser.parse_common_struct::<Island>(Wrapped::In)?;
                self.islands.push(island);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum PourType {
    #[default]
    SolidPour,
    Hatch
    // TODO: add other variants 
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum ThermalType {
    #[default]
    Therm45,
    // TODO: add other variants
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct IslandRemoval {
    pub area: f64,
    pub condition: IslandRemovalCondition,
}

impl PcadDeserialize for IslandRemoval {
    const TAG: &'static [u8] = b"islandRemoval";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        if field_name == b"area" {
            self.area = parser.parse_f64()?;
            parser.expect_token(TokenKind::CloseParen)?;

            let cond = parser.parse_string()?;
            self.condition = match cond.as_str() {
                "unconnected" => IslandRemovalCondition::Unconnected,
                _ => return Err(ParserError::GeneralError("Unknown condition")),
            };
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum IslandRemovalCondition {
    #[default]
    Unconnected,
    // TODO: add other variants
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PcbPoly {
    pub points: Vec<(f64, f64)>,
    #[serde(rename = "netNameRef")]
    pub net_name_ref: String,
}

impl PcadDeserialize for PcbPoly {
    const TAG: &'static [u8] = b"pcbPoly";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"pt" => {
                let x = parser.parse_f64()?;
                let y = parser.parse_f64()?;
                self.points.push((x, y));
                parser.expect_token(TokenKind::CloseParen)?;
                Ok(true)
            }
            b"netNameRef" => {
                self.net_name_ref = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Island {
    #[serde(rename = "islandOutline")]
    pub island_outline: Vec<(f64, f64)>,
    pub thermals: Vec<Thermal>,
    #[serde(rename = "cutout", default)]
    pub cutout: Option<Cutout>
}

impl PcadDeserialize for Island {
    const TAG: &'static [u8] = b"island";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"islandOutline" => {
                loop {
                    if matches!(parser.peek_token_type(), Some(TokenType::CloseParen)) {
                        parser.expect_token(TokenKind::CloseParen)?;
                        break;
                    }

                    parser.expect_token(TokenKind::OpenParen)?;
                    let pt_name = parser.next_token()?;
                    if let TokenType::Symbol(bytes) = &pt_name.token {
                        if bytes == b"pt" {
                            let x = parser.parse_f64()?;
                            let y = parser.parse_f64()?;
                            self.island_outline.push((x, y));
                            parser.expect_token(TokenKind::CloseParen)?;
                        } else {
                            parser.skip_field_value()?;
                        }
                    } else {
                        parser.skip_field_value()?;
                    }
                }
                Ok(true)
            }
            b"thermal" => {
                let thermal = parser.parse_common_struct::<Thermal>(Wrapped::In)?;
                self.thermals.push(thermal);
                Ok(true)
            }
            b"cutout" => {
                let cutout = parser.parse_common_struct::<Cutout>(Wrapped::In)?;
                self.cutout = Some(cutout);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Cutout {
    #[serde(rename = "cutoutOutline")]
    pub cutout_outline: Vec<(f64, f64)>
}

impl PcadDeserialize for Cutout {
    const TAG: &'static [u8] = b"cutout";
    const HAS_INTERNAL_NAME: bool = false;
    
    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"cutoutOutline" => {
                loop {
                    if matches!(parser.peek_token_type(), Some(TokenType::CloseParen)) {
                        parser.expect_token(TokenKind::CloseParen)?;
                        break;
                    }
                    
                    parser.expect_token(TokenKind::OpenParen)?;
                    let pt_name = parser.next_token()?;
                    if let TokenType::Symbol(bytes) = &pt_name.token {
                        if bytes == b"pt" {
                            let x = parser.parse_f64()?;
                            let y = parser.parse_f64()?;
                            self.cutout_outline.push((x, y));
                            parser.expect_token(TokenKind::CloseParen)?;
                        } else {
                            parser.skip_field_value()?;
                        }
                    } else {
                        parser.skip_field_value()?;
                    }
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Thermal {
    pub start: (f64, f64),
    pub end: (f64, f64),
    #[serde(rename = "thermalWidth")]
    pub thermal_width: f64,
}

impl PcadDeserialize for Thermal {
    const TAG: &'static [u8] = b"thermal";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"pt" => {
                let x = parser.parse_f64()?;
                let y = parser.parse_f64()?;
                parser.expect_token(TokenKind::CloseParen)?;

                if self.start == (0.0, 0.0) {
                    self.start = (x, y);
                } else {
                    self.end = (x, y);
                }
                Ok(true)
            }
            b"thermalWidth" => {
                self.thermal_width = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(rename = "dimStyle")]
    pub dim_style: DimStyles,
    #[serde(rename = "pt")]
    pub point: (f64, f64),
    pub rotation: f64,
    #[serde(rename = "isFlipped", default)]
    pub is_flipped: bool,
    #[serde(rename = "dimOrient")]
    pub dim_orient: DimOrient,
    #[serde(rename = "dimTextOrient")]
    pub dim_text_orient: DimTextOrient,
    #[serde(rename = "dimPrecision")]
    pub dim_precision: i32,
    #[serde(rename = "dimDisplayUnits")]
    pub dim_display_units: bool,
    #[serde(rename = "dimUnits")]
    pub dim_units: DimUnits,
    #[serde(rename = "dimLineWidth")]
    pub dim_line_width: i32,
    #[serde(rename = "dimLeaderStyle")]
    pub dim_leader_style: DimLeaderStyle,
    #[serde(rename = "dimLeaderSize")]
    pub dim_leader_size: i32,
    #[serde(rename = "dimCenterSize")]
    pub dim_center_size: i32,
    #[serde(rename = "dimPlusLinearTol")]
    pub dim_plus_linear_tol: i32,
    #[serde(rename = "dimMinusLinearTol")]
    pub dim_minus_linear_tol: i32,
    #[serde(rename = "dimPlusDegTol")]
    pub dim_plus_deg_tol: i32,
    #[serde(rename = "dimMinusDegTol")]
    pub dim_minus_deg_tol: i32,
    #[serde(rename = "dimShowTolerance")]
    pub dim_show_tolerance: bool,
    #[serde(rename = "dimShowDiaSymbol")]
    pub dim_show_dia_symbol: bool,
    #[serde(rename = "dimDimLineGraphics")]
    pub dim_dim_line_graphics: DimDimLineGraphics,
    #[serde(rename = "dimExtLineGraphics")]
    pub dim_ext_line_graphics: DimExtLineGraphics,
    #[serde(rename = "dimTextGraphics")]
    pub dim_text_graphics: DimTextGraphics,
    #[serde(rename = "dimPoints")]
    pub dim_points: DimPoints,
    #[serde(rename = "arrowheadWidth")]
    pub arrowhead_width: f64,
    #[serde(rename = "arrowheadHeight")]
    pub arrowhead_height: f64,
    #[serde(rename = "dimensionId")]
    pub dimension_id: i32,
    #[serde(rename = "dimensionOffsets")]
    pub dimension_offsets: Vec<(f64, f64)>,
}

impl PcadDeserialize for Dimension {
    const TAG: &'static [u8] = b"dimension";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"dimStyle" => {
                let style_str = parser.parse_field(|p| p.parse_string())?;
                self.dim_style = match style_str.as_str() {
                    "dim_point_to_point" => DimStyles::dimPointToPoint,
                    _ => return Err(ParserError::GeneralError("Unknown dimension style")),
                };
                Ok(true)
            }
            b"pt" => {
                let x = parser.parse_f64()?;
                let y = parser.parse_f64()?;
                self.point = (x, y);
                parser.expect_token(TokenKind::CloseParen)?;
                Ok(true)
            }
            b"rotation" => {
                self.rotation = parser.parse_field(|p| p.parse_number64())?;
                Ok(true)
            }
            b"isFlipped" => {
                self.is_flipped = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"dimOrient" => {
                let orient_str = parser.parse_field(|p| p.parse_string())?;
                self.dim_orient = match orient_str.as_str() {
                    "dim_vertical" => DimOrient::DimVertical,
                    "dim_horizontal" => DimOrient::DimHorizontal,
                    _ => return Err(ParserError::GeneralError("Unknown dimension orientation")),
                };
                Ok(true)
            }
            b"dimTextOrient" => {
                let orient_str = parser.parse_field(|p| p.parse_string())?;
                self.dim_text_orient = match orient_str.as_str() {
                    "dim_vertical" => DimTextOrient::DimVertical,
                    "dim_horizontal" => DimTextOrient::DimHorizontal,
                    _ => return Err(ParserError::GeneralError("Unknown dimension text orientation")),
                };
                Ok(true)
            }
            b"dimPrecision" => {
                self.dim_precision = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"dimDisplayUnits" => {
                self.dim_display_units = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"dimUnits" => {
                let units_str = parser.parse_field(|p| p.parse_string())?;
                self.dim_units = match units_str.as_str() {
                    "dim_units_cm" => DimUnits::DimUnitsCm,
                    _ => return Err(ParserError::GeneralError("Unknown dimension units")),
                };
                Ok(true)
            }
            b"dimLineWidth" => {
                self.dim_line_width = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"dimLeaderStyle" => {
                let style_str = parser.parse_field(|p| p.parse_string())?;
                self.dim_leader_style = match style_str.as_str() {
                    "dim_leader_style_text" => DimLeaderStyle::DimLeaderStyleText,
                    _ => return Err(ParserError::GeneralError("Unknown dimension leader style")),
                };
                Ok(true)
            }
            b"dimLeaderSize" => {
                self.dim_leader_size = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"dimCenterSize" => {
                self.dim_center_size = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"dimPlusLinearTol" => {
                self.dim_plus_linear_tol = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"dimMinusLinearTol" => {
                self.dim_minus_linear_tol = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"dimPlusDegTol" => {
                self.dim_plus_deg_tol = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"dimMinusDegTol" => {
                self.dim_minus_deg_tol = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"dimShowTolerance" => {
                self.dim_show_tolerance = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"dimShowDiaSymbol" => {
                self.dim_show_dia_symbol = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"dimDimLineGraphics" => {
                self.dim_dim_line_graphics = parser.parse_common_struct::<DimDimLineGraphics>(Wrapped::In)?;
                Ok(true)
            }
            b"dimExtLineGraphics" => {
                self.dim_ext_line_graphics = parser.parse_common_struct::<DimExtLineGraphics>(Wrapped::In)?;
                Ok(true)
            }
            b"dimTextGraphics" => {
                self.dim_text_graphics = parser.parse_common_struct::<DimTextGraphics>(Wrapped::In)?;
                Ok(true)
            }
            b"dimPoints" => {
                self.dim_points = parser.parse_common_struct::<DimPoints>(Wrapped::In)?;
                Ok(true)
            }
            b"arrowheadWidth" => {
                self.arrowhead_width = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"arrowheadHeight" => {
                self.arrowhead_height = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"dimensionID" => {
                self.dimension_id = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"dimensionOffsets" => {
                loop {
                    if matches!(parser.peek_token_type(), Some(TokenType::CloseParen)) {
                        parser.expect_token(TokenKind::CloseParen)?;
                        break;
                    }
                    parser.expect_token(TokenKind::OpenParen)?;
                    let pt_name = parser.next_token()?;
                    if let TokenType::Symbol(pt_bytes) = &pt_name.token {
                        if pt_bytes == b"pt" {
                            let x = parser.parse_f64()?;
                            let y = parser.parse_f64()?;
                            self.dimension_offsets.push((x, y));
                            parser.expect_token(TokenKind::CloseParen)?;
                        } else {
                            parser.skip_field_value()?;
                        }
                    } else {
                        parser.skip_field_value()?;
                    }
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DimDimLineGraphics {
    #[serde(rename = "dimGraphics")]
    pub dim_graphics: Vec<DimGraphic>,
}

impl PcadDeserialize for DimDimLineGraphics {
    const TAG: &'static [u8] = b"dimDimLineGraphics";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        if field_name == b"dimGraphics" {
            let graphics = parser.parse_common_struct::<DimGraphics>(Wrapped::In)?;
            self.dim_graphics = graphics.dim_graphics;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DimExtLineGraphics {
    #[serde(rename = "dimGraphics")]
    pub dim_graphics: Vec<DimGraphic>,
}

impl PcadDeserialize for DimExtLineGraphics {
    const TAG: &'static [u8] = b"dimExtLineGraphics";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        if field_name == b"dimGraphics" {
            let graphics = parser.parse_common_struct::<DimGraphics>(Wrapped::In)?;
            self.dim_graphics = graphics.dim_graphics;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DimTextGraphics {
    #[serde(rename = "dimGraphics")]
    pub dim_graphics: Vec<DimGraphic>,
}

impl PcadDeserialize for DimTextGraphics {
    const TAG: &'static [u8] = b"dimTextGraphics";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        if field_name == b"dimGraphics" {
            let graphics = parser.parse_common_struct::<DimGraphics>(Wrapped::In)?;
            self.dim_graphics = graphics.dim_graphics;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum DimStyles {
    #[default]
    dimPointToPoint
    // TODO: add other variants
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DimGraphics {
    #[serde(rename = "dimGraphics")]
    pub dim_graphics: Vec<DimGraphic>,
}

impl PcadDeserialize for DimGraphics {
    const TAG: &'static [u8] = b"dimGraphics";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        if field_name == b"dimGraphic" {
            parser.expect_token(TokenKind::OpenParen)?;
            let graphic_type = parser.next_token()?;

            match &graphic_type.token {
                TokenType::Symbol(bytes) => {
                    match bytes.as_slice() {
                        b"line" => {
                            parser.expect_token(TokenKind::OpenParen)?;
                            parser.expect_token_bytes(b"pt")?;
                            let start_x = parser.parse_f64()?;
                            let start_y = parser.parse_f64()?;
                            parser.expect_token(TokenKind::CloseParen)?;

                            parser.expect_token(TokenKind::OpenParen)?;
                            parser.expect_token_bytes(b"pt")?;
                            let end_x = parser.parse_f64()?;
                            let end_y = parser.parse_f64()?;
                            parser.expect_token(TokenKind::CloseParen)?;

                            parser.expect_token(TokenKind::OpenParen)?;
                            parser.expect_token_bytes(b"width")?;
                            let width = parser.parse_f64()?;
                            parser.expect_token(TokenKind::CloseParen)?;

                            parser.expect_token(TokenKind::CloseParen)?; 
                            parser.expect_token(TokenKind::CloseParen)?; 

                            self.dim_graphics.push(DimGraphic::Line {
                                start: (start_x, start_y),
                                end: (end_x, end_y),
                                width,
                            });
                        }
                        b"poly" => {
                            let mut points = Vec::new();
                            loop {
                                if matches!(parser.peek_token_type(), Some(TokenType::CloseParen)) {
                                    parser.expect_token(TokenKind::CloseParen)?;
                                    break;
                                }
                                parser.expect_token(TokenKind::OpenParen)?;
                                let pt_name = parser.next_token()?;
                                if let TokenType::Symbol(pt_bytes) = &pt_name.token {
                                    if pt_bytes == b"pt" {
                                        let x = parser.parse_f64()?;
                                        let y = parser.parse_f64()?;
                                        points.push((x, y));
                                        parser.expect_token(TokenKind::CloseParen)?;
                                    } else {
                                        parser.skip_field_value()?;
                                    }
                                } else {
                                    parser.skip_field_value()?;
                                }
                            }
                            parser.expect_token(TokenKind::CloseParen)?; 
                            self.dim_graphics.push(DimGraphic::Poly { points });
                        }
                        b"text" => {
                            parser.expect_token(TokenKind::OpenParen)?;
                            parser.expect_token_bytes(b"pt")?;
                            let point_x = parser.parse_f64()?;
                            let point_y = parser.parse_f64()?;
                            parser.expect_token(TokenKind::CloseParen)?;

                            let text = parser.parse_string()?;

                            let mut text_style_ref = String::new();
                            let mut rotation = None;
                            let mut justify = TextJustify::Center;
                            let mut extent = (0.0, 0.0);

                            loop {
                                if matches!(parser.peek_token_type(), Some(TokenType::CloseParen)) {
                                    parser.expect_token(TokenKind::CloseParen)?;
                                    break;
                                }
                                parser.expect_token(TokenKind::OpenParen)?;
                                let field_name = parser.next_token()?;
                                if let TokenType::Symbol(field_bytes) = &field_name.token {
                                    match field_bytes.as_slice() {
                                        b"textStyleRef" => {
                                            text_style_ref = parser.parse_string()?;
                                            parser.expect_token(TokenKind::CloseParen)?;
                                        }
                                        b"rotation" => {
                                            rotation = Some(parser.parse_f64()?);
                                            parser.expect_token(TokenKind::CloseParen)?;
                                        }
                                        b"justify" => {
                                            let justify_str = parser.parse_string()?;
                                            justify = match justify_str.as_str() {
                                                "Center" => TextJustify::Center,
                                                _ => TextJustify::Center,
                                            };
                                            parser.expect_token(TokenKind::CloseParen)?;
                                        }
                                        b"extent" => {
                                            let w = parser.parse_f64()?;
                                            let h = parser.parse_f64()?;
                                            extent = (w, h);
                                            parser.expect_token(TokenKind::CloseParen)?;
                                        }
                                        _ => {
                                            parser.skip_field_value()?;
                                        }
                                    }
                                } else {
                                    parser.skip_field_value()?;
                                }
                            }

                            parser.expect_token(TokenKind::CloseParen)?; 
                            self.dim_graphics.push(DimGraphic::Text {
                                point: (point_x, point_y),
                                text,
                                text_style_ref,
                                rotation,
                                justify,
                                extent,
                            });
                        }
                        _ => {
                            parser.skip_field_value()?;
                            parser.expect_token(TokenKind::CloseParen)?;
                        }
                    }
                }
                _ => {
                    parser.skip_field_value()?;
                    parser.expect_token(TokenKind::CloseParen)?;
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DimGraphic {
    Line {
        start: (f64, f64),
        end: (f64, f64),
        width: f64,
    },
    Poly {
        points: Vec<(f64, f64)>,
    },
    Text {
        point: (f64, f64),
        text: String,
        #[serde(rename = "textStyleRef")]
        text_style_ref: String,
        rotation: Option<f64>,
        justify: TextJustify,
        extent: (f64, f64),
    },
}

impl Default for DimGraphic {
    fn default() -> Self {
        DimGraphic::Line {
            start: (0.0, 0.0),
            end: (0.0, 0.0),
            width: 0.0
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DimPoints {
    pub pt1: (f64, f64),
    pub pt2: (f64, f64),
    #[serde(rename = "textPt")]
    pub text_pt: (f64, f64),
}

impl PcadDeserialize for DimPoints {
    const TAG: &'static [u8] = b"dimPoints";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"pt" => {
                let x = parser.parse_f64()?;
                let y = parser.parse_f64()?;
                parser.expect_token(TokenKind::CloseParen)?;

                if self.pt1 == (0.0, 0.0) {
                    self.pt1 = (x, y);
                } else if self.pt2 == (0.0, 0.0) {
                    self.pt2 = (x, y);
                } else {
                    self.text_pt = (x, y);
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum DimOrient {
    #[default]
    DimVertical,
    DimHorizontal,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum DimTextOrient {
    #[default]
    DimVertical,
    DimHorizontal,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum DimUnits {
    #[default]
    DimUnitsCm,
    // TODO: add other variants
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum DimLeaderStyle {
    #[default]
    DimLeaderStyleText,
    // TODO: add other variants
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum TextJustify {
    #[default]
    Center,
    // TODO: add other variants
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PcbPrintSettings {
    #[serde(rename = "printQueueEntries")]
    pub print_queue_entries: Vec<PrintQueueEntry>,
}

impl PcadDeserialize for PcbPrintSettings {
    const TAG: &'static [u8] = b"pcbPrintSettings";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"printQueueEntry" => {  
                let entry = parser.parse_common_struct::<PrintQueueEntry>(Wrapped::In)?;
                self.print_queue_entries.push(entry);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PrintQueueEntry {
    pub id: String,
    #[serde(rename = "orderedLayerList")]
    pub ordered_layer_list: Vec<i32>,
    #[serde(rename = "isThinStrokeText")]
    pub is_thin_stroke_text: bool,
    #[serde(rename = "isSelected")]
    pub is_selected: bool,
    #[serde(rename = "drillSymSize")]
    pub drill_sym_size: f64,
    #[serde(rename = "outputMirror")]
    pub output_mirror: bool,
    #[serde(rename = "outputPads")]
    pub output_pads: bool,
    #[serde(rename = "outputVias")]
    pub output_vias: bool,
    #[serde(rename = "outputHoles")]
    pub output_holes: bool,
    #[serde(rename = "printRegion")]
    pub print_region: ((f64, f64), (f64, f64)),
    #[serde(rename = "printDrawOrder")]
    pub print_draw_order: PrintDrawOrder,
}

impl PcadDeserialize for PrintQueueEntry {
    const TAG: &'static [u8] = b"printQueueEntry";
    const HAS_INTERNAL_NAME: bool = true;

    fn load_internal_name(&mut self, id: &[u8]) {
        self.id = String::from_utf8_lossy(id).into_owned();
    }

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"orderedLayerList" => {
                let mut layers = Vec::new();
                loop {
                    if matches!(parser.peek_token_type(), Some(TokenType::CloseParen)) {
                        parser.expect_token(TokenKind::CloseParen)?;
                        break;
                    }
                    parser.expect_token(TokenKind::OpenParen)?;
                    parser.expect_token_bytes(b"layerNumRef")?;
                    let layer = parser.parse_i32()?;
                    layers.push(layer);
                    parser.expect_token(TokenKind::CloseParen)?;
                }
                self.ordered_layer_list = layers;
                Ok(true)
            }
            b"isThinStrokeText" => {
                self.is_thin_stroke_text = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"isSelected" => {
                self.is_selected = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"drillSymSize" => {
                self.drill_sym_size = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"outputMirror" => {
                self.output_mirror = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"outputPads" => {
                self.output_pads = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"outputVias" => {
                self.output_vias = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"outputHoles" => {
                self.output_holes = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"PrintRegion" => {
                parser.expect_token(TokenKind::OpenParen)?;
                parser.expect_token_bytes(b"pt")?;
                let x1 = parser.parse_f64()?;
                let y1 = parser.parse_f64()?;
                parser.expect_token(TokenKind::CloseParen)?;

                parser.expect_token(TokenKind::OpenParen)?;
                parser.expect_token_bytes(b"pt")?;
                let x2 = parser.parse_f64()?;
                let y2 = parser.parse_f64()?;
                parser.expect_token(TokenKind::CloseParen)?;
                parser.expect_token(TokenKind::CloseParen)?;

                self.print_region = ((x1, y1), (x2, y2));
                Ok(true)
            }
            b"printDrawOrder" => {
                self.print_draw_order = parser.parse_common_struct::<PrintDrawOrder>(Wrapped::In)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PrintDrawOrder {
    #[serde(rename = "isCustom")]
    pub is_custom: bool,
    #[serde(rename = "drawOrderItems")]
    pub draw_order_items: Vec<DrawOrderItem>,
    #[serde(rename = "printNonsignalLayerItems")]
    pub print_nonsignal_layer_items: Vec<PrintNonsignalLayerItem>,
    #[serde(rename = "printSignalLayerItems")]
    pub print_signal_layer_items: Vec<PrintSignalLayerItem>,
}

impl PcadDeserialize for PrintDrawOrder {
    const TAG: &'static [u8] = b"printDrawOrder";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"isCustom" => {
                self.is_custom = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"drawOrderItem" => {  
                let item = parser.parse_common_struct::<DrawOrderItem>(Wrapped::In)?;
                self.draw_order_items.push(item);
                Ok(true)
            }
            b"printNonsignalLayerDrawOrderItem" => {
                let item = parser.parse_common_struct::<PrintNonsignalLayerItem>(Wrapped::In)?;
                self.print_nonsignal_layer_items.push(item);
                Ok(true)
            }
            b"printSignalLayerDrawOrderItem" => {
                let item = parser.parse_common_struct::<PrintSignalLayerItem>(Wrapped::In)?;
                self.print_signal_layer_items.push(item);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DrawOrderItem {
    pub name: String,
}

impl PcadDeserialize for DrawOrderItem {
    const TAG: &'static [u8] = b"drawOrderItem";
    const HAS_INTERNAL_NAME: bool = true;

    fn load_internal_name(&mut self, name: &[u8]) {
        self.name = String::from_utf8_lossy(name).into_owned();
    }

    fn load_field(&mut self, _field_name: &[u8], _parser: &mut PcadParser) -> Result<bool, ParserError> {
        Ok(false)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PrintNonsignalLayerItem {
    #[serde(rename = "layerNumRef")]
    pub layer_num_ref: i32,
    #[serde(rename = "drawOrderItems")]
    pub draw_order_items: Vec<DrawOrderItem>,
}

impl PcadDeserialize for PrintNonsignalLayerItem {
    const TAG: &'static [u8] = b"printNonsignalLayerItem";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"layerNumRef" => {
                self.layer_num_ref = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"drawOrderItem" => {  
                let item = parser.parse_common_struct::<DrawOrderItem>(Wrapped::In)?;
                self.draw_order_items.push(item);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PrintSignalLayerItem {
    #[serde(rename = "layerNumRef")]
    pub layer_num_ref: i32,
    #[serde(rename = "drawOrderItems")]
    pub draw_order_items: Vec<DrawOrderItem>,
}

impl PcadDeserialize for PrintSignalLayerItem {
    const TAG: &'static [u8] = b"printSignalLayerItem";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"layerNumRef" => {
                self.layer_num_ref = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"drawOrderItem" => {  
                let item = parser.parse_common_struct::<DrawOrderItem>(Wrapped::In)?;
                self.draw_order_items.push(item);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DrillSymSettings {
    // TODO: finish 
}

impl PcadDeserialize for DrillSymSettings {
    const TAG: &'static [u8] = b"drillSymSettings";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        parser.parse_field(|p| {Ok(())});
        Ok(false)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GerberSettings {
    pub units: GerberUnits,
    #[serde(rename = "numFormat")]
    pub num_format: GerberNumFormat,
    #[serde(rename = "autoDrawApertureSize")]
    pub auto_draw_aperture_size: f64,
    #[serde(rename = "outputPath")]
    pub output_path: String,
    #[serde(rename = "viewLog")]
    pub view_log: bool,
    #[serde(rename = "autoClear")]
    pub auto_clear: bool,
    #[serde(rename = "g54Option")]
    pub g54_option: bool,
    #[serde(rename = "useArcs")]
    pub use_arcs: bool,
    #[serde(rename = "embedApertures")]
    pub embed_apertures: bool,
    #[serde(rename = "useApertureHoles")]
    pub use_aperture_holes: bool,
    #[serde(rename = "drawRotated")]
    pub draw_rotated: bool,
    #[serde(rename = "drawPolygons")]
    pub draw_polygons: bool,
}

impl PcadDeserialize for GerberSettings {
    const TAG: &'static [u8] = b"gerberSettings";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"units" => {
                let units_str = parser.parse_field(|p| p.parse_string())?;
                self.units = match units_str.as_str() {
                    "in" => GerberUnits::In,
                    _ => return Err(ParserError::GeneralError("Unknown gerber units")),
                };
                Ok(true)
            }
            b"numFormat" => {
                let format_str = parser.parse_field(|p| p.parse_string())?;
                self.num_format = match format_str.as_str() {
                    "gbr44" => GerberNumFormat::Gbr44,
                    _ => return Err(ParserError::GeneralError("Unknown gerber num format")),
                };
                Ok(true)
            }
            b"autoDrawApertureSize" => {
                self.auto_draw_aperture_size = parser.parse_field(|p| p.parse_f64())?;
                Ok(true)
            }
            b"outputPath" => {
                self.output_path = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"viewLog" => {
                self.view_log = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"autoClear" => {
                self.auto_clear = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"g54Option" => {
                self.g54_option = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"useArcs" => {
                self.use_arcs = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"embedApertures" => {
                self.embed_apertures = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"useApertureHoles" => {
                self.use_aperture_holes = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"drawRotated" => {
                self.draw_rotated = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"drawPolygons" => {
                self.draw_polygons = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum GerberUnits {
    #[default]
    In,
    //TODO: add other variants
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum GerberNumFormat {
    #[default]
    Gbr44,
    //TODO: add other variants
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NcDrillSettings {
    pub units: NcDrillUnits,
    #[serde(rename = "ncDrillFormat")]
    pub nc_drill_format: (i32, i32),
    #[serde(rename = "codeFormat")]
    pub code_format: CodeFormat,
    #[serde(rename = "zeroFormat")]
    pub zero_format: ZeroFormat,
    #[serde(rename = "outputPath")]
    pub output_path: String,
    #[serde(rename = "viewLog")]
    pub view_log: bool,
    #[serde(rename = "autoClear")]
    pub auto_clear: bool,
}

impl PcadDeserialize for NcDrillSettings {
    const TAG: &'static [u8] = b"ncDrillSettings";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"units" => {
                let units_str = parser.parse_field(|p| p.parse_string())?;
                self.units = match units_str.as_str() {
                    "in" => NcDrillUnits::In,
                    _ => return Err(ParserError::GeneralError("Unknown nc drill units")),
                };
                Ok(true)
            }
            b"ncDrillFormat" => {
                let digits = parser.parse_i32()?;
                let decimals = parser.parse_i32()?;
                self.nc_drill_format = (digits, decimals);
                parser.expect_token(TokenKind::CloseParen)?;
                Ok(true)
            }
            b"codeFormat" => {
                let format_str = parser.parse_field(|p| p.parse_string())?;
                self.code_format = match format_str.as_str() {
                    "AsciiEven" => CodeFormat::AsciiEven,
                    _ => return Err(ParserError::GeneralError("Unknown code format")),
                };
                Ok(true)
            }
            b"zeroFormat" => {
                let zero_str = parser.parse_field(|p| p.parse_string())?;
                self.zero_format = match zero_str.as_str() {
                    "Leading" => ZeroFormat::Leading,
                    _ => return Err(ParserError::GeneralError("Unknown zero format")),
                };
                Ok(true)
            }
            b"outputPath" => {
                self.output_path = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"viewLog" => {
                self.view_log = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"autoClear" => {
                self.auto_clear = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum NcDrillUnits {
    #[default]
    In,
    //TODO: add other variants
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum CodeFormat {
    #[default]
    AsciiEven,
    //TODO: add other variants
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum ZeroFormat {
    #[default]
    Leading,
    //TODO: add other variants
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProgramState {
    #[serde(rename = "layerState")]
    pub layer_state: LayerState,
    #[serde(rename = "gridState")]
    pub grid_state: GridState,
    #[serde(rename = "ecoState")]
    pub eco_state: EcoState,
    #[serde(rename = "onlineDrcState")]
    pub online_drc_state: OnlineDrcState,
    #[serde(rename = "drcState")]
    pub drc_state: DrcState,
    #[serde(rename = "currentTextStyle")]
    pub current_text_style: String,
    #[serde(rename = "currentPadStyle")]
    pub current_pad_style: String,
    #[serde(rename = "currentViaStyle")]
    pub current_via_style: String,
}

impl PcadDeserialize for ProgramState {
    const TAG: &'static [u8] = b"programState";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"layerState" => {
                self.layer_state = parser.parse_common_struct::<LayerState>(Wrapped::In)?;
                Ok(true)
            }
            b"gridState" => {
                self.grid_state = parser.parse_common_struct::<GridState>(Wrapped::In)?;
                Ok(true)
            }
            b"ecoState" => {
                self.eco_state = parser.parse_common_struct::<EcoState>(Wrapped::In)?;
                Ok(true)
            }
            b"onlineDrcState" => {
                self.online_drc_state = parser.parse_common_struct::<OnlineDrcState>(Wrapped::In)?;
                Ok(true)
            }
            b"drcState" => {
                self.drc_state = parser.parse_common_struct::<DrcState>(Wrapped::In)?;
                Ok(true)
            }
            b"currentTextStyle" => {
                self.current_text_style = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"currentPadStyle" => {
                self.current_pad_style = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"currentViaStyle" => {
                self.current_via_style = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LayerState {
    #[serde(rename = "currentLayer")]
    pub current_layer: i32,
}

impl PcadDeserialize for LayerState {
    const TAG: &'static [u8] = b"layerState";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"currentLayer" => {
                parser.expect_token(TokenKind::OpenParen)?;      
                parser.expect_token_bytes(b"layerNumRef")?;     
                self.current_layer = parser.parse_i32()?;       
                parser.expect_token(TokenKind::CloseParen)?;    

                parser.expect_token(TokenKind::CloseParen)?;

                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GridState {
    #[serde(rename = "currentAbsGrid")]
    pub current_abs_grid: String,
    #[serde(rename = "currentRelGrid")]
    pub current_rel_grid: String,
    #[serde(rename = "currentViaGrid")]
    pub current_via_grid: String,
    #[serde(rename = "isAbsoluteGrid")]   
    pub is_absolute_grid: bool,
    #[serde(rename = "isDottedGrid")]     
    pub is_dotted_grid: bool,
    #[serde(rename = "isVisibleGrid")]    
    pub is_visible_grid: bool,
    #[serde(rename = "isPromptForRel")]   
    pub is_prompt_for_rel: bool,
    #[serde(rename = "viaGridVisibility")]
    pub via_grid_visibility: ViaGridVisibility,
}

impl PcadDeserialize for GridState {
    const TAG: &'static [u8] = b"gridState";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"currentAbsGrid" => {
                self.current_abs_grid = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"currentRelGrid" => {
                self.current_rel_grid = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"currentViaGrid" => {
                self.current_via_grid = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"isAbsoluteGrid" => {  
                self.is_absolute_grid = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"isDottedGrid" => {    
                self.is_dotted_grid = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"isVisibleGrid" => {   
                self.is_visible_grid = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"isPromptForRel" => {  
                self.is_prompt_for_rel = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"viaGridVisibility" => {
                let visibility_str = parser.parse_field(|p| p.parse_string())?;
                self.via_grid_visibility = match visibility_str.to_lowercase().as_str() {
                    "show_at_plowing" => ViaGridVisibility::ShowAtPlowing,
                    _ => return Err(ParserError::GeneralError("Unknown via grid visibility")),
                };
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum ViaGridVisibility {
    #[default]
    ShowAtPlowing,
    //TODO: add other variants
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EcoState {
    #[serde(rename = "ecoRecording")]
    pub eco_recording: bool,
}

impl PcadDeserialize for EcoState {
    const TAG: &'static [u8] = b"ecoState";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"ecoRecording" => {
                self.eco_recording = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OnlineDrcState {
    #[serde(rename = "onlineDrcEnabled")]
    pub online_drc_enabled: bool,
    #[serde(rename = "onlineDrcReport")]
    pub online_drc_report: bool,
    #[serde(rename = "onlineDrcClearanceEnabled")]
    pub online_drc_clearance_enabled: bool,
    #[serde(rename = "onlineDrcTextEnabled")]
    pub online_drc_text_enabled: bool,
    #[serde(rename = "onlineDrcSameCompPadsEnabled")]
    pub online_drc_same_comp_pads_enabled: bool,
    #[serde(rename = "onlineDrcComponentEnabled")]
    pub online_drc_component_enabled: bool,
    #[serde(rename = "onlineDrcSilkEnabled")]
    pub online_drc_silk_enabled: bool,
    #[serde(rename = "onlineDrcNetlistEnabled")]
    pub online_drc_netlist_enabled: bool,
    #[serde(rename = "onlineDrcWidthEnabled")]
    pub online_drc_width_enabled: bool,
}

impl PcadDeserialize for OnlineDrcState {
    const TAG: &'static [u8] = b"onlineDrcState";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"onlineDrcEnabled" => {
                self.online_drc_enabled = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"onlineDrcReport" => {
                self.online_drc_report = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"onlineDrcClearanceEnabled" => {
                self.online_drc_clearance_enabled = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"onlineDrcTextEnabled" => {
                self.online_drc_text_enabled = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"onlineDrcSameCompPadsEnabled" => {
                self.online_drc_same_comp_pads_enabled = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"onlineDrcComponentEnabled" => {
                self.online_drc_component_enabled = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"onlineDrcSilkEnabled" => {
                self.online_drc_silk_enabled = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"onlineDrcNetlistEnabled" => {
                self.online_drc_netlist_enabled = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"onlineDrcWidthEnabled" => {
                self.online_drc_width_enabled = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}




#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DrcState {
    #[serde(rename = "drcRegion")]
    pub drc_region: ((f64, f64), (f64, f64)),
}

impl PcadDeserialize for DrcState {
    const TAG: &'static [u8] = b"drcState";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"drcRegion" => {
                parser.expect_token(TokenKind::OpenParen)?;
                parser.expect_token_bytes(b"pt")?;
                let x1 = parser.parse_f64()?;
                let y1 = parser.parse_f64()?;
                parser.expect_token(TokenKind::CloseParen)?;

                parser.expect_token(TokenKind::OpenParen)?;
                parser.expect_token_bytes(b"pt")?;
                let x2 = parser.parse_f64()?;
                let y2 = parser.parse_f64()?;
                parser.expect_token(TokenKind::CloseParen)?;
                parser.expect_token(TokenKind::CloseParen)?;

                self.drc_region = ((x1, y1), (x2, y2));
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LayerSets {
    // #[serde(rename = "layerSets")]
    pub layer_sets: Vec<LayerSet>,
}

impl PcadDeserialize for LayerSets {
    const TAG: &'static [u8] = b"layerSets";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        if field_name == b"layerSet" {
            let layer_set = parser.parse_seq_struct::<LayerSet>(Wrapped::In)?;
            self.layer_sets.push(layer_set);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LayerSet {
    pub name: String,
    pub layers: Vec<String>,
}

impl PcadDeserialize for LayerSet {
    const TAG: &'static [u8] = b"layerSet";
    const HAS_INTERNAL_NAME: bool = true;

    fn load_internal_name(&mut self, name: &[u8]) {
        self.name = String::from_utf8_lossy(name).into_owned();
    }

    fn load_field(&mut self, value: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        let layer_str = String::from_utf8_lossy(value).into_owned();
        self.layers.push(layer_str);
        Ok(true)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DisplayDrawOrder {
    #[serde(rename = "isCustom")]
    pub is_custom: bool,
    #[serde(rename = "autoOrderCurrentLayer")]
    pub auto_order_current_layer: bool,
    #[serde(rename = "autoOrderSilkscreen")]
    pub auto_order_silkscreen: bool,
    #[serde(rename = "drawOrderItems")]
    pub draw_order_items: Vec<DrawOrderItem>,
    #[serde(rename = "displayNonsignalLayerItems")]
    pub display_nonsignal_layer_items: Vec<DisplayNonsignalLayerItem>,
    #[serde(rename = "displaySignalLayerItems")]
    pub display_signal_layer_items: Vec<DisplaySignalLayerItem>,
}

impl PcadDeserialize for DisplayDrawOrder {
    const TAG: &'static [u8] = b"displayDrawOrder";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"isCustom" => {
                self.is_custom = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"autoOrderCurrentLayer" => {
                self.auto_order_current_layer = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"autoOrderSilkscreen" => {
                self.auto_order_silkscreen = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"drawOrderItem" => {  
                let item = parser.parse_common_struct::<DrawOrderItem>(Wrapped::In)?;
                self.draw_order_items.push(item);
                Ok(true)
            }
            b"displayNonsignalLayerDrawOrderItem" => {  
                let item = parser.parse_common_struct::<DisplayNonsignalLayerItem>(Wrapped::In)?;
                self.display_nonsignal_layer_items.push(item);
                Ok(true)
            }
            b"displaySignalLayerDrawOrderItem" => {  
                let item = parser.parse_common_struct::<DisplaySignalLayerItem>(Wrapped::In)?;
                self.display_signal_layer_items.push(item);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DisplayNonsignalLayerItem {
    #[serde(rename = "layerNumRef")]
    pub layer_num_ref: i32,
    #[serde(rename = "drawOrderItems")]
    pub draw_order_items: Vec<DrawOrderItem>,
}

impl PcadDeserialize for DisplayNonsignalLayerItem {
    const TAG: &'static [u8] = b"displayNonsignalLayerItem";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"layerNumRef" => {
                self.layer_num_ref = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"drawOrderItem" => { 
                let item = parser.parse_common_struct::<DrawOrderItem>(Wrapped::In)?;
                self.draw_order_items.push(item);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DisplaySignalLayerItem {
    #[serde(rename = "layerNumRef")]
    pub layer_num_ref: i32,
    #[serde(rename = "drawOrderItems")]
    pub draw_order_items: Vec<DrawOrderItem>,
}

impl PcadDeserialize for DisplaySignalLayerItem {
    const TAG: &'static [u8] = b"displaySignalLayerItem";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"layerNumRef" => {
                self.layer_num_ref = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"drawOrderItem" => { 
                let item = parser.parse_common_struct::<DrawOrderItem>(Wrapped::In)?;
                self.draw_order_items.push(item);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReportSettings {
    #[serde(rename = "reportStyle")]
    pub report_style: ReportStyle,
    #[serde(rename = "reportDestination")]
    pub report_destination: ReportDestination,
    #[serde(rename = "point")]
    pub point: (f64, f64),
    #[serde(rename = "reportDefinitions")]
    pub report_definitions: ReportDefinitions,
}

impl PcadDeserialize for ReportSettings {
    const TAG: &'static [u8] = b"reportSettings";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"reportStyle" => {
                let style_str = parser.parse_field(|p| p.parse_string())?;
                self.report_style = match style_str.as_str() {
                    "reportStyleAccel" => ReportStyle::ReportStyleAccel,
                    _ => return Err(ParserError::GeneralError("Unknown report style")),
                };
                Ok(true)
            }
            b"reportDestination" => {
                let dest_str = parser.parse_field(|p| p.parse_string())?;
                self.report_destination = match dest_str.as_str() {
                    "DestinationScreen" => ReportDestination::DestinationScreen,
                    _ => return Err(ParserError::GeneralError("Unknown report destination")),
                };
                Ok(true)
            }
            b"pt" => {
                let x = parser.parse_f64()?;
                let y = parser.parse_f64()?;
                self.point = (x, y);
                parser.expect_token(TokenKind::CloseParen)?;
                Ok(true)
            }
            b"reportDefinitions" => {
                self.report_definitions = parser.parse_common_struct::<ReportDefinitions>(Wrapped::In)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReportDefinitions {
    pub report_definitions: Vec<ReportDefinition>,
}

impl PcadDeserialize for ReportDefinitions {
    const TAG: &'static [u8] = b"reportDefinitions";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        if field_name == b"reportDefinition" {
            let def = parser.parse_common_struct::<ReportDefinition>(Wrapped::In)?;
            self.report_definitions.push(def);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum ReportStyle {
    #[default]
    ReportStyleAccel,
    //TODO: add other variants
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum ReportDestination {
    #[default]
    DestinationScreen,
    //TODO: add other variants
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReportDefinition {
    #[serde(rename = "reportName")]
    pub report_name: String,
    #[serde(rename = "reportExtension")]
    pub report_extension: String,
    #[serde(rename = "reportShowFlag")]
    pub report_show_flag: bool,
    #[serde(rename = "reportType")]
    pub report_type: ReportType,
    #[serde(rename = "reportLinesPerPage")]
    pub report_lines_per_page: i32,
    #[serde(rename = "reportColumnWidth")]
    pub report_column_width: i32,
    #[serde(rename = "reportHeader")]
    pub report_header: String,
    #[serde(rename = "reportFooter")]
    pub report_footer: String,
    #[serde(rename = "reportShowCDFPreface")]
    pub report_show_cdf_preface: bool,
    #[serde(rename = "reportShowColumnNames")]
    pub report_show_column_names: bool,
    #[serde(rename = "reportFieldsSections")]
    pub report_fields_sections: ReportFieldsSections,
}

impl PcadDeserialize for ReportDefinition {
    const TAG: &'static [u8] = b"reportDefinition";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"reportName" => {
                self.report_name = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"reportExtension" => {
                self.report_extension = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"reportShowFlag" => {
                self.report_show_flag = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"reportType" => {
                let type_str = parser.parse_field(|p| p.parse_string())?;
                self.report_type = match type_str.as_str() {
                    "ReportTypeApertures" => ReportType::ReportTypeApertures,
                    "ReportTypeAttributes" => ReportType::ReportTypeAttributes,
                    "ReportTypeBillOfMaterials" => ReportType::ReportTypeBillOfMaterials,
                    "ReportTypeComponentsLocations" => ReportType::ReportTypeComponentsLocations,
                    "ReportTypeDrcErrors" => ReportType::ReportTypeDrcErrors,
                    "ReportTypeGlueDots" => ReportType::ReportTypeGlueDots,
                    "ReportTypeLibrary" => ReportType::ReportTypeLibrary,
                    "ReportTypePickAndPlace" => ReportType::ReportTypePickAndPlace,
                    "ReportTypeRooms" => ReportType::ReportTypeRooms,
                    "ReportTypeStatistics" => ReportType::ReportTypeStatistics,
                    "ReportTypeTestPoint" => ReportType::ReportTypeTestPoint,
                    "ReportTypeVariant" => ReportType::ReportTypeVariant,
                    _ => return Err(ParserError::GeneralError("Unknown report type")),
                };
                Ok(true)
            }
            b"reportLinesPerPage" => {
                self.report_lines_per_page = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"reportColumnWidth" => {
                self.report_column_width = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"reportHeader" => {
                self.report_header = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"reportFooter" => {
                self.report_footer = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"reportShowCDFPreface" => {
                self.report_show_cdf_preface = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"reportShowColumnNames" => {
                self.report_show_column_names = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"reportFieldsSections" => {
                self.report_fields_sections = parser.parse_common_struct::<ReportFieldsSections>(Wrapped::In)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum ReportType {
    #[default]
    ReportTypeApertures,
    ReportTypeAttributes,
    ReportTypeBillOfMaterials,
    ReportTypeComponentsLocations,
    ReportTypeDrcErrors,
    ReportTypeGlueDots,
    ReportTypeLibrary,
    ReportTypePickAndPlace,
    ReportTypeRooms,
    ReportTypeStatistics,
    ReportTypeTestPoint,
    ReportTypeVariant,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReportFields {
    #[serde(rename = "reportField")]
    pub fields: Vec<ReportField>,
}

impl PcadDeserialize for ReportFields {
    const TAG: &'static [u8] = b"reportFields";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        if field_name == b"reportField" {
            let field = parser.parse_common_struct::<ReportField>(Wrapped::In)?;
            self.fields.push(field);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReportFieldsSections {
    #[serde(rename = "reportFields")]
    pub report_fields: Vec<ReportFields>,
}

impl PcadDeserialize for ReportFieldsSections {
    const TAG: &'static [u8] = b"reportFieldsSections";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        if field_name == b"reportFields" {
            let fields = parser.parse_common_struct::<ReportFields>(Wrapped::In)?;
            self.report_fields.push(fields);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReportField {
    #[serde(rename = "reportFieldName")]
    pub report_field_name: String,
    #[serde(rename = "reportFieldType")]
    pub report_field_type: PropertyType,
    #[serde(rename = "reportFieldSortOrder")]
    pub report_field_sort_order: i32,
    #[serde(rename = "reportFieldSortType")]
    pub report_field_sort_type: SortType,
    #[serde(rename = "reportFieldShowFlag")]
    pub report_field_show_flag: bool,
    #[serde(rename = "reportFieldColumnWidth")]
    pub report_field_column_width: i32,
}

impl PcadDeserialize for ReportField {
    const TAG: &'static [u8] = b"reportField";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"reportFieldName" => {
                self.report_field_name = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"reportFieldType" => {
                let type_str = parser.parse_field(|p| p.parse_string())?;
                self.report_field_type = match type_str.as_str() {
                    "PropertyTypeDCode" => PropertyType::PropertyTypeDCode,
                    "PropertyTypeShape" => PropertyType::PropertyTypeShape,
                    "PropertyTypeDimX" => PropertyType::PropertyTypeDimX,
                    "PropertyTypeDimY" => PropertyType::PropertyTypeDimY,
                    "PropertyTypeDiameter" => PropertyType::PropertyTypeDiameter,
                    "PropertyTypeType" => PropertyType::PropertyTypeType,
                    "PropertyTypeAngle" => PropertyType::PropertyTypeAngle,
                    "PropertyTypeOffsetX" => PropertyType::PropertyTypeOffsetX,
                    "PropertyTypeOffsetY" => PropertyType::PropertyTypeOffsetY,
                    "PropertyTypeRefdes" => PropertyType::PropertyTypeRefdes,
                    "PropertyTypeCompValue" => PropertyType::PropertyTypeCompValue,
                    "PropertyTypeNetname" => PropertyType::PropertyTypeNetname,
                    "PropertyTypeCount" => PropertyType::PropertyTypeCount,
                    "PropertyTypeComponentName" => PropertyType::PropertyTypeComponentName,
                    "PropertyTypePatternName" => PropertyType::PropertyTypePatternName,
                    "PropertyTypeLayer" => PropertyType::PropertyTypeLayer,
                    "PropertyTypeLocationX" => PropertyType::PropertyTypeLocationX,
                    "PropertyTypeLocationY" => PropertyType::PropertyTypeLocationY,
                    "PropertyTypeRotation" => PropertyType::PropertyTypeRotation,
                    "PropertyTypeFixed" => PropertyType::PropertyTypeFixed,
                    "PropertyTypeErrorNumber" => PropertyType::PropertyTypeErrorNumber,
                    "PropertyTypeError" => PropertyType::PropertyTypeError,
                    "PropertyTypeBoardside" => PropertyType::PropertyTypeBoardside,
                    "PropertyTypeRoom" => PropertyType::PropertyTypeRoom,
                    "PropertyTypeComponents" => PropertyType::PropertyTypeComponents,
                    "PropertyTypeUnassignedComps" => PropertyType::PropertyTypeUnassignedComps,
                    "PropertyTypeVariant" => PropertyType::PropertyTypeVariant,
                    "PropertyTypeVariantDescription" => PropertyType::PropertyTypeVariantDescription,
                    "PropertyTypeExcludedComponents" => PropertyType::PropertyTypeExcludedComponents,
                    _ => return Err(ParserError::GeneralError("Unknown property type")),
                };
                Ok(true)
            }
            b"reportFieldSortOrder" => {
                self.report_field_sort_order = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            b"reportFieldSortType" => {
                let sort_str = parser.parse_field(|p| p.parse_string())?;
                self.report_field_sort_type = match sort_str.as_str() {
                    "ascending" => SortType::Ascending,
                    "descending" => SortType::Descending,
                    "None" => SortType::None,
                    _ => return Err(ParserError::GeneralError("Unknown sort type")),
                };
                Ok(true)
            }
            b"reportFieldShowFlag" => {
                self.report_field_show_flag = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            b"reportFieldColumnWidth" => {
                self.report_field_column_width = parser.parse_field(|p| p.parse_i32())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum PropertyType {
    #[default]
    PropertyTypeDCode,
    PropertyTypeShape,
    PropertyTypeDimX,
    PropertyTypeDimY,
    PropertyTypeDiameter,
    PropertyTypeType,
    PropertyTypeAngle,
    PropertyTypeOffsetX,
    PropertyTypeOffsetY,
    PropertyTypeRefdes,
    PropertyTypeCompValue,
    PropertyTypeNetname,
    PropertyTypeCount,
    PropertyTypeComponentName,
    PropertyTypePatternName,
    PropertyTypeLayer,
    PropertyTypeLocationX,
    PropertyTypeLocationY,
    PropertyTypeRotation,
    PropertyTypeFixed,
    PropertyTypeErrorNumber,
    PropertyTypeError,
    PropertyTypeBoardside,
    PropertyTypeRoom,
    PropertyTypeComponents,
    PropertyTypeUnassignedComps,
    PropertyTypeVariant,
    PropertyTypeVariantDescription,
    PropertyTypeExcludedComponents,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum SortType {
    #[default]
    Ascending,
    Descending,
    None,
}


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct OdbSettings {
    #[serde(rename = "outputPath")]
    pub output_path: String,
    #[serde(rename = "viewLog")]
    pub view_log: bool,
}

impl PcadDeserialize for OdbSettings {
    const TAG: &'static [u8] = b"odbSettings";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"outputPath" => {
                self.output_path = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"viewLog" => {
                self.view_log = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LayersStackup {
    #[serde(rename = "layerStackupData")]
    pub layer_stackup_data: Vec<LayerStackupData>, 
}

impl PcadDeserialize for LayersStackup {
    const TAG: &'static [u8] = b"layersStackup";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"layerStackupData" => {
                let data = parser.parse_common_struct::<LayerStackupData>(Wrapped::In)?;
                self.layer_stackup_data.push(data);
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LayerStackupData {
    #[serde(rename = "layerStackupName")]
    pub name: String,
    #[serde(rename = "layerStackupMaterial")]
    pub material: String,
    #[serde(rename = "layerStackupThickness")]
    pub thickness: String,
    #[serde(rename = "layerStackupDielectricConstant")]
    pub dielectric_constant: String,
    #[serde(rename = "layerStackupDisplay")]
    pub display: bool,
}

impl PcadDeserialize for LayerStackupData {
    const TAG: &'static [u8] = b"layerStackupData";
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError> {
        match field_name {
            b"layerStackupName" => {
                self.name = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"layerStackupMaterial" => {
                self.material = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"layerStackupThickness" => {
                self.thickness = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"layerStackupDielectricConstant" => {
                self.dielectric_constant = parser.parse_field(|p| p.parse_string())?;
                Ok(true)
            }
            b"layerStackupDisplay" => {
                self.display = parser.parse_field(|p| p.parse_bool())?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
