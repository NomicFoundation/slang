# 9.1. Slang Versioning Policy

Slang is not tied to a specific Solidity version, and can be used to analyze multiple Solidity versions with the same API.
At the same time, Slang's own API needs to be consistent and stable as new versions of Solidity are released.

The NPM package uses [Semantic Versioning](https://semver.org/) and [CHANGELOG.md](https://github.com/NomicFoundation/slang/blob/main/CHANGELOG.md)
to communicate such changes in the form of `{MAJOR}.{MINOR}.{PATCH}` versions, to report breaking changes, new features, and bug fixes (respectively).
But it is not not really designed to report two (often orthogonal) kinds of changes: changes in Slang's APIs, and changes in the Solidity language itself.

Therefore, it is important to define and follow a clear versioning policy for Slang, that will make this distinction clear.
Each changelog entry will contain a link to the pull request that introduced the change. The pull request will contain a description
of the change, its impact on our APIs, the Solidity language, and any potential impact on the users.

## Major Version Updates

These are `{MAJOR}.0.0` releases, and they are intended to represent breaking changes in Slang's APIs.
For example, when an existing API is removed, or replaced with a different one, and will often require
manual intervention from the user to update their code.

These releases are only done when absolutely necessary, and are not expected to happen often.
They are often accompanied by detailed announcements, explaining the motivation behind it.

## Minor Version Updates

These are `X.{MINOR}.0` releases, and they are intended to represent one of two things:

1. A new Slang API or feature that is backwards compatible with previous versions. While users should take note of it,
   in case it is useful to them, the existing APIs should continue to work without any changes.
2. Adoption of a new Solidity version, which can often impact the structure of the CST (adding or changing existing nodes),
   or user-authored tree queries. Users should also take note of it, especially if they want to support multiple (or latest) Solidity versions.

Slang intends to always keep up with Solidity language updates, and promptly implement user feedback and feature requests.
As such, these releases will happen semi-regularly, and we advise users to review the changelog when they happen.

## Patch Version Updates

These are `X.X.{PATCH}` releases, and they are intended to represent bug fixes and minor improvements.
While they can impact user code (if the older version had a bug, which is now fixed), they are not expected to break
Slang APIs, and should be adopted as soon as possible, to make sure user code is always performing correctly.
