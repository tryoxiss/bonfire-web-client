# bonfire
(TEMPORARY NAME) repo for decentralized discord-like federated chat! following all common conventions like markdown and naming schemes. Written in rust! 

Versioning is `major:minor:patch::build_status`. Breaking changes may occur, but should be avoided as much as possible. 

OLD: 
```
Bonfire Software Stack (WIP): 

Server-Side software: 
.------------------------------------------------------------------.
| bonfire-server                                                   |
'------------------------------------------------------------------'
.--------------. .----------. .------. .-------. .-----------------.
| Bonfire Spec | | maria db | | Rust | | HTTPS | | Web Client      | 
'--------------' '----------' '------' '-------' '-----------------'

Client-Side software: 
.------------------------------------------------------------------.
| (offical) bonfire-client (will be native eventually but not now) |
'------------------------------------------------------------------'
.--------------. .--------------------------------. .--------------.
| Bonfire Spec | | Yew                            | | Electron     |
|              | '--------------------------------' '--------------'
|              | .------. .------. .------. .-----. .--------------.
|              | | Rust | | HTML | | WASM | | CSS | | chromium     |
'--------------' '------' '------' '------' '-----' '--------------'

Bonfire Standards Stack: 

Specification: 
.-------------------------------------------------------------------.
| Bonfire Specification                                             |
'-------------------------------------------------------------------'
.----------------------. .--------------------. .----------. .------.
| Federation Standards | | Username Standards | | Markdown | | SQL  |
'----------------------' '--------------------' '----------' '------'

Fragments Mentioned: 
.-------------------------------------------------------------------.
| Federation Standards                                              |
'-------------------------------------------------------------------'
.------------------------------------------. 
| diaspora* federation protocol            | 
'------------------------------------------' 

.-------------------------------------------------------------------.
| Username Standards (@username#0001+subname@instance.tld           |
'-------------------------------------------------------------------'
.-----------------------------. .------------. .--------------------.
| @user@instance.tld (ActPub) | | sub-emails | | @username#0001     |
'-----------------------------' '------------' '--------------------'
```
