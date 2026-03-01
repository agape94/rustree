# rustree
A helper tool for cloning bare repositories and managing clean, isolated Git worktrees with minimal friction.

`rustree` streamlines a powerful Git workflow: one bare repository + multiple lightweight working directories, each tied to a branch.

## Why rustree?
When working on multiple branches simultaneously (features, tickets, hotfixes), the typical workflow looks like:
- Stash changes
- Checkout another branch
- Pull
- Repeat

Or you clone the same repository multiple times.
Both approaches are messy.

Git worktrees solve this problem — but managing them manually is tedious.
`rustree` automates this by:
- Cloning a repository as a bare repo
- Creating a clean structure for multiple worktrees
- Automatically handling SSH keys
- Managing branch creation and upstream tracking
Result: clean separation between branches, no stashing, no duplicate clones.

## What Your Repository Will Look Like
After cloning and creating worktrees, your project structure will look like this:
```
project
├── .bare
│   └...
├── .git
├── main
│   ├── .git
│   ├── .gitignore
│   ├── LICENSE
│   └── README.md
└── TICKET-123
    ├── .git
    ├── .gitignore
    ├── LICENSE
    └── README.md
```
Explanation
- `.bare/` → the actual Git repository (bare)
- `.git` → points to `.bare`
- `main/` → worktree for `main` branch
- `TICKET-123/` → worktree for a feature branch

Each directory is an isolated working copy, backed by the same Git object database.

## What Problems Does This Solve?
### 1. No More Stashing
Work on multiple branches simultaneously without:
- `git stash`
- Dirty working trees
- Context switching headaches

### 2. No Duplicate Clones
Instead of:
```
project-main/
project-feature/
project-hotfix/
```
You get:
```
project/
├── .bare/
├── main/
├── feature/
└── hotfix/
```
One object database. Multiple working directories.

### 3. Clean Feature Isolation
Each branch:
- Has its own folder
- Has its own build artifacts
- Has its own environment

Perfect for:
- Large builds
- Docker projects
- Python virtual environments
- Monorepos

### 4. Faster CI-like Local Testing
You can:
- Keep `main/` clean. You always have a working version of your code ready to go
- Test features in isolation
- Compare branches side by side

## Installation
`cargo install --path .`

Or build manually:

`cargo build --release`

## Usage
```
Usage: rustree <COMMAND>
Helper tool that helps you cloning bare repositories and managing git worktrees for a bare repository

Usage: rustree <COMMAND>

Commands:
  clone     Clone a repository and create worktree for default branch
  worktree  Manage worktrees
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
### Clone Command

Clone a repository as a bare repository and initialize the worktree structure.

```
rustree clone [OPTIONS] <REPOSITORY_URL>
```
#### Arguments
- `<REPOSITORY_URL>` — SSH or HTTPS repository URL
#### Options
```
-s, --ssh-key <SSH_KEY>
    Path to the SSH key.
    If not specified, all keys under '$HOME/.ssh' will be tried
```
#### Example

`rustree clone git@github.com:user/project.git`

Or with explicit key:

`rustree clone -s ~/.ssh/id_ed25519 git@github.com:user/project.git`

### Worktree Command
Create and manage additional worktrees.

```
rustree worktree [OPTIONS] <DIRECTORY> <BRANCH>
```
#### Arguments
- `<DIRECTORY>` — Path where the worktree should be created
- `<BRANCH>` — Branch name for the new worktree

If the branch does not exist:
- It will be created
- Based on `--base-branch` or the repository’s default branch

#### Options
```
--base-branch <BASE_BRANCH>
    Base branch for the new branch.
    If empty, the default branch of the repository will be used

-s, --ssh-key <SSH_KEY>
    Path to the SSH key.
    If not specified, all keys under '$HOME/.ssh' will be tried
```
#### Examples
- Create a new feature branch:

    `rustree worktree TICKET-123 TICKET-123`
- Create from specific base branch:

    `rustree worktree feature-x feature-x --base-branch develop`

### SSH Key Handling
If no `--ssh-key` is provided:
- All keys in `$HOME/.ssh` are tried automatically
- The first successful key is used

This makes working across multiple Git providers seamless.

## Typical Workflow
```
# Clone once
rustree clone git@github.com:user/project.git

# Work on ticket
rustree worktree TICKET-123 TICKET-123

cd TICKET-123
# hack hack hack

# Work on another ticket simultaneously
rustree worktree TICKET-456 TICKET-456
```
No branch switching. No stashing. No chaos.

## Future Plans
- Better remote detection
- Smarter branch auto-tracking
- Worktree listing & removal
- Configurable layout
- Possibly renaming the tool

## When Should You Use This?
You’ll love `rustree` if:
- You work with multiple feature branches daily
- You manage ticket-based workflows (Jira, Linear, etc.)
- You dislike stashing
- You want deterministic branch isolation
- You work in large repositories

## Philosophy
- Keep Git powerful.
- Remove friction.
- Make worktrees effortless.

## Contributing

`rustree` is an open source project and contributions are welcome and appreciated!  

Whether you want to:

- Fix bugs  
- Add new features (e.g., better remote handling, worktree management)  
- Improve documentation  

…your help is appreciated.  

### How to Contribute

1. Fork the repository  
2. Create a feature branch: `git checkout -b feature/my-feature`  
3. Make your changes and commit: `git commit -am 'Add my feature'`  
4. Push to your fork: `git push origin feature/my-feature`  
5. Open a Pull Request  

Please make sure your code follows the existing style and is well-tested.  
All contributions are licensed under the same terms as `rustree`.
