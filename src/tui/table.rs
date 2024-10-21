use vec;
use ratatui::{prelude::*, widgets::*};
use sea_orm::{DbErr};
use crate::dao::{*};
use crate::entities::courses;

pub async fn show_table() -> Result<Table<'static>, DbErr> {
    let header = Row::new(vec![
        Cell::from("H1"),
        Cell::from("H2"),
        Cell::from("H3"),
    ]);
    let mut rows = vec![];
    // let rows = [Row::new(vec!["Cell1", "Cell2", "Cell3"])];
    // Columns widths are constrained in the same way as Layout...
    let widths = [
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(10),
    ];
    let res = course_dao::list(6690, 1, 100).await;
    let res = order_dao::list(6690, 1, 100).await;
    // 如果查询出错，则直接返回
    if let Err(e) = res {
        return Err(e);
    }
    for o in res.unwrap() {
        let row = Row::new(vec![
            o.shop_name.clone(),
            o.username.clone(),
            o.card_name.clone(),
        ]);
        rows.push(row);  // 追加到 rows
    }
    let table = Table::new(rows, widths)
        .header(header)
        // ...and they can be separated by a fixed spacing.
        .column_spacing(1)
        // You can set the style of the entire Table.
        .style(Style::new().blue())
        .bold()
        // It has an optional header, which is simply a Row always visible at the top.
        // .header(
        //     Row::new(vec!["Col1", "Col2", "Col3"])
        //         .style(Style::new().bold())
        //         // To add space between the header and the rest of the rows, specify the margin
        //         .bottom_margin(1),
        // )
        // It has an optional footer, which is simply a Row always visible at the bottom.
        // .footer(Row::new(vec!["Updated on Dec 28"]))
        // As any other widget, a Table can be wrapped in a Block.
        .block(Block::new().title("哈哈哈哈"))
        // The selected row and its content can also be styled.
        .highlight_style(Style::new().reversed())
        // ...and potentially show a symbol in front of the selection.
        .highlight_symbol(">>");
    Ok(table)
}


