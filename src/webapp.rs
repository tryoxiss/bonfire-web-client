
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
fn FaUserIcon() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M224 256A128 128 0 1 0 224 0a128 128 0 1 0 0 256zm-45.7 48C79.8 304 0 383.8 0 482.3C0 498.7 13.3 512 29.7 512H418.3c16.4 0 29.7-13.3 29.7-29.7C448 383.8 368.2 304 269.7 304H178.3z"/></svg> }
} 
#[function_component]
fn FaSearchIcon() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M416 208c0 45.9-14.9 88.3-40 122.7L502.6 457.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L330.7 376c-34.4 25.2-76.8 40-122.7 40C93.1 416 0 322.9 0 208S93.1 0 208 0S416 93.1 416 208zM208 352a144 144 0 1 0 0-288 144 144 0 1 0 0 288z"/></svg> }
} 
#[function_component]
fn FaPushpinIcon() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M32 32C32 14.3 46.3 0 64 0H320c17.7 0 32 14.3 32 32s-14.3 32-32 32H290.5l11.4 148.2c36.7 19.9 65.7 53.2 79.5 94.7l1 3c3.3 9.8 1.6 20.5-4.4 28.8s-15.7 13.3-26 13.3H32c-10.3 0-19.9-4.9-26-13.3s-7.7-19.1-4.4-28.8l1-3c13.8-41.5 42.8-74.8 79.5-94.7L93.5 64H64C46.3 64 32 49.7 32 32zM160 384h64v96c0 17.7-14.3 32-32 32s-32-14.3-32-32V384z"/></svg> }
} 
#[function_component]
fn FaInvisibleIcon() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512"><path d="M40.1 467.1l-11.2 9c-3.2 2.5-7.1 3.9-11.1 3.9C8 480 0 472 0 462.2V192C0 86 86 0 192 0S384 86 384 192V462.2c0 9.8-8 17.8-17.8 17.8c-4 0-7.9-1.4-11.1-3.9l-11.2-9c-13.4-10.7-32.8-9-44.1 3.9L269.3 506c-3.3 3.8-8.2 6-13.3 6s-9.9-2.2-13.3-6l-26.6-30.5c-12.7-14.6-35.4-14.6-48.2 0L141.3 506c-3.3 3.8-8.2 6-13.3 6s-9.9-2.2-13.3-6L84.2 471c-11.3-12.9-30.7-14.6-44.1-3.9zM160 192a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm96 32a32 32 0 1 0 0-64 32 32 0 1 0 0 64z"/></svg> }
}
#[function_component]
fn FaHomeIcon() -> Html {
    html! { <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><path d="M575.8 255.5c0 18-15 32.1-32 32.1h-32l.7 160.2c0 2.7-.2 5.4-.5 8.1V472c0 22.1-17.9 40-40 40H456c-1.1 0-2.2 0-3.3-.1c-1.4 .1-2.8 .1-4.2 .1H416 392c-22.1 0-40-17.9-40-40V448 384c0-17.7-14.3-32-32-32H256c-17.7 0-32 14.3-32 32v64 24c0 22.1-17.9 40-40 40H160 128.1c-1.5 0-3-.1-4.5-.2c-1.2 .1-2.4 .2-3.6 .2H104c-22.1 0-40-17.9-40-40V360c0-.9 0-1.9 .1-2.8V287.6H32c-18 0-32-14-32-32.1c0-9 3-17 10-24L266.4 8c7-7 15-8 22-8s15 2 21 7L564.8 231.5c8 7 12 15 11 24z"/></svg> }
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

// #[function_component]
// fn RoundButton() -> Html {
//     html! {
//     <button class="round-button">
//         <FaPushpinIcon />
//     </button>
//     }
// }


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

        <main>
            <header class="med-padding">
                <h1><span class="muted">{"@"}</span><span>{"doggo"}</span><span class="muted">{"@instance.tld"}</span></h1>
                <vr/>
                <span style="margin-left: auto;"><button><FaPushpinIcon /></button></span>
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
                <button class="round-button"><FaPlusIcon /></button>

                <div 
                    class="expandable-textarea"
                    role="textbox"
                    contenteditable="true"
                    placeholder="send a message">
                </div>

                <button class="round-button"><FaSendIcon /></button>
            </div>

        </main>

        <div role="profile"></div>
    </app>
    }
}

pub fn render_app() {
    yew::Renderer::<App>::new().render();
}