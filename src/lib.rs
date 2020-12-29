use matrix_stack::MATRIX_STACK;
use nalgebra::Point;
use nalgebra::Vector2;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::BlendMode;
use sdl2::render::Canvas;
use sdl2::render::RenderTarget;
use sdl2::render::TextureValueError;
use sdl2::video::WindowBuildError;
use sdl2::IntegerOrSdlError;

pub mod matrix_stack;
pub mod timer;

pub fn map(value: f64, orig_min: f64, orig_max: f64, new_min: f64, new_max: f64) -> f64 {
    value * (new_max - new_min) / (orig_max - orig_min) + new_min
}

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

pub trait EventExt {
    fn is_quit(&self) -> bool;
}

impl EventExt for Event {
    fn is_quit(&self) -> bool {
        match self {
            Event::Quit { .. } => true,
            _ => false,
        }
    }
}

pub trait CanvasExt {
    fn background(&mut self, color: Color);
    fn reset_matrix(&mut self);
    fn translate(&mut self, dx: f64, dy: f64);
    fn rotate(&mut self, radians: f64);
    fn push_matrix(&mut self);
    fn pop_matrix(&mut self);
    fn ext_draw_line(
        &mut self,
        start: &Point<f64, nalgebra::U2>,
        end: &Point<f64, nalgebra::U2>,
        color: Color,
    ) -> Result<(), SdlError>;
    fn ext_fill_circle(
        &mut self,
        center: &Point<f64, nalgebra::U2>,
        radius: f64,
        color: Color,
    ) -> Result<(), SdlError>;
}

impl<RT: RenderTarget> CanvasExt for Canvas<RT> {
    fn background(&mut self, color: Color) {
        self.set_draw_color(color);
        self.clear();
    }

    fn reset_matrix(&mut self) {
        MATRIX_STACK.write().clear();
        MATRIX_STACK.write().push(nalgebra::Matrix3::identity());
    }

    fn translate(&mut self, dx: f64, dy: f64) {
        *MATRIX_STACK.write().last_mut().unwrap() *=
            nalgebra::geometry::Translation2::new(dx, dy).to_homogeneous();
    }

    fn rotate(&mut self, radians: f64) {
        *MATRIX_STACK.write().last_mut().unwrap() *=
            nalgebra::geometry::Rotation::from_axis_angle(&nalgebra::Vector3::z_axis(), radians)
    }

    fn push_matrix(&mut self) {
        let top = MATRIX_STACK.read().last().unwrap().clone();
        MATRIX_STACK.write().push(top);
    }

    fn pop_matrix(&mut self) {
        MATRIX_STACK.write().pop();
    }

    fn ext_draw_line(
        &mut self,
        start: &Point<f64, nalgebra::U2>,
        end: &Point<f64, nalgebra::U2>,
        color: Color,
    ) -> Result<(), SdlError> {
        let start = MATRIX_STACK.read().last().unwrap().transform_point(start);
        let end = MATRIX_STACK.read().last().unwrap().transform_point(end);
        self.line(
            start.x as i16,
            start.y as i16,
            end.x as i16,
            end.y as i16,
            color,
        )
        .map_err(SdlError::Draw)
    }

    fn ext_fill_circle(
        &mut self,
        center: &Point<f64, nalgebra::U2>,
        radius: f64,
        color: Color,
    ) -> Result<(), SdlError> {
        let center = MATRIX_STACK.read().last().unwrap().transform_point(center);
        self.filled_circle(center.x as i16, center.y as i16, radius as i16, color)
            .map_err(SdlError::Draw)
    }
}

pub fn init_sdl(
    title: impl AsRef<str>,
    size: Vector2<f64>,
) -> Result<
    (
        sdl2::Sdl,
        sdl2::VideoSubsystem,
        sdl2::render::WindowCanvas,
        sdl2::EventPump,
    ),
    SdlError,
> {
    let sdl = sdl2::init().map_err(SdlError::Init)?;
    let video = sdl.video().map_err(SdlError::InitVideo)?;
    let mut canvas = video
        .window(title.as_ref(), size.x as u32, size.y as u32)
        .resizable()
        .build()?
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(SdlError::CanvasBuild)?;
    canvas.set_blend_mode(BlendMode::Blend);
    let event_pump = sdl.event_pump().map_err(SdlError::EventPump)?;
    Ok((sdl, video, canvas, event_pump))
}

#[derive(Debug, thiserror::Error)]
pub enum SdlError {
    #[error("SDL initialization failed: {0}")]
    Init(String),
    #[error("SDL video subsystem initialization failed: {0}")]
    InitVideo(String),
    #[error("SDL_image initialization failed: {0}")]
    InitImage(String),
    #[error("Creating SDL event pump failed: {0}")]
    EventPump(String),
    #[error("Drawing to SDL canvas failed: {0}")]
    Draw(String),
    #[error("Loading image failed: {0}")]
    LoadImage(String),
    #[error("Building SDL window failed: {0}")]
    WindowBuild(#[from] WindowBuildError),
    #[error("Building SDL canvas failed: {0}")]
    CanvasBuild(IntegerOrSdlError),
    #[error("Creating SDL texture failed: {0}")]
    CreateTexture(#[from] TextureValueError),
    #[error("Writing directly to texture data failed: {0}")]
    LockTexture(String),
    #[error("Rendering text failed: {0}")]
    Font(#[from] FontError),
}
