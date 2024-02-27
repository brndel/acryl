use acryl_core::{
    math::{Area, Pt, Vector2},
    Color,
};

use crate::stream::{
    color::ColorOperation,
    graphics_state::{GraphicsState, LineCap, LineJoin},
    path_construction::PathConstruction,
    path_painting::{FillRule, PathPainting},
};

use super::StreamBuilder;

pub struct PathBuilder<'builder, 'page> {
    builder: &'builder mut StreamBuilder<'page>,
    path: Vec<PathConstruction>,
}

impl<'builder, 'page> PathBuilder<'builder, 'page> {
    pub fn new(builder: &'builder mut StreamBuilder<'page>) -> Self {
        Self {
            builder,
            path: Vec::new(),
        }
    }

    pub fn move_to(&mut self, position: Vector2<Pt>) {
        self.path
            .push(PathConstruction::MoveTo(self.builder.transform(position)))
    }

    pub fn line_to(&mut self, position: Vector2<Pt>) {
        self.path
            .push(PathConstruction::LineTo(self.builder.transform(position)))
    }

    pub fn cubic_bezier(&mut self, p1: Vector2<Pt>, p2: Vector2<Pt>, p3: Vector2<Pt>) {
        let p1 = self.builder.transform(p1);
        let p2 = self.builder.transform(p2);
        let p3 = self.builder.transform(p3);

        self.path.push(PathConstruction::CubicBezier { p1, p2, p3 })
    }

    pub fn rect(&mut self, rect: Area<Pt>) {
        self.path
            .push(PathConstruction::Rect(self.builder.transform(rect)))
    }

    pub fn paint(self, fill: Option<FillPaintArgs>, stroke: Option<StrokePaintArgs>) {
        self.builder.push(GraphicsState::SaveState);

        let fill_rule = fill.map(|fill| fill.push(self.builder));
        let close_stroke = stroke.map(|stroke| stroke.push(self.builder));

        for element in self.path {
            self.builder.push(element);
        }

        let path_paint = PathPainting::new(fill_rule, close_stroke);

        self.builder.push(path_paint);

        self.builder.push(GraphicsState::RestoreState);
    }
}

pub struct FillPaintArgs {
    pub color: Color,
    pub fill_rule: FillRule,
}

impl FillPaintArgs {
    fn push(self, builder: &mut StreamBuilder) -> FillRule {
        builder.push(ColorOperation::FillColor(self.color));
        self.fill_rule
    }
}

pub struct StrokePaintArgs {
    pub close: bool,
    pub color: Color,
    pub line_width: Pt,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub miter_limit: Pt,
    pub dash_pattern: (Vec<Pt>, u32),
}

impl StrokePaintArgs {
    fn push(self, builder: &mut StreamBuilder) -> bool {
        builder.push(ColorOperation::StrokeColor(self.color));
        builder.push(GraphicsState::LineWidth(self.line_width));
        builder.push(GraphicsState::LineCap(self.line_cap));
        builder.push(GraphicsState::LineJoin(self.line_join));
        builder.push(GraphicsState::MiterLimit(self.miter_limit));
        builder.push(GraphicsState::DashPattern(self.dash_pattern.0, self.dash_pattern.1));
        self.close
    }
}
