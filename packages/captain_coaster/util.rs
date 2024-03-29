use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Deserializer};
use tokio::io::{AsyncWrite, AsyncWriteExt};

use std::error::Error;
use std::fmt::{self, Display, Write};
use std::io;
use std::marker::{PhantomData, Unpin};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// HTTP body response stream.
pub struct ResponseStream<T, E>(pub Box<dyn Stream<Item = Result<T, E>> + Unpin>);

/// **NOTE:** This is just a stub. It panics on deserialization.
impl<'de, T, E> Deserialize<'de> for ResponseStream<T, E> {
	fn deserialize<D>(_: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		unimplemented!("Response stream is not supposed to be deserialized");
	}
}

impl<T, E> ResponseStream<T, E>
where
	T: AsRef<[u8]>,
	E: Into<Box<dyn Error + Send + Sync + 'static>>,
{
	/// Copy this stream to anything that implements `AsyncWrite`.
	pub async fn to_writer<W>(mut self, writer: &mut W) -> io::Result<()>
	where
		W: AsyncWrite + Unpin + ?Sized,
	{
		while let Some(r) = self.0.next().await {
			let chunk = r.map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
			writer.write_all(chunk.as_ref()).await?;
		}

		Ok(())
	}
}

/// Marker trait for delimiting. We represent each type of delimiting
/// with an unit struct and implement this
pub trait Delimiting {
	const DELIMITER: char;
}

/// Marker trait for whether the delimiting unit struct can be used by
/// iterators. This is not implemented by `multi` - Multiple instances are
/// allowed only in form data and query, and we need something for parsing
/// stuff from CLI. At the same time, we also cannot allow serializing this
/// container in the same way as others.
pub trait Allowed {}

macro_rules! impl_delim {
	($ty:ident => $delim:expr) => {
		#[derive(Debug, Clone)]
		pub struct $ty;

		impl Delimiting for $ty {
			const DELIMITER: char = $delim;
		}
	};
}

impl_delim!(Csv => ',');
impl Allowed for Csv {}

impl_delim!(Ssv => ' ');
impl Allowed for Ssv {}

impl_delim!(Tsv => '\t');
impl Allowed for Tsv {}

impl_delim!(Pipes => '|');
impl Allowed for Pipes {}

// NOTE: We use ampersand only for convenience.
impl_delim!(Multi => '&');

/// Wrapper over a vector which also holds a marker type for delimiting.
#[derive(Debug, Clone)]
pub struct Delimited<T, D>(Vec<T>, PhantomData<D>);

impl<T, D> From<Vec<T>> for Delimited<T, D> {
	fn from(v: Vec<T>) -> Self {
		Delimited(v, PhantomData)
	}
}

impl<T, D> Deref for Delimited<T, D> {
	type Target = Vec<T>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T, D> DerefMut for Delimited<T, D> {
	fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
		&mut self.0
	}
}

impl<T: FromStr, D: Delimiting> FromStr for Delimited<T, D> {
	type Err = <T as FromStr>::Err;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let vec: Result<Vec<_>, _> = s.split(D::DELIMITER).map(|s| s.parse::<T>()).collect();
		Ok(Delimited(vec?, PhantomData))
	}
}

impl<T: Display, D: Delimiting + Allowed> Display for Delimited<T, D> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for (i, v) in self.0.iter().enumerate() {
			if i > 0 {
				f.write_char(D::DELIMITER)?;
			}

			v.fmt(f)?;
		}

		Ok(())
	}
}
