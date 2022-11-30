use serde_json::{json, Map, Value};

pub struct ExcalidrawFile {
    pub version: i32,
    pub source: Option<String>,
    pub elements: Vec<Element>,
    pub app_state: AppState,
    pub files: Map<String, Value>,
}

impl Default for ExcalidrawFile {
    fn default() -> Self {
        Self {
            version: 2,
            source: None,
            elements: Vec::with_capacity(0),
            app_state: Default::default(),
            files: Map::with_capacity(0),
        }
    }
}

impl ToString for ExcalidrawFile {
    fn to_string(&self) -> String {
        json!(
            {
                "type": "excalidraw",
                "version": self.version,
                "source": self.source.clone(),
                "elements": self.elements.iter().map(|it| it.into()).collect::<Vec<Value>>(),
                "app_state": Into::<Value>::into(&self.app_state)
            }
        )
        .to_string()
    }
}

pub enum Element {
    Text {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        angle: i32,
        stroke_color: String,
        background_color: String,
        fill_style: String,
        stroke_width: i32,
        stroke_style: String,
        roughness: i32,
        opacity: i32,
        stroke_sharpness: String,
        locked: bool,
        text: String,
        font_size: i32,
        font_family: i32,
        text_align: String,
        vertical_align: String,
        baseline: i32,
    },
    Line {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        angle: i32,
        stroke_color: String,
        background_color: String,
        fill_style: String,
        stroke_width: i32,
        stroke_style: String,
        roughness: i32,
        opacity: i32,
        stroke_sharpness: String,
        locked: bool,
        points: Vec<[i32; 2]>,
    },
}

pub mod elements {
    pub const ANGLE: i32 = 0;
    pub const STROKE_COLOR: &str = "#000000";
    pub const BACKGROUND_COLOR: &str = "transparent";
    pub const FILL_STYLE: &str = "hachure";
    pub const STROKE_WIDTH: i32 = 1;
    pub const STROKE_STYLE: &str = "solid";
    pub const ROUGHNESS: i32 = 0;
    pub const OPACITY: i32 = 100;
    pub const STROKE_SHARPNESS: &str = "sharp";
    pub const LOCKED: bool = false;
    pub const FONT_SIZE_SMALL: i32 = 16;
    pub const FONT_SIZE_MEDIUM: i32 = 20;
    pub const FONT_SIZE_LARGE: i32 = 28;
    pub const FONT_SIZE_EXTRA_LARGE: i32 = 36;
    pub const FONT_FAMILY_HAND_DRAWN: i32 = 1;
    pub const FONT_FAMILY_NORMAL: i32 = 2;
    pub const FONT_FAMILY_MONOSPACE: i32 = 3;
    pub const TEXT_ALIGN_LEFT: &str = "left";
    pub const TEXT_ALIGN_CENTER: &str = "center";
    pub const TEXT_ALIGN_RIGHT: &str = "right";
    pub const VERTICAL_ALIGN_TOP: &str = "top";
    pub const VERTICAL_ALIGN_CENTER: &str = "center";
    pub const VERTICAL_ALIGN_BOTTOM: &str = "bottom";
    pub const BASELINE: i32 = 15;
}

impl Element {
    pub fn text(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        angle: i32,
        stroke_color: String,
        background_color: String,
        fill_style: String,
        stroke_width: i32,
        stroke_style: String,
        opacity: i32,
        stroke_sharpness: String,
        locked: bool,
        text: String,
        font_size: i32,
        font_family: i32,
        text_align: String,
        vertical_align: String,
    ) -> Self {
        Self::Text {
            x,
            y,
            width,
            height,
            angle,
            stroke_color,
            background_color,
            fill_style,
            stroke_width,
            stroke_style,
            roughness: 0,
            opacity,
            stroke_sharpness,
            locked,
            text: text.clone(),
            font_size,
            font_family,
            text_align,
            vertical_align,
            baseline: 15,
        }
    }

    pub fn line(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        angle: i32,
        stroke_color: String,
        background_color: String,
        fill_style: String,
        stroke_width: i32,
        stroke_style: String,
        opacity: i32,
        stroke_sharpness: String,
        locked: bool,
        points: Vec<[i32; 2]>,
    ) -> Self {
        Self::Line {
            x,
            y,
            width,
            height,
            angle,
            stroke_color,
            background_color,
            fill_style,
            stroke_width,
            stroke_style,
            roughness: 0,
            opacity,
            stroke_sharpness,
            locked,
            points,
        }
    }

    pub fn small_monospaced_text(x: i32, y: i32, locked: bool, text: String) -> Self {
        Self::text(
            x,
            y,
            (4 + text.chars().count() * 9) as i32,
            (text.lines().count() * 19) as i32,
            0,
            elements::STROKE_COLOR.into(),
            elements::BACKGROUND_COLOR.into(),
            elements::FILL_STYLE.into(),
            elements::STROKE_WIDTH,
            elements::STROKE_STYLE.into(),
            elements::OPACITY,
            elements::STROKE_SHARPNESS.into(),
            locked,
            text,
            elements::FONT_SIZE_SMALL,
            elements::FONT_FAMILY_MONOSPACE,
            elements::TEXT_ALIGN_LEFT.into(),
            elements::VERTICAL_ALIGN_TOP.into(),
        )
    }

    pub fn draw_line(
        x: i32,
        y: i32,
        locked: bool,
        points: Vec<[i32; 2]>,
    ) -> Self {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for p in &points {
            if p[0] > max_x {
                max_x = p[0];
            }
            if p[0] < min_x {
                min_x = p[0];
            }
            if p[1] > max_y {
                max_y = p[1];
            }
            if p[1] < min_y {
                min_y = p[1];
            }
        }
        Self::line(
            x,
            y,
            min_x.abs() + max_x.abs(),
            min_y.abs() + max_y.abs(),
            elements::ANGLE,
            elements::STROKE_COLOR.into(),
            elements::BACKGROUND_COLOR.into(),
            elements::FILL_STYLE.into(),
            elements::STROKE_WIDTH,
            elements::STROKE_STYLE.into(),
            elements::OPACITY,
            elements::STROKE_SHARPNESS.into(),
            locked,
            points,
        )
    }
}

impl Into<Value> for &Element {
    fn into(self) -> Value {
        match self {
            Element::Text {
                x,
                y,
                width,
                height,
                angle,
                stroke_color,
                background_color,
                fill_style,
                stroke_width,
                stroke_style,
                roughness,
                opacity,
                stroke_sharpness,
                locked,
                text,
                font_size,
                font_family,
                text_align,
                vertical_align,
                baseline,
            } => json!(
                {
                    "type": "text",
                    "x": x,
                    "y": y,
                    "width": width,
                    "height": height,
                    "angle": angle,
                    "strokeColor": stroke_color,
                    "backgroundColor": background_color,
                    "fillStyle": fill_style,
                    "strokeWidth": stroke_width,
                    "strokeStyle": stroke_style,
                    "roughness": roughness,
                    "opacity": opacity,
                    "strokeSharpness": stroke_sharpness,
                    "locked": locked,
                    "text": text,
                    "fontSize": font_size,
                    "fontFamily": font_family,
                    "textAlign": text_align,
                    "verticalAlign": vertical_align,
                    "baseline": baseline,
                }
            ),
            Element::Line {
                x,
                y,
                width,
                height,
                angle,
                stroke_color,
                background_color,
                fill_style,
                stroke_width,
                stroke_style,
                roughness,
                opacity,
                stroke_sharpness,
                locked,
                points,
            } => json!({
                "type": "line",
                "x": x,
                "y": y,
                "width": width,
                "height": height,
                "angle": angle,
                "strokeColor": stroke_color,
                "backgroundColor": background_color,
                "fillStyle": fill_style,
                "strokeWidth": stroke_width,
                "strokeStyle": stroke_style,
                "roughness": roughness,
                "opacity": opacity,
                "strokeSharpness": stroke_sharpness,
                "locked": locked,
                "points": points,
            }),
        }
    }
}

pub struct AppState {
    pub grid_size: i32,
    pub view_background_color: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            grid_size: 20,
            view_background_color: "#ffffff".into(),
        }
    }
}

impl Into<Value> for &AppState {
    fn into(self) -> Value {
        json!(
            {
                "gridSize": self.grid_size,
                "viewBackgroundColor": self.view_background_color.clone(),
            }
        )
    }
}

pub enum FontFamily {}
