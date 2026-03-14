pub fn iter_chars<'a>(slices: &'a [&'a str]) -> impl Iterator<Item = char> + 'a {
    slices.iter().flat_map(|s| s.chars())
}

pub trait SlicesOps<'a>: AsRef<[&'a str]> {
    fn slices(&self) -> &[&'a str] {
        self.as_ref()
    }

    fn equals_slices(
        &self,
        other: &[&'a str],
    ) -> bool {
        iter_chars(self.slices()).eq(iter_chars(other))
    }

    fn equals_str(
        &self,
        target: &str,
    ) -> bool {
        iter_chars(self.slices()).eq(target.chars())
    }

    fn starts_with_str(
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

impl<'a, T> SlicesOps<'a> for T where T: AsRef<[&'a str]> {}
