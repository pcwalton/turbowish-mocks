use derive_more::{Constructor, From};
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Paragraph, Table, Widget};

static FRAME_UPPER_LEFT_SYMBOL: &'static str = "╭";
static FRAME_UPPER_RIGHT_SYMBOL: &'static str = "╮";
static FRAME_LOWER_RIGHT_SYMBOL: &'static str = "╯";
static FRAME_LOWER_LEFT_SYMBOL: &'static str = "╰";
static FRAME_HORIZONTAL_SYMBOL: &'static str = "─";
static FRAME_VERTICAL_SYMBOL: &'static str = "│";
static POWERLINE_MAIN_SEPARATOR_LABEL_LTR: &'static str = "\u{e0b0}";
static POWERLINE_SUB_SEPARATOR_LABEL_LTR: &'static str = "\u{e0b1}";
static POWERLINE_MAIN_SEPARATOR_LABEL_RTL: &'static str = "\u{e0b2}";
static POWERLINE_SUB_SEPARATOR_LABEL_RTL: &'static str = "\u{e0b3}";
static SCROLLBAR_UP_SYMBOL: &'static str = "\u{f431}";
static SCROLLBAR_DOWN_SYMBOL: &'static str = "\u{f433}";

static DOTS: [char; 256] = [
    '⠀', '⡀', '⠄', '⡄', '⠂', '⡂', '⠆', '⡆', '⠁', '⡁', '⠅', '⡅', '⠃', '⡃', '⠇', '⡇', '⢀', '⣀', '⢄',
    '⣄', '⢂', '⣂', '⢆', '⣆', '⢁', '⣁', '⢅', '⣅', '⢃', '⣃', '⢇', '⣇', '⠠', '⡠', '⠤', '⡤', '⠢', '⡢',
    '⠦', '⡦', '⠡', '⡡', '⠥', '⡥', '⠣', '⡣', '⠧', '⡧', '⢠', '⣠', '⢤', '⣤', '⢢', '⣢', '⢦', '⣦', '⢡',
    '⣡', '⢥', '⣥', '⢣', '⣣', '⢧', '⣧', '⠐', '⡐', '⠔', '⡔', '⠒', '⡒', '⠖', '⡖', '⠑', '⡑', '⠕', '⡕',
    '⠓', '⡓', '⠗', '⡗', '⢐', '⣐', '⢔', '⣔', '⢒', '⣒', '⢖', '⣖', '⢑', '⣑', '⢕', '⣕', '⢓', '⣓', '⢗',
    '⣗', '⠰', '⡰', '⠴', '⡴', '⠲', '⡲', '⠶', '⡶', '⠱', '⡱', '⠵', '⡵', '⠳', '⡳', '⠷', '⡷', '⢰', '⣰',
    '⢴', '⣴', '⢲', '⣲', '⢶', '⣶', '⢱', '⣱', '⢵', '⣵', '⢳', '⣳', '⢷', '⣷', '⠈', '⡈', '⠌', '⡌', '⠊',
    '⡊', '⠎', '⡎', '⠉', '⡉', '⠍', '⡍', '⠋', '⡋', '⠏', '⡏', '⢈', '⣈', '⢌', '⣌', '⢊', '⣊', '⢎', '⣎',
    '⢉', '⣉', '⢍', '⣍', '⢋', '⣋', '⢏', '⣏', '⠨', '⡨', '⠬', '⡬', '⠪', '⡪', '⠮', '⡮', '⠩', '⡩', '⠭',
    '⡭', '⠫', '⡫', '⠯', '⡯', '⢨', '⣨', '⢬', '⣬', '⢪', '⣪', '⢮', '⣮', '⢩', '⣩', '⢭', '⣭', '⢫', '⣫',
    '⢯', '⣯', '⠘', '⡘', '⠜', '⡜', '⠚', '⡚', '⠞', '⡞', '⠙', '⡙', '⠝', '⡝', '⠛', '⡛', '⠟', '⡟', '⢘',
    '⣘', '⢜', '⣜', '⢚', '⣚', '⢞', '⣞', '⢙', '⣙', '⢝', '⣝', '⢛', '⣛', '⢟', '⣟', '⠸', '⡸', '⠼', '⡼',
    '⠺', '⡺', '⠾', '⡾', '⠹', '⡹', '⠽', '⡽', '⠻', '⡻', '⠿', '⡿', '⢸', '⣸', '⢼', '⣼', '⢺', '⣺', '⢾',
    '⣾', '⢹', '⣹', '⢽', '⣽', '⢻', '⣻', '⢿', '⣿',
];

#[derive(From)]
pub enum AnyWidget<'a> {
    BarChart(BarChart<'a>),
    BoxFrame(BoxFrame<'a>),
    Paragraph(Paragraph<'a>),
    Powerline(Powerline<'a>),
    Scrollbar(Scrollbar),
    SegmentedControl(SegmentedControl<'a>),
    Table(Table<'a>),
}

impl<'a> Widget for AnyWidget<'a> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        match self {
            AnyWidget::BarChart(widget) => widget.render(area, buffer),
            AnyWidget::BoxFrame(widget) => widget.render(area, buffer),
            AnyWidget::Paragraph(widget) => widget.render(area, buffer),
            AnyWidget::Powerline(widget) => widget.render(area, buffer),
            AnyWidget::Scrollbar(widget) => widget.render(area, buffer),
            AnyWidget::SegmentedControl(widget) => widget.render(area, buffer),
            AnyWidget::Table(widget) => widget.render(area, buffer),
        }
    }
}

// Segmented controls

#[derive(Constructor)]
pub struct SegmentedControl<'a> {
    labels: &'a [&'a str],
    selected_index: u32,
    bg_color: Color,
    fg_color: Color,
}

impl<'a> Widget for SegmentedControl<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut x = area.x;
        let left_edge_style = if self.selected_index == 0 {
            Style::default().fg(self.fg_color)
        } else {
            Style::default().fg(self.bg_color)
        };
        buf.set_string(x, area.y, "", left_edge_style);
        x += 1;

        for (index, label) in self.labels.iter().enumerate() {
            let style = if index == self.selected_index as usize {
                Style::default().fg(self.bg_color).bg(self.fg_color)
            } else {
                Style::default().fg(self.fg_color).bg(self.bg_color)
            };
            if index > 0 {
                buf.set_string(x, area.y, " ", style);
                x += 1;
            }
            buf.set_string(x, area.y, label, style);
            x += label.chars().count() as u16;
            if index < self.labels.len() - 1 {
                buf.set_string(x, area.y, " ", style);
                x += 1;
            }
        }

        let right_edge_style = if self.selected_index as usize == self.labels.len() - 1 {
            Style::default().fg(self.fg_color)
        } else {
            Style::default().fg(self.bg_color)
        };
        buf.set_string(x, area.y, "", right_edge_style);
    }
}

// Bar chart

#[derive(Constructor)]
pub struct BarChart<'a> {
    data: &'a [f32],
    min_y: f32,
    max_y: f32,
    color: Color,
}

impl<'a> Widget for BarChart<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let y_range = self.max_y - self.min_y;
        let (mut string, mut current_char) = (String::new(), 0);
        let mut x = 0;
        while x < self.data.len() {
            if x > 0 && x % 2 == 0 {
                string.push(DOTS[current_char as usize]);
                current_char = 0;
            }
            let height_norm = clamp((self.data[x] - self.min_y) / y_range, 0.0, 1.0);
            let height = (height_norm * 4.0).round() as u32;
            current_char = (current_char << 4) | ((1 << height) - 1);
            x += 1;
        }
        if x % 2 == 1 {
            string.push(DOTS[current_char as usize]);
        }

        buf.set_string(area.x, area.y, string, Style::default().fg(self.color));
    }
}

// Scrollbar

#[derive(Constructor)]
pub struct Scrollbar {
    min_val: f32,
    max_val: f32,
    min_range: f32,
    max_range: f32,
    color: Color,
}

impl Widget for Scrollbar {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let mut min_val = (self.min_val - self.min_range) / (self.max_range - self.min_range);
        let mut max_val = (self.max_val - self.min_range) / (self.max_range - self.min_range);
        min_val = clamp(min_val, 0.0, 1.0);
        max_val = clamp(max_val, 0.0, 1.0);
        let min_pos = (min_val * (area.height - 2) as f32).floor() as u16 + area.y + 1;
        let max_pos = (max_val * (area.height - 2) as f32).ceil() as u16 + area.y + 1;

        let style = Style::default().fg(self.color);
        buffer.set_string(area.x, area.y, SCROLLBAR_UP_SYMBOL, style);
        for y in (area.y + 1)..(area.bottom() - 1) {
            let string = if y >= min_pos && y <= max_pos {
                "█"
            } else {
                "░"
            };
            buffer.set_string(area.x, y, string, style);
        }
        buffer.set_string(area.x, area.bottom() - 1, SCROLLBAR_DOWN_SYMBOL, style);
    }
}

// Powerline

pub struct Powerline<'a> {
    pub labels: &'a [&'a str],
    pub main_color: Color,
    pub sub_color: Color,
    pub sub_sub_bg_color: Color,
    pub sub_sub_fg_color: Color,
    pub sub_separator_color: Color,
    pub direction: PowerlineDirection,
    pub main_visibility: MainVisibility,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PowerlineDirection {
    LeftToRight,
    RightToLeft,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MainVisibility {
    Visible,
    Invisible,
}

impl<'a> Widget for Powerline<'a> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let mut x = match self.direction {
            PowerlineDirection::LeftToRight => area.x,
            PowerlineDirection::RightToLeft => area.right(),
        };
        for (index, label) in self.labels.iter().enumerate() {
            let style = match (index, self.main_visibility) {
                (0, MainVisibility::Visible) => Style::default()
                    .bg(self.main_color)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
                (1, MainVisibility::Visible) | (0, MainVisibility::Invisible) => {
                    Style::default().bg(self.sub_color).fg(Color::Black)
                }
                _ => Style::default()
                    .bg(self.sub_sub_bg_color)
                    .fg(self.sub_sub_fg_color),
            };
            write_and_advance(&mut x, area.y, " ", style, buffer, self.direction);
            write_and_advance(&mut x, area.y, label, style, buffer, self.direction);
            write_and_advance(&mut x, area.y, " ", style, buffer, self.direction);

            let (separator_style, separator_is_sub);
            match (index, self.main_visibility) {
                (0, MainVisibility::Visible) => {
                    separator_style = Style::default().bg(self.sub_color).fg(self.main_color);
                    separator_is_sub = false;
                }
                (1, MainVisibility::Visible) | (0, MainVisibility::Invisible) => {
                    separator_style = Style::default()
                        .bg(self.sub_sub_bg_color)
                        .fg(self.sub_color);
                    separator_is_sub = false;
                }
                (index, _) if index < self.labels.len() - 1 => {
                    separator_style = Style::default()
                        .bg(self.sub_sub_bg_color)
                        .fg(self.sub_separator_color);
                    separator_is_sub = true;
                }
                _ => {
                    separator_style = Style::default().fg(self.sub_sub_bg_color);
                    separator_is_sub = false;
                }
            }

            let separator_label = match (separator_is_sub, self.direction) {
                (false, PowerlineDirection::LeftToRight) => POWERLINE_MAIN_SEPARATOR_LABEL_LTR,
                (true, PowerlineDirection::LeftToRight) => POWERLINE_SUB_SEPARATOR_LABEL_LTR,
                (false, PowerlineDirection::RightToLeft) => POWERLINE_MAIN_SEPARATOR_LABEL_RTL,
                (true, PowerlineDirection::RightToLeft) => POWERLINE_SUB_SEPARATOR_LABEL_RTL,
            };

            write_and_advance(
                &mut x,
                area.y,
                separator_label,
                separator_style,
                buffer,
                self.direction,
            );
        }

        fn write_and_advance(
            x: &mut u16,
            y: u16,
            string: &str,
            style: Style,
            buffer: &mut Buffer,
            direction: PowerlineDirection,
        ) {
            let string_length = string.chars().count() as u16;
            if direction == PowerlineDirection::RightToLeft {
                *x -= string_length;
            }
            buffer.set_string(*x, y, string, style);
            if direction == PowerlineDirection::LeftToRight {
                *x += string_length;
            }
        }
    }
}

// Frame

pub struct BoxFrame<'a> {
    pub label: &'a str,
    pub border_color: Color,
    pub text_color: Color,
}

impl<'a> Widget for BoxFrame<'a> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let mut top_string = FRAME_UPPER_LEFT_SYMBOL.to_owned();
        let mut bottom_string = FRAME_LOWER_LEFT_SYMBOL.to_owned();
        for _ in 1..(area.width - 1) {
            top_string.push_str(FRAME_HORIZONTAL_SYMBOL);
            bottom_string.push_str(FRAME_HORIZONTAL_SYMBOL);
        }
        top_string.push_str(FRAME_UPPER_RIGHT_SYMBOL);
        bottom_string.push_str(FRAME_LOWER_RIGHT_SYMBOL);

        let border_style = Style::default().fg(self.border_color);
        buffer.set_string(area.x, area.y, &top_string, border_style);
        buffer.set_string(area.x, area.bottom() - 1, &bottom_string, border_style);
        for y in (area.y + 1)..(area.bottom() - 1) {
            buffer.set_string(area.x, y, FRAME_VERTICAL_SYMBOL, border_style);
            buffer.set_string(area.right() - 1, y, FRAME_VERTICAL_SYMBOL, border_style);
        }

        let text_style = Style::default()
            .fg(self.text_color)
            .add_modifier(Modifier::BOLD);
        buffer.set_string(area.x + 2, area.y, " ", text_style);
        buffer.set_string(area.x + 3, area.y, &self.label, text_style);
        buffer.set_string(
            area.x + 3 + self.label.chars().count() as u16,
            area.y,
            " ",
            text_style,
        );
    }
}

fn clamp(x: f32, min_val: f32, max_val: f32) -> f32 {
    if x < min_val {
        min_val
    } else if x > max_val {
        max_val
    } else {
        x
    }
}
