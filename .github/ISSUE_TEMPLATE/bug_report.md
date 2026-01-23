---
name: Bug Report
about: Report a bug or unexpected behavior
title: '[BUG] '
labels: bug
assignees: ''
---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Create component '...'
2. Render report '...'
3. See error

**Code Sample**
```rust
// Minimal code to reproduce the issue
use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    // Your code here
    Ok(())
}
```

**Expected behavior**
A clear description of what you expected to happen.

**Actual behavior**
What actually happened. Include error messages if applicable.

**Environment:**
- RenderReport version: [e.g. 0.1.0-alpha.1]
- Rust version: [run `rustc --version`]
- Operating System: [e.g. Ubuntu 22.04, macOS 14, Windows 11]

**Additional context**
Add any other context about the problem here. Screenshots, generated PDFs, or Typst compilation errors are helpful.
