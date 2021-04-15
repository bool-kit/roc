use crate::editor::ed_error::EdResult;
use crate::editor::mvc::app_update::InputOutcome;
use crate::editor::mvc::ed_model::EdModel;
use crate::editor::slow_pool::MarkNodeId;
use crate::lang::ast::Expr2;
use crate::lang::pool::NodeId;
use crate::lang::pool::PoolStr;
use crate::ui::text::lines::SelectableLines;

pub fn update_invalid_lookup(
    input_str: &str,
    old_pool_str: &PoolStr,
    curr_mark_node_id: MarkNodeId,
    ast_node_id: NodeId<Expr2>,
    ed_model: &mut EdModel,
) -> EdResult<InputOutcome> {
    if input_str.chars().all(|ch| ch.is_ascii_alphanumeric()) {
        let old_caret_pos = ed_model.get_caret();
        let mut new_lookup_str = String::new();

        new_lookup_str.push_str(old_pool_str.as_str(ed_model.module.env.pool));

        let caret_offset = ed_model
            .grid_node_map
            .get_offset_to_node_id(ed_model.get_caret(), curr_mark_node_id)?;

        new_lookup_str.insert_str(caret_offset, input_str);

        let new_pool_str = PoolStr::new(&new_lookup_str, &mut ed_model.module.env.pool);

        // update AST
        ed_model
            .module
            .env
            .pool
            .set(ast_node_id, Expr2::InvalidLookup(new_pool_str));

        // update MarkupNode
        let curr_mark_node_mut = ed_model.markup_node_pool.get_mut(curr_mark_node_id);
        let content_str_mut = curr_mark_node_mut.get_content_mut()?;
        content_str_mut.insert_str(caret_offset, input_str);

        // update caret
        ed_model.simple_move_carets_right(input_str.len());

        // update GridNodeMap and CodeLines
        ed_model.insert_between_line(
            old_caret_pos.line,
            old_caret_pos.column,
            input_str,
            curr_mark_node_id,
        )?;

        Ok(InputOutcome::Accepted)
    } else {
        Ok(InputOutcome::Ignored)
    }
}