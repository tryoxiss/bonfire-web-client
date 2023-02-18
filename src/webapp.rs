
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

//# Everything prefixed with Fa is from font awesome Free
// They provide this licence disclaimer. 
// This will also be featured in the credits popup.
// Font Awesome Free 6.3.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2023 Fonticons, Inc. 
#[function_component]
fn FaPlusIcon() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M240 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H32c-17.7 0-32 14.3-32 32s14.3 32 32 32H176V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H384c17.7 0 32-14.3 32-32s-14.3-32-32-32H240V80z"></path></svg> }
}

#[function_component]
fn FaSendIcon() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M498.1 5.6c10.1 7 15.4 19.1 13.5 31.2l-64 416c-1.5 9.7-7.4 18.2-16 23s-18.9 5.4-28 1.6L284 427.7l-68.5 74.1c-8.9 9.7-22.9 12.9-35.2 8.1S160 493.2 160 480V396.4c0-4 1.5-7.8 4.2-10.7L331.8 202.8c5.8-6.3 5.6-16-.4-22s-15.7-6.4-22-.7L106 360.8 17.7 316.6C7.1 311.3 .3 300.7 0 288.9s5.9-22.8 16.1-28.7l448-256c10.7-6.1 23.9-5.5 34 1.4z"/></svg> }
}



#[function_component]
fn MessageRoot() -> Html {
    html! {
    <li>
        <div style="background-image: url(https://picsum.photos/id/237/200/300);" class="pfp" />

        <div class="content">
            <header>
                <text class="author">{ "doggo"}</text>
                <time>{ "07 Feb 2023 at 23:09"}</time>
            </header>
            <div class="content">
                <text>{"lorem"}</text>
            </div>
        </div>
    </li>
    }
}

#[function_component]
fn Message() -> Html {
    html! {
    <li>
    <time>{ "23:09"}</time>
        <div class="content">
        <text>{"lorem"}</text>
        </div>
    </li>
    }
}


#[function_component]
fn App() -> Html {
    html! { 
    <app class="ctp-mocha">
        <nav role="groups">
            <ul>
                <li><a href="#1" style="background-image: url(https://picsum.photos/id/237/200/300);"></a></li>
                <li><a href="#2" style="background-image: url(https://picsum.photos/id/217/200/300);"></a></li>
            </ul>
            <hr />
            <ul>
                <li><a href="#1" style="background-image: url(https://picsum.photos/id/237/200/300);"></a></li>
                <li><a href="#2" style="background-image: url(https://picsum.photos/id/217/200/300);"></a></li>
                <li><a href="#3" style="background-image: url(https://picsum.photos/id/237/1920/1080);"></a></li>
                <li><a href="#4" style="background-image: url(https://picsum.photos/id/291/200/300);"></a></li>
                <li><a href="#5" style="background-image: url(https://picsum.photos/id/221/200/300);"></a></li>
                <li><a href="#6" style="background-image: url(https://picsum.photos/id/237/200/300);"></a></li>
                <li><a href="#7" style="background-image: url(https://picsum.photos/id/297/200/300);"></a></li>
                <li><a href="#8" style="background-image: url(https://picsum.photos/id/2197/200/300);"></a></li>
            </ul>
        </nav>
        <nav role="channels"></nav>

        <div role="messages">
            <header>
            </header>

            <ul>
                <MessageRoot />
                <Message />
                <MessageRoot />
                <Message />
                <MessageRoot />
                <Message />
                <Message />
                <Message />
                <MessageRoot />
                <Message />
                <MessageRoot />
                <Message />
                <MessageRoot />
                <Message />
                <Message />
                <Message />
                <MessageRoot />
                <Message />
                <MessageRoot />
                <Message />
                <MessageRoot />
                <Message />
                <Message />
                <Message />
                <MessageRoot />
                <Message />
                <MessageRoot />
                <Message />
                <MessageRoot />
                <Message />
                <Message />
                <Message />
            </ul>

            <div class="message-area">
                <div 
                    class="expandable-textarea"
                    role="textbox"
                    contenteditable="true"
                    placeholder="send a message">
                    {"Your default value"}
                </div>
            </div>

        </div>

        <div role="profile"></div>
    </app>
    }
}

pub fn render_app() {
    yew::Renderer::<App>::new().render();
}