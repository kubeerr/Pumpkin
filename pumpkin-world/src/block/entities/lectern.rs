use crate::block::entities::BlockEntity;
use crate::inventory::{Clearable, Inventory, InventoryFuture, split_stack};
use crossbeam::epoch::Atomic;
use pumpkin_data::item_stack::ItemStack;
use pumpkin_nbt::compound::NbtCompound;
use pumpkin_util::math::position::BlockPos;
use std::any::Any;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use tokio::sync::Mutex;

pub struct LecternBlockEntity {
    position: BlockPos,
    dirty: AtomicBool,

    book: Arc<Mutex<ItemStack>>,
    current_page: AtomicI32,
    page_count: AtomicI32,
}

impl BlockEntity for LecternBlockEntity {
    fn write_nbt<'a>(
        &'a self,
        nbt: &'a mut NbtCompound,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        self.write_inventory_nbt(nbt, true)
    }

    fn from_nbt(nbt: &NbtCompound, position: BlockPos) -> Self
    where
        Self: Sized,
    {
        let lectern = Self {
            position,
            dirty: AtomicBool::new(false),
            book: Arc::new(Mutex::new(ItemStack::EMPTY.clone())),
            current_page: AtomicI32::new(0),
            page_count: AtomicI32::new(0),
        };
        let stack = [lectern.book.clone()];

        lectern.read_data(nbt, &stack);
        lectern
    }

    fn resource_location(&self) -> &'static str {
        Self::ID
    }

    fn get_position(&self) -> BlockPos {
        self.position
    }

    fn get_inventory(self: Arc<Self>) -> Option<Arc<dyn Inventory>> {
        Some(self)
    }

    fn is_dirty(&self) -> bool {
        self.dirty.load(Ordering::Relaxed)
    }

    fn clear_dirty(&self) {
        self.dirty.store(false, Ordering::Relaxed);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl LecternBlockEntity {
    pub const ID: &'static str = "minecraft:lectern";
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
