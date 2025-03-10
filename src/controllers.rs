//! `Controller` submit command to the top to the widget tree to update

use druid::{widget::Controller, Env, UpdateCtx, Widget};

use crate::data::*;
use crate::delegate::SAVE;

pub struct TodoItemController;

impl<W: Widget<TodoItem>> Controller<TodoItem, W> for TodoItemController {
    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut UpdateCtx,
        old_data: &TodoItem,
        data: &TodoItem,
        env: &Env,
    ) {
        if old_data.done != data.done {
            // send command to the top of the widget tree
            ctx.submit_command(SAVE);
        }
        child.update(ctx, old_data, data, env);
    }
}
