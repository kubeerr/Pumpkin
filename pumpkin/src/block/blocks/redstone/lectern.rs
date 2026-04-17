use crate::block::registry::BlockActionResult;
use crate::block::{BlockBehaviour, BlockFuture, NormalUseArgs, OnPlaceArgs, UseWithItemArgs};
use pumpkin_data::block_properties::{BlockProperties, LecternLikeProperties};
use pumpkin_macros::pumpkin_block;
use pumpkin_world::BlockStateId;
use pumpkin_world::world::BlockAccessor;

#[pumpkin_block("minecraft:lectern")]
pub struct LecternBlock;

impl BlockBehaviour for LecternBlock {
    fn on_place<'a>(&'a self, args: OnPlaceArgs<'a>) -> BlockFuture<'a, BlockStateId> {
        Box::pin(async move {
            let mut props = LecternLikeProperties::default(args.block);
            props.facing = args
                .player
                .living_entity
                .entity
                .get_horizontal_facing()
                .opposite();
            props.to_state_id(args.block)
        })
    }

    fn normal_use<'a>(&'a self, args: NormalUseArgs<'a>) -> BlockFuture<'a, BlockActionResult> {
        Box::pin(async move {
            let state_id = args.world.get_block_state(args.position).await.id;
            let props = LecternLikeProperties::from_state_id(state_id, args.block);
            if !props.has_book {
                return BlockActionResult::Pass;
            }
            todo!("implement book interface")
        })
    }

    fn use_with_item<'a>(
        &'a self,
        args: UseWithItemArgs<'a>,
    ) -> BlockFuture<'a, BlockActionResult> {
        Box::pin(async move {
            let state_id = args.world.get_block_state_id(args.position).await;
            let props = LecternLikeProperties::from_state_id(state_id, args.block);
            if props.has_book {
                return BlockActionResult::PassToDefaultBlockAction;
            }

            let mut item_stack = args.item_stack.lock().await;
            let item_id = item_stack.item.id;
            todo!()
        })
    }
}
