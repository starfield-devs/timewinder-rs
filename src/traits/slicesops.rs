pub fn iter_chars<'a>(slices: &'a [&'a str]) -> impl Iterator<Item = char> + 'a {
    slices.iter().flat_map(|s| s.chars())
}

pub trait SlicesOps<'a>: AsRef<[&'a str]> {
    fn slices(&self) -> &[&'a str] {
        self.as_ref()
    }

    fn equals<T>(
        &self,
        other: T,
    ) -> bool
    where
        T: EqualsTarget<'a>,
    {
        other.eq_impl(self.slices())
    }

    fn starts_with(
        &self,
        prefix: &str,
    ) -> bool {
        let mut prefix_chars = prefix.chars();
        for c in iter_chars(self.slices()) {
            if let Some(pc) = prefix_chars.next() {
                if pc != c {
                    return false;
                }
            } else {
                return true;
            }
        }
        prefix_chars.next().is_none()
    }
}

pub trait EqualsTarget<'a> {
    fn eq_impl(
        &self,
        slices: &[&'a str],
    ) -> bool;
}

/// Enables `T.equals("DEMO")`.
impl<'a> EqualsTarget<'a> for &'a str {
    fn eq_impl(
        &self,
        slices: &[&'a str],
    ) -> bool {
        iter_chars(slices).eq(self.chars())
    }
}

/// Enables `T.equals(T.slices())`.
impl<'a> EqualsTarget<'a> for &'a [&'a str] {
    fn eq_impl(
        &self,
        slices: &[&'a str],
    ) -> bool {
        iter_chars(slices).eq(iter_chars(self))
    }
}
