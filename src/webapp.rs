
// use yew::prelude::*;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub author: String,

    #[prop_or_default]
    pub content: String,

    #[prop_or_default]
    pub icon: String,

    #[prop_or_default]
    pub header_name: String,

    #[prop_or_default]
    pub title: String,
}

// Then somewhere else you can use the component inside `html!`
// #[function_component]
// fn HelloWorld() -> Html {
//     html! { <p>{ "Hello world" }</p> }
// }

#[function_component]
fn MessageRoot(props: &Props) -> Html {
    html! { 
        <message>
            <a class="pfp"><img class="pfp" src={ props.icon.clone() }/></a>
            <fill>
                <author><a>{ props.author.clone() }</a></author>
                <content>
                { props.content.clone() }
                </content>
            </fill>
        </message> 
    }
}

#[function_component]
fn Message(props: &Props) -> Html {
    html! { 
        <message>
            <fill>
                <content>
                { props.content.clone() }
                </content>
            </fill>
        </message> 
    }
}

#[function_component]
fn CommandResponse(props: &Props) -> Html {
    html! { 
        <message class="bot-message">
            <a class="pfp"><img class="pfp" src={ props.icon.clone() }/></a>
            <fill>
                <author><a>{ props.author.clone() }</a><span class="bot-tag">{ "BOT" }</span></author>
                <content>
                { props.content.clone() }

                    <p class="text-small"><span>{"Only you can see this messgae. It will disapear when your client is restarted or the tab is closed. "}</span><a>{"Dismiss it now."}</a></p>
                </content>
            </fill>
        </message> 
    }
}

#[function_component]
fn DrawHeader(props: &Props) -> Html {
    html! { 
    <header>
        <p>{"@khaim0919"}</p>
    </header>
    }
}

#[function_component]
fn MessageButton(props: &Props) -> Html {
    html! { 
    <button>
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M240 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H176V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H384c17.7 0 32-14.3 32-32s-14.3-32-32-32H240V80z"/></svg>
    </button>
    }
}

#[function_component]
fn GroupIcon(props: &Props) -> Html {
    html! { 
    <a href="#" class="active"><img src="https://link.storjshare.io/raw/jvxikkhiqnksyeatwcn3iigoa3ta/techlgbt/accounts/avatars/109/504/275/977/175/789/original/a3e6266b885c9b43.png" alt=""/></a>
    }
}



#[function_component]
fn InfoChannel(props: &Props) -> Html {
    html! { 
    <li>
        <span><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM216 336h24V272H216c-13.3 0-24-10.7-24-24s10.7-24 24-24h48c13.3 0 24 10.7 24 24v88h8c13.3 0 24 10.7 24 24s-10.7 24-24 24H216c-13.3 0-24-10.7-24-24s10.7-24 24-24zm40-208a32 32 0 1 1 0 64 32 32 0 1 1 0-64z"/></svg></span>
        <text>{"rules"}</text>
    </li>
    }
}

#[function_component]
fn TextChannel(props: &Props) -> Html {
    html! { 
    <li>
        <span><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M181.3 32.4c17.4 2.9 29.2 19.4 26.3 36.8L197.8 128h95.1l11.5-69.3c2.9-17.4 19.4-29.2 36.8-26.3s29.2 19.4 26.3 36.8L357.8 128H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H347.1L325.8 320H384c17.7 0 32 14.3 32 32s-14.3 32-32 32H315.1l-11.5 69.3c-2.9 17.4-19.4 29.2-36.8 26.3s-29.2-19.4-26.3-36.8l9.8-58.7H155.1l-11.5 69.3c-2.9 17.4-19.4 29.2-36.8 26.3s-29.2-19.4-26.3-36.8L90.2 384H32c-17.7 0-32-14.3-32-32s14.3-32 32-32h68.9l21.3-128H64c-17.7 0-32-14.3-32-32s14.3-32 32-32h68.9l11.5-69.3c2.9-17.4 19.4-29.2 36.8-26.3zM187.1 192L165.8 320h95.1l21.3-128H187.1z"/></svg></span>
        <text>{"general"}</text>
    </li>
    }
}

#[function_component]
fn VoiceChannel(props: &Props) -> Html {
    html! { 
    <li>
        <span><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M176 0C123 0 80 43 80 96V256c0 53 43 96 96 96s96-43 96-96V96c0-53-43-96-96-96zM48 216c0-13.3-10.7-24-24-24s-24 10.7-24 24v40c0 89.1 66.2 162.7 152 174.4V464H104c-13.3 0-24 10.7-24 24s10.7 24 24 24h72 72c13.3 0 24-10.7 24-24s-10.7-24-24-24H200V430.4c85.8-11.7 152-85.3 152-174.4V216c0-13.3-10.7-24-24-24s-24 10.7-24 24v40c0 70.7-57.3 128-128 128s-128-57.3-128-128V216z"/></svg></span>
        <text>{"girlies-call"}</text>
    </li>
    }
}



#[function_component]
fn App() -> Html {
    html! { 
    <app>
        <div class="error-message">
            <p>{ "A fatal error occured" }</p>
            <h1>{ "Please use a larger screen" }</h1>
            <p>{ "Our web app does not work on smaller screen sizes, if you would like to use it on mobile. We have an app for iOS and Android." }</p>

            <br/>

            <p><a href="#">{"iOS App Store"}</a> {"|"} <a href="#">{"Google Play"}</a></p>
        </div>

        <nav role="groups">

            <a href="#"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 576" style="margin-top: 10px; margin-left: 10px;">/*<!--! Font Awesome Pro 6.3.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2023 Fonticons, Inc. -->*/
            <path d="M575.8 255.5c0 18-15 32.1-32 32.1h-32l.7 160.2c0 2.7-.2 5.4-.5 8.1V472c0 22.1-17.9 40-40 40H456c-1.1 0-2.2 0-3.3-.1c-1.4 .1-2.8 .1-4.2 .1H416 392c-22.1 0-40-17.9-40-40V448 384c0-17.7-14.3-32-32-32H256c-17.7 0-32 14.3-32 32v64 24c0 22.1-17.9 40-40 40H160 128.1c-1.5 0-3-.1-4.5-.2c-1.2 .1-2.4 .2-3.6 .2H104c-22.1 0-40-17.9-40-40V360c0-.9 0-1.9 .1-2.8V287.6H32c-18 0-32-14-32-32.1c0-9 3-17 10-24L266.4 8c7-7 15-8 22-8s15 2 21 7L564.8 231.5c8 7 12 15 11 24z"/></svg></a>

            // dm icons go here

            <hr />

            // group icons go here

            <GroupIcon />
            <GroupIcon />
            <GroupIcon />

            <a href="#" style="background: transparent;"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" style="margin-left: 12px; margin-top: 9px; fill: var(--ctp-mocha-subtext1)"><path d="M240 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H176V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H384c17.7 0 32-14.3 32-32s-14.3-32-32-32H240V80z"/></svg></a>
        </nav>

        <nav role="channels">

            <ul>
                <InfoChannel />
                <TextChannel />
                <TextChannel />
                <VoiceChannel />
            </ul>

            <p><span>{ "All icons are from "}</span><a href="https://fontawesome.com/search?o=r&m=free">{" Font Awesome Free"}</a></p>
        </nav>

        <main>
            <DrawHeader />

            <div class="messages">

                <MessageRoot content="text" author="khaim" icon="https://link.storjshare.io/raw/jvxikkhiqnksyeatwcn3iigoa3ta/techlgbt/accounts/avatars/109/504/275/977/175/789/original/a3e6266b885c9b43.png" />
                <Message content="text" />
                <Message content="text" />
                <Message content="text" />
                <Message content="text" />
                <MessageRoot content="text" author="khaim" icon="https://link.storjshare.io/raw/jvxikkhiqnksyeatwcn3iigoa3ta/techlgbt/accounts/avatars/109/504/275/977/175/789/original/a3e6266b885c9b43.png" />
                <Message content="text" />
                <MessageRoot content="text" author="khaim" icon="https://link.storjshare.io/raw/jvxikkhiqnksyeatwcn3iigoa3ta/techlgbt/accounts/avatars/109/504/275/977/175/789/original/a3e6266b885c9b43.png" />
                <Message content="text" />
                <MessageRoot content="text" author="khaim" icon="https://link.storjshare.io/raw/jvxikkhiqnksyeatwcn3iigoa3ta/techlgbt/accounts/avatars/109/504/275/977/175/789/original/a3e6266b885c9b43.png" />
                <Message content="text" />
                <MessageRoot content="text" author="khaim" icon="https://link.storjshare.io/raw/jvxikkhiqnksyeatwcn3iigoa3ta/techlgbt/accounts/avatars/109/504/275/977/175/789/original/a3e6266b885c9b43.png" />
                <Message content="text" />
                <MessageRoot content="text" author="khaim" icon="https://link.storjshare.io/raw/jvxikkhiqnksyeatwcn3iigoa3ta/techlgbt/accounts/avatars/109/504/275/977/175/789/original/a3e6266b885c9b43.png" />
                <Message content="text" />
                <MessageRoot content="text" author="khaim" icon="https://link.storjshare.io/raw/jvxikkhiqnksyeatwcn3iigoa3ta/techlgbt/accounts/avatars/109/504/275/977/175/789/original/a3e6266b885c9b43.png" />
                <Message content="text" />
                <MessageRoot content="text" author="khaim" icon="https://link.storjshare.io/raw/jvxikkhiqnksyeatwcn3iigoa3ta/techlgbt/accounts/avatars/109/504/275/977/175/789/original/a3e6266b885c9b43.png" />
                <Message content="text" />
                <MessageRoot content="text" author="khaim" icon="https://link.storjshare.io/raw/jvxikkhiqnksyeatwcn3iigoa3ta/techlgbt/accounts/avatars/109/504/275/977/175/789/original/a3e6266b885c9b43.png" />
                <Message content="text" />
                <MessageRoot content="text" author="khaim" icon="https://link.storjshare.io/raw/jvxikkhiqnksyeatwcn3iigoa3ta/techlgbt/accounts/avatars/109/504/275/977/175/789/original/a3e6266b885c9b43.png" />
                <Message content="text" />
                <MessageRoot content="text" author="khaim" icon="https://link.storjshare.io/raw/jvxikkhiqnksyeatwcn3iigoa3ta/techlgbt/accounts/avatars/109/504/275/977/175/789/original/a3e6266b885c9b43.png" />
                <Message content="text" />
                
                
            </div>
            
            <div class="message-box-area">
                <MessageButton />
                <textarea name="message-box" id="message-box" cols="30" rows="2"></textarea>
            </div>
        </main>

        <div role="members-list">
        </div>
    </app>
    }
}

pub fn render_app() {
    yew::Renderer::<App>::new().render();
}