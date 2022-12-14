use serde::Serialize;
use serde_json::{Map, Value};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcalidrawFile {
    pub r#type: String,
    pub version: i32,
    pub source: Option<String>,
    pub elements: Vec<Element>,
    pub app_state: AppState,
    pub files: Map<String, Value>,
}

impl ExcalidrawFile {
    pub fn draw(&mut self, element: &impl Drawable, x: i32, y: i32, locked: bool) -> (i32, i32) {
        element.draw(self, x, y, locked)
    }
}

impl Default for ExcalidrawFile {
    fn default() -> Self {
        Self {
            r#type: "excalidraw".into(),
            version: 2,
            source: None,
            elements: Vec::with_capacity(0),
            app_state: Default::default(),
            files: Map::with_capacity(0),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Element {
    #[serde(rename_all = "camelCase")]
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
    #[serde(rename_all = "camelCase")]
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
    #[serde(rename_all = "camelCase")]
    Rectangle {
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

    pub fn rectangle(
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
    ) -> Self {
        Self::Rectangle {
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
        }
    }

    pub fn draw_small_monospaced_text(x: i32, y: i32, locked: bool, text: String) -> Self {
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

    pub fn simple_line(x: i32, y: i32, locked: bool, points: Vec<[i32; 2]>) -> Self {
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

    pub fn simple_rectangle(x: i32, y: i32, width: i32, height: i32, locked: bool) -> Self {
        Self::rectangle(
            x,
            y,
            width,
            height,
            elements::ANGLE,
            elements::STROKE_COLOR.into(),
            elements::BACKGROUND_COLOR.into(),
            elements::FILL_STYLE.into(),
            elements::STROKE_WIDTH,
            elements::STROKE_STYLE.into(),
            elements::OPACITY,
            elements::STROKE_SHARPNESS.into(),
            locked,
        )
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
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

pub trait Drawable {
    /// Draw the element onto a file
    /// 
    /// Returns the width and height of the drawn element
    fn draw(&self, file: &mut ExcalidrawFile, x: i32, y: i32, locked: bool) -> (i32, i32);
}
