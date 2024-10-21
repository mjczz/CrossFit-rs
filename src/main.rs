mod entities;
mod dao;
mod route;
mod tui;

use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

use sea_orm::{DbErr};
use crate::dao::{*};
use crate::entities::{*};
use std::{thread, time};
use axum::Error;
use futures::future::ok;
use futures::stream::select_with_strategy;
use ratatui::widgets::List;
use serde::{Deserialize, Serialize};
use crate::route::regist_route;
use crate::tui::table::show_table;

async fn say_world() {
    let ten_millis = time::Duration::from_secs(2);
    thread::sleep(ten_millis);
    println!("world");
}

async fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
                match key.code {
                    KeyCode::Left => {
                        let l = List::new(["lsjf", "slfls"]);
                        terminal.draw(|frame| {
                            frame.render_widget(l, frame.area());
                        })?;
                    }
                    KeyCode::Right => {
                        let ll = show_table().await;
                        if ll.is_ok() {
                            terminal.draw(move |frame| {
                                // let l = List::new(["lsjf", "slfls"]);
                                // frame.render_widget(ll., frame.area());
                                frame.render_widget(ll.unwrap(), frame.area())
                                // let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                                //     .white()
                                //     .on_blue();
                                // frame.render_widget(greeting, frame.area());
                            })?;
                        }
                    }
                    KeyCode::Up => {}
                    KeyCode::Down => {}
                    _ => {}
                }
            }
        }
    }
}

// 使用 tokio::main 来启动异步运行时
#[tokio::main]
async fn main() -> Result<(), Error> {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let op = say_world();
    // This println! comes first
    println!("hello");
    op.await;


    let mut terminal = ratatui::init();
    let rr = terminal.clear();
    if rr.is_err() {
        return Ok(());
    };
    let _app_result = run(terminal).await;
    ratatui::restore();
    // app_result
    Ok(())


    // build our application with a route
    // let app = regist_route().await;
    // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
    // Ok(())
}



