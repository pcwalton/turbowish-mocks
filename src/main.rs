use crate::widgets::{
    BarChart, BoxFrame, MainVisibility, Powerline, PowerlineDirection, Scrollbar, SegmentedControl,
};
use better_panic::Settings;
use chrono::Local;
use crossterm::{cursor, execute, terminal};
use std::collections::HashMap;
use std::io::{self, Stdout};
use std::panic;
use stretch::geometry::{Point, Rect, Size};
use stretch::node::Node;
use stretch::number::Number;
use stretch::result::Layout;
use stretch::style::{AlignItems, Dimension, FlexDirection, Style};
use stretch::Stretch;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Rect as TuiRect};
use tui::style::{Color, Modifier, Style as TuiStyle};
use tui::text::{Span, Spans};
use tui::widgets::{Cell, Paragraph, Row, Table};
use tui::{Frame, Terminal};
use widgets::AnyWidget;

mod widgets;

fn main() -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    panic::set_hook(Box::new(move |panic_info| {
        let mut stdout = io::stdout();
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
        execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

        execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
        execute!(stdout, cursor::Show).unwrap();

        terminal::disable_raw_mode().unwrap();
        Settings::auto().create_panic_handler()(panic_info);
    }));

    terminal.draw(|frame| draw_frame(frame)).unwrap();
    Ok(())
}

static TITLE_LABEL: &'static str = "ﴱ Tokio";
static TITLE_BAR_RUNTIME_COUNT_LABELS: [&'static str; 2] = ["runtime", "runtimes"];
static TITLE_BAR_THREAD_COUNT_LABELS: [&'static str; 2] = ["thread", "threads"];
static MENU_BUTTON_LABEL: &'static str = "☰ Menu";
static TIME_FORMAT: &'static str = "%x %r";
static PERFORMANCE_LABEL: &'static str = "Performance";
static PERFORMANCE_RUN_PERCENT_TIME_LABEL: &'static str = "Runtime";
static PERFORMANCE_DEPTH_LABEL: &'static str = "Sched. depth";
static PERFORMANCE_POLL_TIME_LABEL: &'static str = "Poll time";
static PERFORMANCE_WAKE_TIME_LABEL: &'static str = "Wake time";
static PERFORMANCE_LABELS: [&'static str; 4] = [
    PERFORMANCE_RUN_PERCENT_TIME_LABEL,
    PERFORMANCE_DEPTH_LABEL,
    PERFORMANCE_POLL_TIME_LABEL,
    PERFORMANCE_WAKE_TIME_LABEL,
];
static PERFORMANCE_EXPAND_LABEL: &'static str = "\u{fa4e}";
static TASKS_LABEL: &'static str = "Tasks";
static TASKS_TAB_LABEL_ALL: &'static str = "All";
static TASKS_TAB_LABEL_RUNNING: &'static str = "\u{f04b} Running";
static TASKS_TAB_LABEL_SLEEPING: &'static str = "\u{f04c} Sleeping";
static TASKS_TAB_LABEL_DEADLOCKED: &'static str = "\u{f071} Deadlocked";
static TASKS_TAB_LABELS: [&'static str; 4] = [
    TASKS_TAB_LABEL_ALL,
    TASKS_TAB_LABEL_RUNNING,
    TASKS_TAB_LABEL_SLEEPING,
    TASKS_TAB_LABEL_DEADLOCKED,
];
static TASKS_VIEW_MODE_LABEL_FLAT: &'static str = "\u{f03a}";
static TASKS_VIEW_MODE_LABEL_TREE: &'static str = "\u{fb44}";
static TASKS_VIEW_MODE_LABELS: [&'static str; 2] =
    [TASKS_VIEW_MODE_LABEL_FLAT, TASKS_VIEW_MODE_LABEL_TREE];
static TASKS_TABLE_STATUS_RUNNING: &'static str = "\u{f04b}";
static TASKS_TABLE_STATUS_SLEEPING: &'static str = "\u{f04c}";
static TASKS_TABLE_STATUS_DEADLOCKED: &'static str = "\u{f071}";
static TASKS_TABLE_BUTTON_OPEN: &'static str = "\u{f457}";
static _TASKS_TABLE_BUTTON_CLOSE: &'static str = "\u{f458}";
static TASKS_TABLE_COLUMN_LABEL_ID: &'static str = "ID";
static TASKS_TABLE_COLUMN_LABEL_NAME: &'static str = "Name";
static TASKS_TABLE_COLUMN_LABEL_STATE: &'static str = "State";
static TASKS_TABLE_COLUMN_LABEL_RUN_PERCENT: &'static str = "Run %";
static TASKS_TABLE_COLUMN_LABEL_POLL_MS: &'static str = "Poll";
static TASKS_TABLE_COLUMN_LABEL_WAKE_MS: &'static str = "Wake";
static TASKS_TABLE_COLUMN_LABEL_ATTRIBUTES: &'static str = "Attributes";
static TASKS_TABLE_COLUMN_LABELS: [&'static str; 8] = [
    "",
    TASKS_TABLE_COLUMN_LABEL_ID,
    TASKS_TABLE_COLUMN_LABEL_NAME,
    TASKS_TABLE_COLUMN_LABEL_STATE,
    TASKS_TABLE_COLUMN_LABEL_RUN_PERCENT,
    TASKS_TABLE_COLUMN_LABEL_POLL_MS,
    TASKS_TABLE_COLUMN_LABEL_WAKE_MS,
    TASKS_TABLE_COLUMN_LABEL_ATTRIBUTES,
];
static TASKS_TABLE_COLUMN_WIDTHS: [u16; 7] = [
    3,  // Widgets
    10, // ID
    24, // Name
    5,  // State
    5,  // Run %
    7,  // Poll ms
    7,  // Wake ms
];

static AUTO_SIZE: Size<Dimension> = Size {
    width: Dimension::Auto,
    height: Dimension::Auto,
};

static FAKE_TARGET_LABEL: &'static str = "my_app (412)";
static FAKE_TASK_COUNTS: [u32; 4] = [405, 3, 402, 0];
const FAKE_RUNTIME_COUNT: u32 = 1;
const FAKE_THREAD_COUNT: u32 = 8;

const PERFORMANCE_SEGMENT_VALUE_WIDTH: u16 = 6;

const THEME_COLOR_TITLE_MAIN_COLOR: Color = Color::Rgb(0x88, 0xc0, 0xd0);
const THEME_COLOR_TITLE_SUB_COLOR: Color = Color::Rgb(0x81, 0xa1, 0xc1);
const THEME_COLOR_TITLE_SUB_SUB_BG: Color = Color::Rgb(0x3b, 0x42, 0x52);
const THEME_COLOR_TITLE_SUB_SUB_FG: Color = Color::Rgb(0xe5, 0xe9, 0xf0);
const THEME_COLOR_TITLE_SUB_SEPARATOR_COLOR: Color = Color::DarkGray;
const THEME_COLOR_PERFORMANCE_BOX_FG: Color = Color::Green;
const THEME_COLOR_PERFORMANCE_LABEL: Color = Color::Gray;
const THEME_COLOR_PERFORMANCE_NUMERIC_COLOR: Color = Color::Green;
const THEME_COLOR_PERFORMANCE_MINOR_COLOR: Color = Color::DarkGray;
const THEME_COLOR_PERFORMANCE_GRAPH_COLOR: Color = Color::Green;
const THEME_COLOR_TASKS_BOX_FG: Color = Color::Red;
const THEME_COLOR_TASKS_FILTER_BG: Color = Color::Black; // Color::Rgb(32, 0, 0);
const THEME_COLOR_TASKS_FILTER_FG: Color = Color::Gray; // Color::Red;
const THEME_COLOR_TASKS_TABLE_HEADER_FG: Color = Color::White;
const THEME_COLOR_TASKS_TABLE_OPEN_CELL_COLOR: Color = Color::DarkGray;
const THEME_COLOR_TASKS_TABLE_MINOR_CELL_COLOR: Color = Color::DarkGray;
const THEME_COLOR_TASKS_TABLE_NAME_CELL_COLOR: Color = Color::Yellow;
const THEME_COLOR_TASKS_TABLE_NUMERIC_CELL_COLOR: Color = Color::Green;
const THEME_COLOR_TASKS_TABLE_ATTRIBUTE_KEY_CELL_COLOR: Color = Color::Blue;
const THEME_COLOR_TASKS_TABLE_ATTRIBUTE_VALUE_CELL_COLOR: Color = Color::Yellow;
const THEME_COLOR_TASKS_TABLE_STATUS_RUNNING_COLOR: Color = Color::Green;
const THEME_COLOR_TASKS_TABLE_STATUS_SLEEPING_COLOR: Color = Color::Gray;
const THEME_COLOR_TASKS_TABLE_STATUS_DEADLOCKED_COLOR: Color = Color::Red;
const THEME_COLOR_SCROLLBAR_COLOR: Color = Color::Gray;

type AppFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

fn draw_frame(frame: &mut AppFrame) {
    // Initialize the DOM.
    let mut stretch = Stretch::new();
    let mut renderer = Renderer::new();
    let main_node = stretch
        .new_node(
            Style {
                size: Size::fixed(frame.size().width, frame.size().height - 1),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                ..Default::default()
            },
            vec![],
        )
        .unwrap();

    // Lay out UI.
    let title_bar_layout = TitleBarLayout::layout(&mut stretch, main_node);
    let performance_pane_layout = PerformancePaneLayout::layout(&mut stretch, main_node);
    let tasks_pane_layout = TasksPaneLayout::layout(&mut stretch, main_node);
    stretch
        .compute_layout(
            main_node,
            Size {
                width: Number::Undefined,
                height: Number::Undefined,
            },
        )
        .unwrap();

    // Build title bar.
    let runtime_count_label = format!(
        "{} {}",
        FAKE_RUNTIME_COUNT, TITLE_BAR_RUNTIME_COUNT_LABELS[0]
    );
    let thread_count_label = format!("{} {}", FAKE_THREAD_COUNT, TITLE_BAR_THREAD_COUNT_LABELS[1]);
    let main_powerline_labels = [
        TITLE_LABEL,
        FAKE_TARGET_LABEL,
        &runtime_count_label[..],
        &thread_count_label[..],
    ];
    renderer.build_node(
        title_bar_layout.main_powerline_node,
        Powerline {
            labels: &main_powerline_labels,
            direction: PowerlineDirection::LeftToRight,
            main_visibility: MainVisibility::Visible,
            main_color: THEME_COLOR_TITLE_MAIN_COLOR,
            sub_color: THEME_COLOR_TITLE_SUB_COLOR,
            sub_sub_bg_color: THEME_COLOR_TITLE_SUB_SUB_BG,
            sub_sub_fg_color: THEME_COLOR_TITLE_SUB_SUB_FG,
            sub_separator_color: THEME_COLOR_TITLE_SUB_SEPARATOR_COLOR,
        },
    );
    let time_label = Local::now().format(TIME_FORMAT).to_string();
    let menu_powerline_labels = [MENU_BUTTON_LABEL, &time_label[..]];
    renderer.build_node(
        title_bar_layout.menu_powerline_node,
        Powerline {
            labels: &menu_powerline_labels,
            direction: PowerlineDirection::RightToLeft,
            main_visibility: MainVisibility::Invisible,
            main_color: THEME_COLOR_TITLE_MAIN_COLOR,
            sub_color: THEME_COLOR_TITLE_SUB_COLOR,
            sub_sub_bg_color: THEME_COLOR_TITLE_SUB_SUB_BG,
            sub_sub_fg_color: THEME_COLOR_TITLE_SUB_SUB_FG,
            sub_separator_color: THEME_COLOR_TITLE_SUB_SEPARATOR_COLOR,
        },
    );

    // Render performance values.
    let performance_numeric_style = TuiStyle::default().fg(THEME_COLOR_PERFORMANCE_NUMERIC_COLOR);
    let performance_minor_style = TuiStyle::default().fg(THEME_COLOR_PERFORMANCE_MINOR_COLOR);
    let rendered_performance_values = vec![
        Spans::from(vec![
            Span::styled("23.3", performance_numeric_style),
            Span::styled("%", performance_minor_style),
        ]),
        Spans::from(vec![Span::styled("2.19", performance_numeric_style)]),
        Spans::from(vec![
            Span::styled("1.05", performance_numeric_style),
            Span::styled("ms", performance_minor_style),
        ]),
        Spans::from(vec![
            Span::styled("0.75", performance_numeric_style),
            Span::styled("ms", performance_minor_style),
        ]),
    ];

    // Build performance pane.
    renderer.build_node(
        performance_pane_layout.performance_node,
        BoxFrame {
            label: PERFORMANCE_LABEL,
            border_color: THEME_COLOR_PERFORMANCE_BOX_FG,
            text_color: Color::White,
        },
    );
    renderer.build_node(
        performance_pane_layout.performance_expand_button_node,
        Paragraph::new(PERFORMANCE_EXPAND_LABEL),
    );
    let performance_node_children = stretch
        .children(performance_pane_layout.performance_graphs_container_node)
        .unwrap();
    for performance_segment_index in 0..PERFORMANCE_LABELS.len() {
        let performance_segment_node = performance_node_children[performance_segment_index];
        let performance_segment_children = stretch.children(performance_segment_node).unwrap();
        let performance_segment_label_node = performance_segment_children[0];
        let performance_segment_value_node = performance_segment_children[1];
        let performance_segment_graph_node = performance_segment_children[2];
        renderer.build_node(
            performance_segment_label_node,
            Paragraph::new(PERFORMANCE_LABELS[performance_segment_index])
                .style(TuiStyle::default().fg(THEME_COLOR_PERFORMANCE_LABEL)),
        );
        renderer.build_node(
            performance_segment_value_node,
            Paragraph::new(rendered_performance_values[performance_segment_index].clone()),
        );
        renderer.build_node(
            performance_segment_graph_node,
            BarChart::new(
                &[4.0, 2.0, 7.0, 1.0, 7.0, 8.0, 3.0],
                0.0,
                7.0,
                THEME_COLOR_PERFORMANCE_GRAPH_COLOR,
            ),
        );
    }

    // Build tasks pane.
    renderer.build_node(
        tasks_pane_layout.tasks_node,
        BoxFrame {
            label: TASKS_LABEL,
            border_color: THEME_COLOR_TASKS_BOX_FG,
            text_color: Color::White,
        },
    );

    let mut tab_labels = vec![];
    for label_index in 0..TASKS_TAB_LABELS.len() {
        tab_labels.push(format!(
            "{} ({})",
            TASKS_TAB_LABELS[label_index], FAKE_TASK_COUNTS[label_index]
        ));
    }
    let tab_label_refs: Vec<_> = tab_labels.iter().map(|label| &**label).collect();
    renderer.build_node(
        tasks_pane_layout.tasks_tabs_node,
        SegmentedControl::new(
            &tab_label_refs[..],
            0,
            THEME_COLOR_TASKS_FILTER_BG,
            THEME_COLOR_TASKS_FILTER_FG,
        ),
    );

    renderer.build_node(
        tasks_pane_layout.tasks_view_mode_node,
        SegmentedControl::new(
            &TASKS_VIEW_MODE_LABELS,
            0,
            THEME_COLOR_TASKS_FILTER_BG,
            THEME_COLOR_TASKS_FILTER_FG,
        ),
    );
    renderer.build_node(
        tasks_pane_layout.tasks_scrollbar_node,
        Scrollbar::new(0.0, 1.0, 0.0, 1.0, THEME_COLOR_SCROLLBAR_COLOR),
    );
    let tasks_table_widths: Vec<_> = stretch
        .children(tasks_pane_layout.tasks_table_node)
        .unwrap()
        .iter()
        .map(|&tasks_table_column_node| {
            Constraint::Length(
                stretch
                    .layout(tasks_table_column_node)
                    .unwrap()
                    .to_rect()
                    .width as u16,
            )
        })
        .collect();
    renderer.build_node(
        tasks_pane_layout.tasks_table_node,
        Table::new(vec![
            create_task_table_row(
                "285",
                "connection-handler",
                TaskStatus::Running,
                "24.5",
                "1.41",
                "0.713",
                &[
                    ("remote-address", "127.0.0.1:56723"),
                    ("request-id", "dbabfa1a-f722-41c0-82dc-a02e88e55d2a"),
                ],
            ),
            create_task_table_row(
                "286",
                "connection-handler",
                TaskStatus::Sleeping,
                "1.9",
                "1.14",
                "0.692",
                &[
                    ("remote-address", "127.0.0.1:34135"),
                    ("request-id", "2087d5f8-7275-4179-a0b4-5ed285b0d988"),
                ],
            ),
            create_task_table_row(
                "1",
                "public-accept",
                TaskStatus::Sleeping,
                "0.6",
                "0.13",
                "0.501",
                &[("local-address", "127.0.0.1:8080")],
            ),
            create_task_table_row(
                "0",
                "main",
                TaskStatus::Sleeping,
                "0.0",
                "0.09",
                "0.106",
                &[],
            ),
        ])
        .header(
            Row::new(TASKS_TABLE_COLUMN_LABELS.to_vec()).style(
                TuiStyle::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(THEME_COLOR_TASKS_TABLE_HEADER_FG),
            ),
        )
        .widths(&tasks_table_widths),
    );

    renderer.render(frame, &stretch, main_node, Point { x: 0, y: 0 });
}

struct TitleBarLayout {
    main_powerline_node: Node,
    menu_powerline_node: Node,
}

impl TitleBarLayout {
    fn layout(stretch: &mut Stretch, main_node: Node) -> TitleBarLayout {
        let title_bar_node = stretch.add_new_child(
            main_node,
            Style {
                size: Size::fixed_height(1),
                ..Default::default()
            },
        );
        let main_powerline_node = stretch.add_new_child(
            title_bar_node,
            Style {
                size: AUTO_SIZE,
                flex_grow: 1.0,
                ..Default::default()
            },
        );
        let menu_powerline_node = stretch.add_new_child(
            title_bar_node,
            Style {
                size: Size::fixed_width(MENU_BUTTON_LABEL.chars().count() as u16 + 3),
                ..Default::default()
            },
        );

        TitleBarLayout {
            main_powerline_node,
            menu_powerline_node,
        }
    }
}

struct PerformancePaneLayout {
    performance_node: Node,
    performance_graphs_container_node: Node,
    performance_expand_button_node: Node,
}

impl PerformancePaneLayout {
    fn layout(stretch: &mut Stretch, main_node: Node) -> PerformancePaneLayout {
        let performance_node = stretch.add_new_child(
            main_node,
            Style {
                size: Size::fixed_height(3),
                ..Default::default()
            },
        );
        let performance_inner_container_node = stretch.add_new_child(
            performance_node,
            Style {
                size: AUTO_SIZE,
                padding: Rect::new(1, 1, 1, 1),
                flex_grow: 1.0,
                ..Default::default()
            },
        );
        let performance_graphs_container_node = stretch.add_new_child(
            performance_inner_container_node,
            Style {
                size: Size::fixed_height(1),
                flex_grow: 1.0,
                ..Default::default()
            },
        );
        let performance_expand_button_node = stretch.add_new_child(
            performance_inner_container_node,
            Style {
                size: Size::fixed(2, 1),
                ..Default::default()
            },
        );
        for &performance_label in &PERFORMANCE_LABELS {
            let performance_segment_node = stretch.add_new_child(
                performance_graphs_container_node,
                Style {
                    size: Size::fixed_height(1),
                    padding: Rect::new(0, 1, 0, 1),
                    flex_grow: 1.0,
                    ..Default::default()
                },
            );
            let _performance_segment_label_node = stretch.add_new_child(
                performance_segment_node,
                Style {
                    size: Size::fixed(performance_label.chars().count() as u16, 1),
                    margin: Rect::new(0, 1, 0, 0),
                    ..Default::default()
                },
            );
            let _performance_segment_time_node = stretch.add_new_child(
                performance_segment_node,
                Style {
                    size: Size::fixed(PERFORMANCE_SEGMENT_VALUE_WIDTH, 1),
                    margin: Rect::new(0, 1, 0, 0),
                    ..Default::default()
                },
            );
            let _performance_segment_graph_node = stretch.add_new_child(
                performance_segment_node,
                Style {
                    size: Size::fixed_height(1),
                    flex_grow: 1.0,
                    ..Default::default()
                },
            );
        }

        PerformancePaneLayout {
            performance_node,
            performance_graphs_container_node,
            performance_expand_button_node,
        }
    }
}

struct TasksPaneLayout {
    tasks_node: Node,
    tasks_tabs_node: Node,
    tasks_view_mode_node: Node,
    tasks_table_node: Node,
    tasks_scrollbar_node: Node,
}

impl TasksPaneLayout {
    fn layout(stretch: &mut Stretch, main_node: Node) -> TasksPaneLayout {
        // Lay out tasks pane.
        let tasks_node = stretch.add_new_child(
            main_node,
            Style {
                size: AUTO_SIZE,
                flex_grow: 1.0,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
        );
        let tasks_inner_container_node = stretch.add_new_child(
            tasks_node,
            Style {
                size: AUTO_SIZE,
                padding: Rect::new(1, 1, 1, 1),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
        );
        let tasks_tab_strip_node = stretch.add_new_child(
            tasks_inner_container_node,
            Style {
                size: Size::fixed_height(1),
                ..Default::default()
            },
        );
        let tasks_tabs_node = stretch.add_new_child(
            tasks_tab_strip_node,
            Style {
                size: AUTO_SIZE,
                margin: Rect::new(0, 0, 0, 1),
                flex_grow: 1.0,
                ..Default::default()
            },
        );
        let tasks_view_mode_node = stretch.add_new_child(
            tasks_tab_strip_node,
            Style {
                size: Size::fixed_width(
                    (TASKS_VIEW_MODE_LABEL_FLAT.chars().count()
                        + TASKS_VIEW_MODE_LABEL_TREE.chars().count()
                        + 4) as u16,
                ),
                ..Default::default()
            },
        );
        let tasks_table_container_node = stretch.add_new_child(
            tasks_inner_container_node,
            Style {
                size: AUTO_SIZE,
                margin: Rect::new(0, 0, 0, 1),
                flex_grow: 1.0,
                ..Default::default()
            },
        );
        let tasks_table_node = stretch.add_new_child(
            tasks_table_container_node,
            Style {
                size: AUTO_SIZE,
                flex_grow: 1.0,
                ..Default::default()
            },
        );
        let tasks_scrollbar_node = stretch.add_new_child(
            tasks_table_container_node,
            Style {
                size: Size::fixed_width(1),
                ..Default::default()
            },
        );

        // Lay out tasks table.
        for &table_column_width in
            &TASKS_TABLE_COLUMN_WIDTHS[0..TASKS_TABLE_COLUMN_LABELS.len() - 1]
        {
            let _tasks_table_column_node = stretch.add_new_child(
                tasks_table_node,
                Style {
                    size: Size::fixed_width(table_column_width),
                    ..Default::default()
                },
            );
        }
        let _tasks_table_last_column_node = stretch.add_new_child(
            tasks_table_node,
            Style {
                size: AUTO_SIZE,
                flex_grow: 1.0,
                ..Default::default()
            },
        );

        TasksPaneLayout {
            tasks_node,
            tasks_view_mode_node,
            tasks_tabs_node,
            tasks_table_node,
            tasks_scrollbar_node,
        }
    }
}

trait StretchExt {
    fn add_new_child(&mut self, parent: Node, style: Style) -> Node;
    fn add_single_line_text(&mut self, parent: Node, string: &str) -> Node;
}

impl StretchExt for Stretch {
    fn add_new_child(&mut self, parent: Node, style: Style) -> Node {
        let node = self.new_node(style, vec![]).unwrap();
        self.add_child(parent, node).unwrap();
        node
    }

    fn add_single_line_text(&mut self, parent: Node, string: &str) -> Node {
        self.add_new_child(
            parent,
            Style {
                size: Size::fixed(string.chars().count() as u16, 1),
                ..Default::default()
            },
        )
    }
}

#[allow(dead_code)]
enum TaskStatus {
    Running,
    Sleeping,
    Deadlocked,
}

fn create_task_table_row<'a>(
    id: &'a str,
    name: &'a str,
    status: TaskStatus,
    run_percent: &'a str,
    poll_ms: &'a str,
    wake_ms: &'a str,
    attributes: &'a [(&'a str, &'a str)],
) -> Row<'a> {
    let open_cell_style = TuiStyle::default().fg(THEME_COLOR_TASKS_TABLE_OPEN_CELL_COLOR);
    let minor_cell_style = TuiStyle::default().fg(THEME_COLOR_TASKS_TABLE_MINOR_CELL_COLOR);
    let name_cell_style = TuiStyle::default().fg(THEME_COLOR_TASKS_TABLE_NAME_CELL_COLOR);
    let numeric_cell_style = TuiStyle::default().fg(THEME_COLOR_TASKS_TABLE_NUMERIC_CELL_COLOR);
    let key_cell_style = TuiStyle::default().fg(THEME_COLOR_TASKS_TABLE_ATTRIBUTE_KEY_CELL_COLOR);
    let value_cell_style =
        TuiStyle::default().fg(THEME_COLOR_TASKS_TABLE_ATTRIBUTE_VALUE_CELL_COLOR);

    let mut attribute_spans = vec![];
    for (index, &(key, value)) in attributes.iter().enumerate() {
        if index > 0 {
            attribute_spans.push(Span::styled(", ", minor_cell_style));
        }
        attribute_spans.push(Span::styled(key, key_cell_style));
        attribute_spans.push(Span::styled("=", minor_cell_style));
        attribute_spans.push(Span::styled(value, value_cell_style));
    }

    let (status_label, status_color);
    match status {
        TaskStatus::Running => {
            status_label = TASKS_TABLE_STATUS_RUNNING;
            status_color = THEME_COLOR_TASKS_TABLE_STATUS_RUNNING_COLOR;
        }
        TaskStatus::Sleeping => {
            status_label = TASKS_TABLE_STATUS_SLEEPING;
            status_color = THEME_COLOR_TASKS_TABLE_STATUS_SLEEPING_COLOR;
        }
        TaskStatus::Deadlocked => {
            status_label = TASKS_TABLE_STATUS_DEADLOCKED;
            status_color = THEME_COLOR_TASKS_TABLE_STATUS_DEADLOCKED_COLOR;
        }
    };
    let status_style = TuiStyle::default().fg(status_color);

    Row::new(vec![
        Cell::from(TASKS_TABLE_BUTTON_OPEN).style(open_cell_style),
        Cell::from(id),
        Cell::from(name).style(name_cell_style),
        Cell::from(status_label).style(status_style),
        Cell::from(Spans::from(vec![
            Span::styled(run_percent, numeric_cell_style),
            Span::styled("%", minor_cell_style),
        ])),
        Cell::from(Spans::from(vec![
            Span::styled(poll_ms, numeric_cell_style),
            Span::styled("ms", minor_cell_style),
        ])),
        Cell::from(Spans::from(vec![
            Span::styled(wake_ms, numeric_cell_style),
            Span::styled("ms", minor_cell_style),
        ])),
        Cell::from(Spans::from(attribute_spans)),
    ])
}

struct Renderer<'a> {
    stretch_node_to_widget: HashMap<Node, AnyWidget<'a>>,
    stretch_node_to_bg_color: HashMap<Node, Color>,
}

impl<'a> Renderer<'a> {
    fn new() -> Renderer<'a> {
        Renderer {
            stretch_node_to_widget: HashMap::new(),
            stretch_node_to_bg_color: HashMap::new(),
        }
    }

    fn build_node<W>(&mut self, node: Node, widget: W)
    where
        W: Into<AnyWidget<'a>>,
    {
        self.stretch_node_to_widget.insert(node, widget.into());
    }

    fn render(
        &mut self,
        frame: &mut AppFrame,
        stretch: &Stretch,
        node: Node,
        world_position: Point<u16>,
    ) {
        let local_rect = stretch.layout(node).unwrap().to_rect();
        let local_style = stretch.style(node).unwrap();

        let mut padding_rect = local_rect.clone();
        let local_padding = resolve_padding(local_style.padding);
        padding_rect.x += world_position.x;
        padding_rect.y += world_position.y;

        if let Some(bg_color) = self.stretch_node_to_bg_color.remove(&node) {
            let mut row = String::new();
            for _ in padding_rect.x..padding_rect.right() {
                row.push(' ');
            }
            for y in padding_rect.y..padding_rect.bottom() {
                frame.render_widget(
                    Paragraph::new(&row[..]).style(TuiStyle::default().bg(bg_color)),
                    TuiRect::new(padding_rect.x, y, padding_rect.width, 1),
                );
            }
        }

        if let Some(widget) = self.stretch_node_to_widget.remove(&node) {
            // Determine content rect.
            let mut content_rect = padding_rect.clone();
            content_rect.x += local_padding.start;
            content_rect.y += local_padding.top;
            content_rect.width -= local_padding.start + local_padding.end;
            content_rect.height -= local_padding.top + local_padding.bottom;

            frame.render_widget(widget, content_rect);
        }

        // Recur.
        if let Ok(kids) = stretch.children(node) {
            for kid in kids {
                self.render(
                    frame,
                    stretch,
                    kid,
                    Point {
                        x: padding_rect.x,
                        y: padding_rect.y,
                    },
                )
            }
        }
    }
}

trait ToRect {
    fn to_rect(&self) -> TuiRect;
}

impl ToRect for Layout {
    fn to_rect(&self) -> TuiRect {
        TuiRect {
            x: self.location.x.round() as u16,
            y: self.location.y.round() as u16,
            width: self.size.width.round() as u16,
            height: self.size.height.round() as u16,
        }
    }
}

// Geometry extensions

trait SizeExt {
    fn fixed(x: u16, y: u16) -> Self;
    fn fixed_width(x: u16) -> Self;
    fn fixed_height(y: u16) -> Self;
}

impl SizeExt for Size<Dimension> {
    fn fixed(x: u16, y: u16) -> Self {
        Size {
            width: Dimension::Points(x as f32),
            height: Dimension::Points(y as f32),
        }
    }
    fn fixed_width(x: u16) -> Self {
        Size {
            width: Dimension::Points(x as f32),
            height: Dimension::Auto,
        }
    }
    fn fixed_height(y: u16) -> Self {
        Size {
            width: Dimension::Auto,
            height: Dimension::Points(y as f32),
        }
    }
}

trait RectExt {
    fn new(top: i32, end: i32, bottom: i32, start: i32) -> Self;
}

impl RectExt for Rect<Dimension> {
    fn new(top: i32, end: i32, bottom: i32, start: i32) -> Self {
        Rect {
            start: Dimension::Points(start as f32),
            end: Dimension::Points(end as f32),
            top: Dimension::Points(top as f32),
            bottom: Dimension::Points(bottom as f32),
        }
    }
}

fn resolve_padding(padding: Rect<Dimension>) -> Rect<u16> {
    return Rect {
        start: resolve_padding_dimension(padding.start),
        end: resolve_padding_dimension(padding.end),
        top: resolve_padding_dimension(padding.top),
        bottom: resolve_padding_dimension(padding.bottom),
    };

    fn resolve_padding_dimension(length: Dimension) -> u16 {
        match length {
            Dimension::Auto | Dimension::Undefined | Dimension::Percent(_) => 0,
            Dimension::Points(length) => length as u16,
        }
    }
}
