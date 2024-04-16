use std::{
    fmt::{self, Debug, Formatter},
    ops::Deref,
};

use specta::{
    reference::{inline, Reference},
    DataType, Type, TypeMap,
};
use tauri::{
    command::{CommandArg, CommandItem},
    InvokeError, Runtime, State,
};

pub struct TState<'r, T: Send + Sync + 'static>(pub &'r T);

impl<'r, T: Send + Sync + 'static> TState<'r, T> {
    /// Retrieve a borrow to the underlying value with a lifetime of `'r`.
    /// Using this method is typically unnecessary as `State` implements
    /// [`std::ops::Deref`] with a [`std::ops::Deref::Target`] of `T`.
    #[inline(always)]
    pub fn inner(&self) -> &'r T {
        self.0
    }
}

impl<T: Send + Sync + 'static> Deref for TState<'_, T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        self.0
    }
}

impl<T: Send + Sync + 'static> Clone for TState<'_, T> {
    fn clone(&self) -> Self {
        TState(self.0)
    }
}

impl<'r, T: Send + Sync + Debug> Debug for TState<'r, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TState").field(&self.0).finish()
    }
}

impl<'r, T: Send + Sync + 'static> From<State<'r, T>> for TState<'r, T> {
    fn from(value: State<'r, T>) -> Self {
        Self(value.inner())
    }
}

impl<'r, 'de: 'r, T: Send + Sync + 'static, R: Runtime> CommandArg<'de, R> for TState<'r, T> {
    /// Grabs the [`State`] from the [`CommandItem`]. This will never fail.
    fn from_command(command: CommandItem<'de, R>) -> Result<Self, InvokeError> {
        Ok(command.message.state_ref().try_get::<T>().map(|v| v.into()).unwrap_or_else(|| {
        panic!(
          "state not managed for field `{}` on command `{}`. You must call `.manage()` before using this command",
          command.key, command.name
        )
      }))
    }
}

impl<'r, T: Send + Sync + 'static> Type for TState<'r, T> {
    fn definition(_type_map: &mut TypeMap) -> DataType {
        DataType::Nullable(Box::new(DataType::Unknown))
    }

    fn inline(type_map: &mut TypeMap, _generics: &[DataType]) -> DataType {
        Self::definition(type_map)
    }

    fn reference(type_map: &mut TypeMap, generics: &[DataType]) -> Reference {
        inline::<Self>(type_map, generics)
    }
}
