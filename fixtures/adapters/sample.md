---
title: Quarterly notes
subtitle: Engineering update
author: Web Team
date: 2026-10-01
---

# Summary

The **release** went out on time. See the [changelog](https://example.com/changelog) for details; the `deploy` step now takes two minutes.

## Highlights

- Faster builds
- Fewer flaky tests
  - Retries removed
- Cleaner logs

## Next steps

1. Migrate the pipeline
2. Retire the old runner

> Measure twice, cut once.

| Area | Status | Owner |
| --- | :---: | ---: |
| Build | done | Ana |
| Tests | in progress | Ben |

```
cargo build --release
cargo test
```

![Release graph](chart.svg)

---

### Closing

That is all for this quarter.
