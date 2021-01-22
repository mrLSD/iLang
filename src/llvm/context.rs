#[derive(Debug, Clone)]
pub struct Context {
    val: u64,
}

impl Context {
    /// Init Contect
    pub fn new() -> Self {
        Self { val: 0 }
    }

    /// Get current context as string
    pub fn get(&self) -> String {
        format!("{}", self.val)
    }

    /// Increment context value
    pub fn inc(&mut self) -> Self {
        self.val += 1;
        Context { val: self.val }
    }

    /// Set context value
    pub fn set(&mut self, val: u64) {
        self.val = val;
    }

    /// Get current context value
    pub fn val(&mut self) -> u64 {
        self.val
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
