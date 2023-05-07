# Changelog

All notablechances to this project will be documented here. The format is based on, but not exactly, that of [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). 

This project, as mentioned in other documents, does not follow Semver exactly, although it is simillar.

Changes that get added here should: 
- **Affect someone in a meaningful way**. Its great that you removed a newline to fit our code style, but that just isn't worrthy of a changelog mention.
- **It's okay if its not user-facing**. If you spent an hour or so cleaning up the code then thats worth a mention in the changelog defintely, as it notes that we care about our codes health and style enough to put active effort into maintaining it.
- **Refrence the issue ID**: We intend to move from github to a self-hosted solution if this project grows, so all mentioned issues should be `GH-NUM`. 

## Examples
(All headings will be reduced by one level for proper changelog entries)

The tags are: 
- **LTS**: Long Term Support
- **EOL**: End of Life (No more support)
- **YANKED**: This should not be used in any production setting.

### [0.1.0] 12 December 2020 (TAGS, LTS, EOL)

**Important**
- Depreciations, security

**Added**

**Removed**

**Bug Fixes**

## [0.1.0] Unrelased
This is our baseline so not much here. We wanted to get this much finished before we made the repo public. 

**Important**
- The repository is now public

**Added**
- User Inerface
- Tooltips
- Commands Header document
    - `@slash_command` wrapper
        - Ensures enough arguments are present
        - Identifies and handles basic flags
        - Handles client object creation
        - 
    - Command Tools class