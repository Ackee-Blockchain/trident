# Contributing

Welcome and thank you for considering contributing to Trident open source!

Reading and following these guidelines will help us make the contribution process easy and effective for everyone involved. It also communicates that you agree to respect the time of developers managing and developing the Trident project. In return, we will reciprocate that respect by addressing your issue, assessing changes and helping you finalize your PRs.

## Table of Contents

- [Code of Conduct](./CODE_OF_CONDUCT.md)
- [Getting Started](#getting-started)
  - [Issues](#issues)
  - [PRs](#prs)
- [Getting Help](#getting-help)

## Code of Conduct

We take our open source and community seriously and hold ourselves and other contributors to high communication standards. By participating and contributing to this project, you agree to uphold our [Code of Conduct](./CODE_OF_CONDUCT.md).

## Getting Started

Contributors are made to this repository via issues and pull requests (PRs). A few general guidelines that cover both:

- Search for existing issues and pull requests before creating your own
- We work hard to ensure issues are handled promptly but depending on the impact, it could take a while to investigate the root cause. A friendly ping in the comment thread to the submitter or a contributor can help draw attention if your issue is blocked.
- If you have never contributed before, see the [First contribution guideline](https://github.com/firstcontributions/first-contributions) and the [Open source guide](https://opensource.guide/how-to-contribute/) for resources and tips on how to get started

### Issues

Issues should be used to report problems with the library, request a new feature, or discuss potential changes before a PR is created.

If you find an issue that addresses your problem, please add your reproduction information to the existing issue rather than creating a new one. Adding a [reaction](https://github.blog/2016-03-10-add-reactions-to-pull-requests-issues-and-comments/) can also help indicate to our maintainers that a particular problem is affecting more than just the reporter.

So, to wrap it up:

- Search for an existing issue before you create your own
- Create an issue before you create a new PR
- Describe your problems or needs as good as you can
- If you are reporting a bug, do not forget to add steps to reproduce, versions (Rust and Trident), a full error message, or describe the bad behaviour that happened
- If possible, do your own investigation and describe how to fix the problem / what is the problem / how to implement it or improve it

### PRs

PRs to our project are always welcome! It can be a quick way to get your fix or improvement slated for the next release. In general, PRs should:

- Only fix or add the functionality in question or address wide-spread whitespace / style issues, not both
- Add unit or integration tests for fixed or changed functionality (if a test suite already exists)
- Address a single concern in the least number of changed lines as possible
- Include documentation in the repository or on our docs site
- Rebase instead of merge

For changes that address core functionality or would require breaking changes (e.g. a major release), it is best to open an issue to discuss your proposal first. It would be really nice to do it because it can save time creating and reviewing changes.

In general, we follow the [Fork-and-pull Git workflow](https://github.com/susam/gitpr):

1. Fork the repository to your own GitHub account
2. Clone the project to your machine
3. Create a branch locally with a succinct but descriptive name
4. Commit changes to the branch
5. Following any formatting and testing guidelines specific for this repository
6. Push changes to your fork
7. Open a PR in our repository and add reviewers

So, to wrap it up:

- Follow our naming and commit conventions
  - Use the emojis from [gitmoji](https://gitmoji.dev/) at the beginning of the commit message, [see our commit messages](https://github.com/Ackee-Blockchain/trident/commits/master)
  - Add link to the issue at the end of the commit message
  - For example: `✨ split test command into build and test - #1, #2`
- Do not modify files that are not related to the issue you are working on
  - If you want to improve formatting, methods, and the other files that are not related to the issue, please create a new issue and do the changes in a new branch / PR
- Do not forget to add maintainers as reviewers (at least one of them) to your PRs
  - [@tribuste](https://github.com/tribuste)
  - [@vmarcin](https://github.com/vmarcin)
  - [@stefanprokopdev](https://github.com/stefanprokopdev)

## Getting Help

Join us in the [Ackee Blockchain Discord](https://discord.gg/x7qXXnGCsa) and post your question there.
