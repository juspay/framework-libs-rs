//! sea-orm support for [`Secret`] and [`StrongSecret`] types.

use sea_orm::{
    ColIdx, QueryResult, TryGetError, TryGetable, TryGetableArray,
    sea_query::{
        ArrayType, ColumnType, Nullable, Value, ValueType, ValueTypeErr, postgres_array::NotU8,
    },
};

use crate::{Secret, Strategy, StrongSecret, ZeroizableSecret};

impl<S, I> ValueType for Secret<S, I>
where
    S: ValueType,
    I: Strategy<S>,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        S::try_from(v).map(Self::new)
    }

    fn type_name() -> String {
        format!("Secret<{}>", S::type_name())
    }

    fn array_type() -> ArrayType {
        S::array_type()
    }

    fn column_type() -> ColumnType {
        S::column_type()
    }

    fn is_option() -> bool {
        S::is_option()
    }

    fn enum_type_name() -> Option<&'static str> {
        S::enum_type_name()
    }
}

impl<S, I> TryGetable for Secret<S, I>
where
    S: TryGetable,
    I: Strategy<S>,
{
    fn try_get_by<Ix: ColIdx>(res: &QueryResult, index: Ix) -> Result<Self, TryGetError> {
        S::try_get_by(res, index).map(Self::new)
    }
}

impl<S, I> Nullable for Secret<S, I>
where
    S: Nullable,
    I: Strategy<S>,
{
    fn null() -> Value {
        S::null()
    }
}

impl<S, I> From<Secret<S, I>> for Value
where
    S: Into<Self>,
    I: Strategy<S>,
{
    fn from(secret: Secret<S, I>) -> Self {
        secret.inner_secret.into()
    }
}

impl<S, I> NotU8 for Secret<S, I>
where
    S: NotU8,
    I: Strategy<S>,
{
}

impl<S, I> TryGetableArray for Secret<S, I>
where
    Vec<S>: TryGetable,
    I: Strategy<S>,
{
    fn try_get_by<Ix: ColIdx>(res: &QueryResult, index: Ix) -> Result<Vec<Self>, TryGetError> {
        <Vec<S> as TryGetable>::try_get_by(res, index)
            .map(|vec| vec.into_iter().map(Self::new).collect())
    }
}

impl<S, I> ValueType for StrongSecret<S, I>
where
    S: ZeroizableSecret + ValueType,
    I: Strategy<S>,
{
    fn try_from(v: Value) -> Result<Self, ValueTypeErr> {
        S::try_from(v).map(Self::new)
    }

    fn type_name() -> String {
        format!("StrongSecret<{}>", S::type_name())
    }

    fn array_type() -> ArrayType {
        S::array_type()
    }

    fn column_type() -> ColumnType {
        S::column_type()
    }

    fn is_option() -> bool {
        S::is_option()
    }

    fn enum_type_name() -> Option<&'static str> {
        S::enum_type_name()
    }
}

impl<S, I> TryGetable for StrongSecret<S, I>
where
    S: ZeroizableSecret + TryGetable,
    I: Strategy<S>,
{
    fn try_get_by<Ix: ColIdx>(res: &QueryResult, index: Ix) -> Result<Self, TryGetError> {
        S::try_get_by(res, index).map(Self::new)
    }
}

impl<S, I> Nullable for StrongSecret<S, I>
where
    S: ZeroizableSecret + Nullable,
    I: Strategy<S>,
{
    fn null() -> Value {
        S::null()
    }
}

impl<S, I> From<StrongSecret<S, I>> for Value
where
    S: ZeroizableSecret + Into<Self> + Clone,
    I: Strategy<S>,
{
    fn from(secret: StrongSecret<S, I>) -> Self {
        secret.inner_secret.clone().into()
    }
}

impl<S, I> NotU8 for StrongSecret<S, I>
where
    S: ZeroizableSecret + NotU8,
    I: Strategy<S>,
{
}

impl<S, I> TryGetableArray for StrongSecret<S, I>
where
    S: ZeroizableSecret,
    Vec<S>: TryGetable,
    I: Strategy<S>,
{
    fn try_get_by<Ix: ColIdx>(res: &QueryResult, index: Ix) -> Result<Vec<Self>, TryGetError> {
        <Vec<S> as TryGetable>::try_get_by(res, index)
            .map(|vec| vec.into_iter().map(Self::new).collect())
    }
}
