use crate::iter::FromIterator;

#[stable(feature = "unit_from_iter", since = "1.23.0")]
impl FromIterator<()> for () {
    fn from_iter<I: IntoIterator<Item=()>>(iter: I) -> Self { loop { } }
}
