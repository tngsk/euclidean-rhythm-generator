mod rhythm;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{error::Error, io};
use rhythm::{euclidean_rhythm, rotate_rhythm, rhythm_to_string};

fn main() -> Result<(), Box<dyn Error>> {
    // ターミナルのセットアップ
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // アプリケーションの状態
    let examples = vec![
        (3, 8, "Cuban tresillo"),
        (5, 8, "Cuban cinquillo"),
        (5, 16, "Bossa-nova"),
        (7, 16, "Brazilian Samba"),
    ];

    let mut selected_example = 0;

    // メインループ
    loop {
        terminal.draw(|f| {
            let size = f.area();

            // レイアウトの定義
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3), // ヘッダー
                        Constraint::Min(2),    // メインコンテンツ (リズム表示)
                        Constraint::Length(3), // フッター (操作説明)
                    ]
                    .as_ref(),
                )
                .split(size);

            // ヘッダーの描画
            let header_text = format!(" Euclidean Rhythm Generator - Example {}/{} ", selected_example + 1, examples.len());
            let header = Paragraph::new(header_text)
                .style(Style::default().fg(Color::Cyan))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(header, chunks[0]);

            // メインコンテンツ（リズム）の描画
            let (pulses, steps, name) = examples[selected_example];
            let rhythm = euclidean_rhythm(pulses, steps);

            let mut rhythm_display = vec![
                format!("Name: {}", name),
                format!("Base: E({}, {}) = [{}]", pulses, steps, rhythm_to_string(&rhythm)),
                String::from(""),
                String::from("Rotations:"),
            ];

            for i in 1..steps {
                let rotated = rotate_rhythm(&rhythm, i);
                rhythm_display.push(format!("  Rotation {}: [{}]", i, rhythm_to_string(&rotated)));
            }

            let text: Text = rhythm_display.join("\n").into();
            let content = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL).title(" Visual Rhythm Presentation "))
                .style(Style::default().fg(Color::White));
            f.render_widget(content, chunks[1]);

            // フッターの描画
            let footer_text = " [q] Quit | [Left/Right] Change Example ";
            let footer = Paragraph::new(footer_text)
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[2]);

        })?;

        // イベント処理
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right => {
                        selected_example = (selected_example + 1) % examples.len();
                    }
                    KeyCode::Left => {
                        if selected_example > 0 {
                            selected_example -= 1;
                        } else {
                            selected_example = examples.len() - 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // ターミナルのクリーンアップ
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
