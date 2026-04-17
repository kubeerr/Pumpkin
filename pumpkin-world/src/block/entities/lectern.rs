use crate::inventory::{Clearable, Inventory, InventoryFuture, split_stack};
use pumpkin_data::item_stack::ItemStack;
use pumpkin_util::math::position::BlockPos;
use std::any::Any;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::sync::Mutex;

pub struct LecternBlockEntity {
    position: BlockPos,
    book: Arc<Mutex<ItemStack>>,
    dirty: AtomicBool,
}

impl LecternBlockEntity {
    pub fn on_book_removed(&self) {
        todo!()
    }
}

impl Clearable for LecternBlockEntity {
    fn clear(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async {
            *self.book.lock().await = ItemStack::EMPTY.clone();
            self.on_book_removed();
            self.mark_dirty();
        })
    }
}

impl Inventory for LecternBlockEntity {
    fn size(&self) -> usize {
        1
    }

    fn is_empty(&self) -> InventoryFuture<'_, bool> {
        Box::pin(async move { self.book.lock().await.is_empty() })
    }

    fn get_stack(&self, slot: usize) -> InventoryFuture<'_, Arc<Mutex<ItemStack>>> {
        Box::pin(async move {
            if slot != 0 {
                return Arc::new(Mutex::new(ItemStack::EMPTY.clone()));
            }
            self.book.clone()
        })
    }

    fn remove_stack(&self, slot: usize) -> InventoryFuture<'_, ItemStack> {
        Box::pin(async move {
            if slot != 0 {
                ItemStack::EMPTY.clone()
            } else {
                let mut removed = ItemStack::EMPTY.clone();
                let mut guard = self.book.lock().await;
                std::mem::swap(&mut removed, &mut *guard);
                self.mark_dirty();
                self.on_book_removed();
                removed
            }
        })
    }

    fn remove_stack_specific(&self, slot: usize, amount: u8) -> InventoryFuture<'_, ItemStack> {
        self.remove_stack(slot) // same thing since it's only one slot with a stack size of 1
    }

    fn set_stack(&self, slot: usize, stack: ItemStack) -> InventoryFuture<'_, ()> {
        Box::pin(async {})
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
