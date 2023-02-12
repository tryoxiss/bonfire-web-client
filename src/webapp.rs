
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
            <a class="left"><img class="pfp" src={ props.icon.clone() }/></a>
            <fill>
                <author><a>{ props.author.clone() }</a> <time>{ "24/DEC/2022 at 2:21" }</time></author>
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
            <div class="left">
                <time>{"22:22"}</time>
            </div>
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
            <a class="left"><img class="pfp" src={ props.icon.clone() }/></a>
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
    <header class="header-bar">
        <p><span><span class="muted">{"@"}</span><strong>{"khaim0919"}</strong><span class="muted">{"#0001@instance.tld"}</span></span> <span class="topic"> {" Nicknames or Channel Topic"} </span></p>

        <span class="menu-bar-buttons">
        <button class="round-button">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M224 256A128 128 0 1 0 224 0a128 128 0 1 0 0 256zm-45.7 48C79.8 304 0 383.8 0 482.3C0 498.7 13.3 512 29.7 512H418.3c16.4 0 29.7-13.3 29.7-29.7C448 383.8 368.2 304 269.7 304H178.3z"/></svg>
        </button>
        <button class="round-button">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M32 32C32 14.3 46.3 0 64 0H320c17.7 0 32 14.3 32 32s-14.3 32-32 32H290.5l11.4 148.2c36.7 19.9 65.7 53.2 79.5 94.7l1 3c3.3 9.8 1.6 20.5-4.4 28.8s-15.7 13.3-26 13.3H32c-10.3 0-19.9-4.9-26-13.3s-7.7-19.1-4.4-28.8l1-3c13.8-41.5 42.8-74.8 79.5-94.7L93.5 64H64C46.3 64 32 49.7 32 32zM160 384h64v96c0 17.7-14.3 32-32 32s-32-14.3-32-32V384z"/></svg>
        </button>
        <button class="round-button">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M416 208c0 45.9-14.9 88.3-40 122.7L502.6 457.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L330.7 376c-34.4 25.2-76.8 40-122.7 40C93.1 416 0 322.9 0 208S93.1 0 208 0S416 93.1 416 208zM208 352a144 144 0 1 0 0-288 144 144 0 1 0 0 288z"/></svg>
        </button>
        </span>
    </header>
    }
}

#[function_component]
fn MessageButton(props: &Props) -> Html {
    html! { 
    <button class="round-button">
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
    <a class="li">
        <span><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM216 336h24V272H216c-13.3 0-24-10.7-24-24s10.7-24 24-24h48c13.3 0 24 10.7 24 24v88h8c13.3 0 24 10.7 24 24s-10.7 24-24 24H216c-13.3 0-24-10.7-24-24s10.7-24 24-24zm40-208a32 32 0 1 1 0 64 32 32 0 1 1 0-64z"/></svg></span>
        <text>{"rules"}</text>
    </a>
    }
}

#[function_component]
fn TextChannel(props: &Props) -> Html {
    html! { 
    <a class="li">
        <span><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M181.3 32.4c17.4 2.9 29.2 19.4 26.3 36.8L197.8 128h95.1l11.5-69.3c2.9-17.4 19.4-29.2 36.8-26.3s29.2 19.4 26.3 36.8L357.8 128H416c17.7 0 32 14.3 32 32s-14.3 32-32 32H347.1L325.8 320H384c17.7 0 32 14.3 32 32s-14.3 32-32 32H315.1l-11.5 69.3c-2.9 17.4-19.4 29.2-36.8 26.3s-29.2-19.4-26.3-36.8l9.8-58.7H155.1l-11.5 69.3c-2.9 17.4-19.4 29.2-36.8 26.3s-29.2-19.4-26.3-36.8L90.2 384H32c-17.7 0-32-14.3-32-32s14.3-32 32-32h68.9l21.3-128H64c-17.7 0-32-14.3-32-32s14.3-32 32-32h68.9l11.5-69.3c2.9-17.4 19.4-29.2 36.8-26.3zM187.1 192L165.8 320h95.1l21.3-128H187.1z"/></svg></span>
        <text>{"general"}</text>
    </a>
    }
}

#[function_component]
fn VoiceChannel(props: &Props) -> Html {
    html! { 
    <a class="li">
        <span><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M176 0C123 0 80 43 80 96V256c0 53 43 96 96 96s96-43 96-96V96c0-53-43-96-96-96zM48 216c0-13.3-10.7-24-24-24s-24 10.7-24 24v40c0 89.1 66.2 162.7 152 174.4V464H104c-13.3 0-24 10.7-24 24s10.7 24 24 24h72 72c13.3 0 24-10.7 24-24s-10.7-24-24-24H200V430.4c85.8-11.7 152-85.3 152-174.4V216c0-13.3-10.7-24-24-24s-24 10.7-24 24v40c0 70.7-57.3 128-128 128s-128-57.3-128-128V216z"/></svg></span>
        <text>{"girlies-call"}</text>
    </a>
    }
}



#[function_component]
fn App() -> Html {
    html! { 
    <app class="ctp-mocha">
        <div class="error-message">
            <p>{ "A fatal error occured" }</p>
            <h1>{ "Please use a larger screen" }</h1>
            <p style="margin-bottom: 0.75rem;">{ "Our web app does not work on smaller screen sizes, if you would like to use it on mobile: we have an app for iOS and Android." }</p>

            <p>{ " Our exact cutoff is 900 pixels in width and 450 pixels in height." }</p>

            <br/>

            <p><a href="#">{"iOS App Store"}</a> {" | "} <a href="#">{"Google Play"}</a></p>
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

            <a href="#" style="background: transparent;"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" style="margin-left: 12px; margin-top: 9px;"><path d="M240 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H176V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H384c17.7 0 32-14.3 32-32s-14.3-32-32-32H240V80z"/></svg></a>
        </nav>

        <nav role="channels">

            <details>
            <summary>{"General"}</summary>
            <ul>
                <InfoChannel />
                <TextChannel />
                <TextChannel />
                <VoiceChannel />
            </ul>
            </details>

            <p><span>{ "All icons are from "}</span><a href="https://fontawesome.com/search?o=r&m=free">{" Font Awesome Free"}</a></p>
        </nav>

        <DrawHeader />

        <main>

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

                <button class="button-rect button-danger">{"Delete Account"}</button>
                <button class="button-rect button-cta">{"Create Account"}</button>
                <button class="button-rect button-neutral">{"edit username"}</button>
                <button class="button-rect button-text">{"edit username"}</button>

                <input type="checkbox" class="switch" checked=true />
                <input type="checkbox" class="switch" />
                <input type="checkbox" class="switch" disabled=true checked=true />
                <input type="checkbox" class="switch" disabled=true />

                <input type="checkbox" checked=true />
                <input type="checkbox" />
                <input type="checkbox" disabled=true checked=true />
                <input type="checkbox" disabled=true />

                <input type="radio" for="test1" checked=true />
                <input type="radio" for="test1" />
                <input type="radio" for="test1" disabled=true checked=true />
                <input type="radio" for="test1" disabled=true />
                
            </div>
            
            <div class="message-box-area">
                <MessageButton />
                <textarea placeholder="Send a message... " name="message-box" id="message-box" cols="30" rows="2"></textarea>

                <button class="round-button">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M498.1 5.6c10.1 7 15.4 19.1 13.5 31.2l-64 416c-1.5 9.7-7.4 18.2-16 23s-18.9 5.4-28 1.6L284 427.7l-68.5 74.1c-8.9 9.7-22.9 12.9-35.2 8.1S160 493.2 160 480V396.4c0-4 1.5-7.8 4.2-10.7L331.8 202.8c5.8-6.3 5.6-16-.4-22s-15.7-6.4-22-.7L106 360.8 17.7 316.6C7.1 311.3 .3 300.7 0 288.9s5.9-22.8 16.1-28.7l448-256c10.7-6.1 23.9-5.5 34 1.4z"/></svg>
                </button>
            </div>
        </main>

        <div role="user-profile" class="user-profile">
            <img class="left" src="https://link.storjshare.io/raw/jvxikkhiqnksyeatwcn3iigoa3ta/techlgbt/accounts/avatars/109/504/275/977/175/789/original/a3e6266b885c9b43.png" />
            <content>
                <a>{"Username"}</a>
                <text class="status">{"status"}</text>

                <button class="round-button" style="float: right; margin-top: -2.5rem;">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M481.9 166.6c3.2 8.7 .5 18.4-6.4 24.6l-30.9 28.1c-7.7 7.1-11.4 17.5-10.9 27.9c.1 2.9 .2 5.8 .2 8.8s-.1 5.9-.2 8.8c-.5 10.5 3.1 20.9 10.9 27.9l30.9 28.1c6.9 6.2 9.6 15.9 6.4 24.6c-4.4 11.9-9.7 23.3-15.8 34.3l-4.7 8.1c-6.6 11-14 21.4-22.1 31.2c-5.9 7.2-15.7 9.6-24.5 6.8l-39.7-12.6c-10-3.2-20.8-1.1-29.7 4.6c-4.9 3.1-9.9 6.1-15.1 8.7c-9.3 4.8-16.5 13.2-18.8 23.4l-8.9 40.7c-2 9.1-9 16.3-18.2 17.8c-13.8 2.3-28 3.5-42.5 3.5s-28.7-1.2-42.5-3.5c-9.2-1.5-16.2-8.7-18.2-17.8l-8.9-40.7c-2.2-10.2-9.5-18.6-18.8-23.4c-5.2-2.7-10.2-5.6-15.1-8.7c-8.8-5.7-19.7-7.8-29.7-4.6L69.1 425.9c-8.8 2.8-18.6 .3-24.5-6.8c-8.1-9.8-15.5-20.2-22.1-31.2l-4.7-8.1c-6.1-11-11.4-22.4-15.8-34.3c-3.2-8.7-.5-18.4 6.4-24.6l30.9-28.1c7.7-7.1 11.4-17.5 10.9-27.9c-.1-2.9-.2-5.8-.2-8.8s.1-5.9 .2-8.8c.5-10.5-3.1-20.9-10.9-27.9L8.4 191.2c-6.9-6.2-9.6-15.9-6.4-24.6c4.4-11.9 9.7-23.3 15.8-34.3l4.7-8.1c6.6-11 14-21.4 22.1-31.2c5.9-7.2 15.7-9.6 24.5-6.8l39.7 12.6c10 3.2 20.8 1.1 29.7-4.6c4.9-3.1 9.9-6.1 15.1-8.7c9.3-4.8 16.5-13.2 18.8-23.4l8.9-40.7c2-9.1 9-16.3 18.2-17.8C213.3 1.2 227.5 0 242 0s28.7 1.2 42.5 3.5c9.2 1.5 16.2 8.7 18.2 17.8l8.9 40.7c2.2 10.2 9.4 18.6 18.8 23.4c5.2 2.7 10.2 5.6 15.1 8.7c8.8 5.7 19.7 7.7 29.7 4.6l39.7-12.6c8.8-2.8 18.6-.3 24.5 6.8c8.1 9.8 15.5 20.2 22.1 31.2l4.7 8.1c6.1 11 11.4 22.4 15.8 34.3zM242 336a80 80 0 1 0 0-160 80 80 0 1 0 0 160z"/></svg>
                </button>
            </content>
        </div>
    </app>
    }
}

pub fn render_app() {
    yew::Renderer::<App>::new().render();
}