use std::{error::Error, io, time::{Duration, Instant}};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Terminal, Frame,
};

/// ユークリッドアルゴリズムを使用してリズムパターンを生成する関数
fn euclidean_rhythm(pulses: usize, steps: usize) -> Vec<bool> {
    if pulses > steps { return vec![]; }
    if pulses == 0 { return vec![false; steps]; }
    if pulses == steps { return vec![true; steps]; }

    let mut groups: Vec<Vec<bool>> = vec![vec![true]; pulses];
    let mut remainders: Vec<Vec<bool>> = vec![vec![false]; steps - pulses];

    while remainders.len() > 1 {
        let min_len = std::cmp::min(groups.len(), remainders.len());
        for i in 0..min_len {
            groups[i].extend(remainders[i].clone());
        }
        if remainders.len() <= groups.len() {
            let mut next_remainders = Vec::new();
            for i in min_len..groups.len() {
                next_remainders.push(groups[i].clone());
            }
            groups.truncate(min_len);
            remainders = next_remainders;
        } else {
            let mut next_remainders = Vec::new();
            for i in min_len..remainders.len() {
                next_remainders.push(remainders[i].clone());
            }
            remainders = next_remainders;
        }
    }
    let mut result = Vec::with_capacity(steps);
    for group in groups { result.extend(group); }
    for remainder in remainders { result.extend(remainder); }
    result
}

#[derive(Clone)]
struct Part {
    pulses: usize,
    steps: usize,
    pattern: Vec<bool>,
    current_step: usize,
    color: Color,
}

impl Part {
    fn new(pulses: usize, steps: usize, color: Color) -> Self {
        Self {
            pulses,
            steps,
            pattern: euclidean_rhythm(pulses, steps),
            current_step: 0,
            color,
        }
    }

    fn update_pattern(&mut self, new_pulses: usize) {
        if new_pulses <= self.steps {
            self.pulses = new_pulses;
            self.pattern = euclidean_rhythm(self.pulses, self.steps);
        }
    }

    fn tick(&mut self) {
        if self.steps > 0 {
            self.current_step = (self.current_step + 1) % self.steps;
        }
    }
}

// 音楽の進行状況（フェーズ）を管理するステートマシン
#[derive(Debug, Clone, Copy, PartialEq)]
enum Phase {
    Awakening,
    Resonance,
    Proliferation,
    Saturation,
}

struct App {
    parts: Vec<Part>,
    global_tick: usize,
    phase: Phase,
    ticks_per_phase: usize,
    is_running: bool,
}

impl App {
    fn new() -> App {
        // 初期状態：フェーズ1 (Awakening) - 非常にシンプル
        let parts = vec![
            Part::new(2, 16, Color::Cyan),      // Part 0 (Base)
            Part::new(2, 15, Color::Magenta),   // Part 1 (Phase shifted)
            Part::new(2, 14, Color::Yellow),    // Part 2 (Phase shifted)
        ];
        App {
            parts,
            global_tick: 0,
            phase: Phase::Awakening,
            ticks_per_phase: 64, // 64 tickごとにフェーズ進行
            is_running: true,
        }
    }

    fn on_tick(&mut self) {
        self.global_tick += 1;

        // 各パートを1ステップ進める
        for part in &mut self.parts {
            part.tick();
        }

        // フェーズの進行とリセットのロジック (Extensible Design)
        let cycle_tick = self.global_tick % (self.ticks_per_phase * 4);
        let new_phase = match cycle_tick {
            t if t < self.ticks_per_phase => Phase::Awakening,
            t if t < self.ticks_per_phase * 2 => Phase::Resonance,
            t if t < self.ticks_per_phase * 3 => Phase::Proliferation,
            _ => Phase::Saturation,
        };

        if self.phase != new_phase {
            self.phase = new_phase;
            self.apply_phase_changes();
        }

        // 徐々にパルスを増やす処理（Proliferation / Saturation）
        if self.phase == Phase::Proliferation || self.phase == Phase::Saturation {
            if self.global_tick % 16 == 0 { // 16tickごとに少しずつ複雑に
                for part in &mut self.parts {
                    let new_pulses = std::cmp::min(part.pulses + 1, part.steps);
                    part.update_pattern(new_pulses);
                }
            }
        }
    }

    fn apply_phase_changes(&mut self) {
        match self.phase {
            Phase::Awakening => {
                // リセット：最初のシンプルな状態に戻る
                self.parts[0].update_pattern(2);
                self.parts[1].update_pattern(2);
                self.parts[2].update_pattern(2);
            }
            Phase::Resonance => {
                // 少しだけ複雑に
                self.parts[0].update_pattern(5);
                self.parts[1].update_pattern(5);
                self.parts[2].update_pattern(5);
            }
            Phase::Proliferation => {
                // 段階的に増やすフェーズへの突入時
                self.parts[0].update_pattern(7);
                self.parts[1].update_pattern(7);
                self.parts[2].update_pattern(7);
            }
            Phase::Saturation => {
                // さらにカオスへ向かう
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new();
    let tick_rate = Duration::from_millis(150); // テンポ調整
    let res = run_app(&mut terminal, &mut app, tick_rate);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    while app.is_running {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    app.is_running = false;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(10),
            ]
            .as_ref(),
        )
        .split(f.area());

    // ヘッダー情報
    let phase_str = match app.phase {
        Phase::Awakening => "Phase 1: Awakening (Simple)",
        Phase::Resonance => "Phase 2: Resonance (Phase Shifting)",
        Phase::Proliferation => "Phase 3: Proliferation (Complexity increases)",
        Phase::Saturation => "Phase 4: Saturation & Chaos",
    };

    let header_text = format!("Rhythmic Tides | {} | Tick: {} | Press 'q' to quit", phase_str, app.global_tick);
    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .style(Style::default().fg(Color::White));
    f.render_widget(header, chunks[0]);

    // サークルUI描画
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Rhythm Circles (Concentric)"))
        .x_bounds([-100.0, 100.0])
        .y_bounds([-100.0, 100.0])
        .paint(|ctx| {
            // 中心点
            ctx.draw(&Points {
                coords: &[(0.0, 0.0)],
                color: Color::White,
            });

            // 各パートを同心円状に描画
            for (i, part) in app.parts.iter().enumerate() {
                let radius = 30.0 + (i as f64) * 25.0; // 同心円の半径

                // 円周をうっすら描く（近似）
                for angle_deg in (0..360).step_by(5) {
                    let angle_rad = (angle_deg as f64).to_radians();
                    let x = radius * angle_rad.cos();
                    let y = radius * angle_rad.sin();
                    ctx.draw(&Points {
                        coords: &[(x, y)],
                        color: Color::DarkGray,
                    });
                }

                // ステップごとのパルスを描画
                for step in 0..part.steps {
                    // 時計の12時位置から時計回りに
                    let angle_rad = std::f64::consts::PI / 2.0 - (2.0 * std::f64::consts::PI * (step as f64) / (part.steps as f64));
                    let x = radius * angle_rad.cos();
                    let y = radius * angle_rad.sin();

                    // 現在の再生位置かどうか
                    let is_current = step == part.current_step;

                    if part.pattern[step] {
                        // パルスあり（音が鳴る位置）
                        let color = if is_current { Color::White } else { part.color };
                        // 目立たせるために少し大きな点（複数点）で描画
                        ctx.draw(&Points {
                            coords: &[(x, y), (x+1.0, y), (x-1.0, y), (x, y+1.0), (x, y-1.0)],
                            color,
                        });
                    } else {
                        // 休符の位置
                        let color = if is_current { Color::Gray } else { Color::DarkGray };
                        ctx.draw(&Points {
                            coords: &[(x, y)],
                            color,
                        });
                    }
                }
            }
        });
    f.render_widget(canvas, chunks[1]);

    // 詳細テキスト表示
    let mut details = vec![];
    for (i, part) in app.parts.iter().enumerate() {
        let pattern_str = part.pattern.iter().enumerate().map(|(idx, &p)| {
            if idx == part.current_step {
                if p { "[X]" } else { "[.]" }
            } else {
                if p { " x " } else { " . " }
            }
        }).collect::<Vec<_>>().join("");

        details.push(Line::from(vec![
            Span::styled(format!("Part {}: E({}, {:02}) ", i+1, part.pulses, part.steps), Style::default().fg(part.color)),
            Span::raw(pattern_str),
        ]));
    }

    let info = Paragraph::new(details)
        .block(Block::default().borders(Borders::ALL).title("Pattern Details"));
    f.render_widget(info, chunks[2]);
}
