---
name: slang-pr-reviews
description: Address reviewer feedback on an open pull request — fetch unresolved review threads, reply within each thread, and commit requested changes. Use whenever responding to PR review comments, addressing review feedback, or working through reviewer threads on the current branch's PR.
---

# Respond to PR Reviews

How to address reviewer feedback on the **current branch's open PR**.

## Work on top of the latest remote

**Only in [claude.ai/code](https://claude.ai/code) environments, and only if you have permission to
pull:** before addressing feedback, fetch and check out the remote branch in case the user pushed to
it, so you always build on the most recent changes.

## Rules

- **Never resolve comment threads.** Leave that to the user.
- **Only act on unresolved threads.** Ignore threads that are already resolved.
- **Read all unresolved threads first**, before acting on any of them, in case they
  reference each other or conflict with one another.
- **Never use PR-level comments.** Only interact through review comment threads.
- **Reply within the thread.** Respond to each thread as a reply on that same thread, not as
  a new top-level comment:
    - **Clear, direct requests**: execute them, then reply describing what was done (cite the commit).
    - **Questions**: reply to the thread directly with the answer — no code change.
    - **Ambiguous requests**: reply to the thread directly to ask for clarification — no code change.

## Notes

- Respond to the **latest** comment in each thread; earlier comments are context.
- Keep replies short and concrete ("Renamed to `parseConfig` and updated callers." — not
  "Great catch! I've gone ahead and...").
