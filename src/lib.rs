#[derive(Debug, Clone, PartialEq)]
pub struct CanvasDimension {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct DrawCanvas {
    pub dim: CanvasDimension,
    pub background_hex: String,
}

impl DrawCanvas {
    pub fn new(w: u32, h: u32) -> Self {
        Self { dim: CanvasDimension { width: w, height: h }, background_hex: "#ffffff".into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_draw_canvas_aaa() {
        // Arrange & Act
        let canvas = DrawCanvas::new(1920, 1080);
        // Assert
        assert_eq!(canvas.dim.width, 1920);
        assert_eq!(canvas.dim.height, 1080);
        assert_eq!(canvas.background_hex, "#ffffff");
    }
}
