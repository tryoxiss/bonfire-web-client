# bonfire
(TEMPORARY NAME) repo for decentralized discord-like federated chat! following all common conventions like markdown and naming schemes. Written in pure rust! 

(Please note: Badges are updated manually)

[![Code Style: Beautiful](repo-style/badges/code-style.png)](CODESTYLE.md) ![Build: 0.1.0](repo-style/badges/version.png) ![Stability: Great](repo-style/badges/stability.png) ![Code Health: Good](repo-style/badges/code-health.png) ![Security: Terrible](repo-style/badges/security.png) ![Contibuters: 2](repo-style/badges/contributers.png) ![Documentation: Poor](repo-style/badges/documentation.png)

For instructions on how to set up bonfire, please look at our [Setup Guide](SETUP.md)

Installer command (I think):
`curl https://github.com/tryoxiss/bonfire-server/blob/new-working-branch/INSTALL.sh -o bonfire_installer.sh; ./bonfire_installer.sh`

## Primary Goals

- Highly resiliant to cortprate control/takeovers
    - Easy to self-host
    - ALWAYS open.
- People are not products
- Just make the best thing we can. 
    - Actually good internet chat
    - And groupware
- And forward looking
    - Extensible
    - Future proof
    - Etc

## Why not XMPP?

XMPP is great: so great in fact, we want to make Bonfire interoperable with XMPP to the extent possible. However, it does not support the hubs and channels format that we want to use. So, unfortuntely, we need to make our own spec. We will still use XMPP for direct messages, or at least make them interoperable with it, but we can't use it for the bulk of Bonfire.

## Community Team

Tryo - admin, project manager, coordinator, art, etc    
Khaim - lead programmer

## Versioning

For versioning, we use our system, of course. However we will release it anually in the same way rust does editions. The spec is open for revisions every year, and releasing a years edition means it is fully compliant with that version of the specification. 

This is seperate from the software version number, which represents its developement.

Every year of developement after 1.0 we will make an LTS release for every edition with spec changes, which will be maintained for the following timespans. LTS releases are feature frozen in time. We currently provide no gaurnteed support window for LTS releases, but we can say that they will be supported at least until the next LTS release has been out for 6 months or so.

Internal versions are clean:version, `Major:Minor:Patch::version`. For example: `1:3:1::beta`. They are incremeneted as follows. Everything after the doubble colon can be ommitted if it is not the defult. 

Stability ratings: 
- LTS `lts` - very stable, for long term support releases.
- Release `release` (Defult) - most commonly run versions. 
- Release Candiate `release_option` - for instances that want to try out new features early.
- Stable Beta `s_beta` - a beta that is as stable as a release candidate, but dosent have all the wanted changes.
- Beta `beta` - unstable, dev only (i.e., try not to run this on a production server)
- Alpha `alpha` - HIGHLY unstable. Often not used, often its just beta stability.

Our numbering system is:
- The first digit indicates fundemental change. Not necesarly breaking: but it is a very diffrent peice of software. This should be avoided.
- Minor is incremented with feature sets (generally, pushes to the main branch). After 1.0, we only intend to do this two or three times a year at most.
- Patch is incremented any time branches are merged OR a feature is added. 

### Naming Format

Releases are named 
bonfire-server-VERSION-os.zip (containing the release files including a readme inside)
the version is 1.3.9 for example, NOT our internal colon seperated system for windows compatability. 

For LTS releases we use `bonfire-EDITION.LTS`, for example `bonfire-2023.LTS`, often with a .zip file extension.

## Random info we have nowhere else to put righjt now

We will use ports `:987X`, where `X` can be any number from `0` -- `9`. (At least for now, but they are unclaimed). 

Current name ideas for the protocol are: 
- oimp:// - open instant messaging protocol
- ximp:// - Extensible Instant Messaging Protol
- ocp:// - open chat protocol
- sdm:// - secure distributed messaging