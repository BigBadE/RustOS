pub struct FixedSizeBlock {
    pub next: Option<&'static mut FixedSizeBlock>
}