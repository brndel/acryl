use core::fmt::Debug;
use core::hash::Hash;

pub type DefaultCoords = ();

pub trait Coords:
    Clone + Copy + Debug + Default + PartialEq + Eq + PartialOrd + Ord + Hash
{
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PdfCoords;

impl Coords for PdfCoords {}

pub type AcrylCoords = ();

impl Coords for AcrylCoords {}
