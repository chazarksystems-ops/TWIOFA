# TWIOFA Agent Report Template

Use this after every pass.

## Pass

```text
Slice ID:
Task title:
Branch:
Task type:
```

## Files Read

```text
- file
- file
```

## Files Changed

```text
- file — summary
- file — summary
```

## Files Intentionally Not Touched

```text
- file or folder — reason
```

## Implementation Summary

```text
What changed?
What did not change?
Why was this scope correct?
```

## User-Visible Changes

```text
What can Chaz now do, see, or test?
```

For docs-only tasks, state:

```text
No gameplay/user-facing runtime behavior changed.
```

## Design Decisions Preserved

```text
- decision preserved
- decision preserved
```

## Drift Prevented

```text
- drift risk avoided
- drift risk avoided
```

## Validation Performed

```text
Commands run:
Results:
Manual checks:
```

If validation was not run, explain why.

## Known Issues / Remaining Gaps

```text
- gap
- gap
```

## Next Recommended Pass

```text
Recommended next slice:
Reason:
Required approval:
```

## Success Claim

Use one of these:

```text
PASS — all required checks passed.
PARTIAL — docs/files created, but validation or audit remains.
BLOCKED — stopped due to missing file, contradiction, or failed validation.
```

Do not use `PASS` unless the pass-specific acceptance gates were actually checked.
