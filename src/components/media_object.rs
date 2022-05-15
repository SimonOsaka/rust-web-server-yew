use yew::{function_component, html};

#[function_component(MediaObjectArticle)]
pub fn mo_article() -> Html {
    html! {
        <article class="media">
            <figure class="media-left">
                <p class="image is-64x64">
                <img src="https://bulma.io/images/placeholders/128x128.png"/>
                </p>
            </figure>
            <div class="media-content">
                <div class="content">
                <p>
                    <strong>{"John Smith"}</strong> <small>{"@johnsmith"}</small> <small>{"31m"}</small>
                    <br/>
                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin ornare magna eros, eu pellentesque tortor vestibulum ut. Maecenas non massa sem. Etiam finibus odio quis feugiat facilisis."}
                </p>
                </div>
                <nav class="level is-mobile">
                <div class="level-left">
                    <a class="level-item">
                    <span class="icon is-small"><i class="fas fa-reply"></i></span>
                    </a>
                    <a class="level-item">
                    <span class="icon is-small"><i class="fas fa-retweet"></i></span>
                    </a>
                    <a class="level-item">
                    <span class="icon is-small"><i class="fas fa-heart"></i></span>
                    </a>
                </div>
                </nav>
            </div>
            <div class="media-right">
                <button class="delete"></button>
            </div>
        </article>
    }
}

#[function_component(MediaObjectPost)]
pub fn mo_post() -> Html {
    html! {
        <article class="media">
            <figure class="media-left">
                <p class="image is-64x64">
                <img src="https://bulma.io/images/placeholders/128x128.png"/>
                </p>
            </figure>
            <div class="media-content">
                <div class="field">
                <p class="control">
                    <textarea class="textarea" placeholder="Add a comment..."></textarea>
                </p>
                </div>
                <nav class="level">
                <div class="level-left">
                    <div class="level-item">
                    <a class="button is-info">{"Submit"}</a>
                    </div>
                </div>
                <div class="level-right">
                    <div class="level-item">
                    <label class="checkbox">
                        <input type="checkbox"/> {"Press enter to submit"}
                    </label>
                    </div>
                </div>
                </nav>
            </div>
        </article>
    }
}

#[function_component(MediaObjectNestPost)]
pub fn mo_nest_post() -> Html {
    html! {
        <>
        <article class="media">
        <figure class="media-left">
            <p class="image is-64x64">
            <img src="https://bulma.io/images/placeholders/128x128.png"/>
            </p>
        </figure>
        <div class="media-content">
            <div class="content">
            <p>
                <strong>{"Barbara Middleton"}</strong>
                <br/>
                {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis porta eros lacus, nec ultricies elit blandit non. Suspendisse pellentesque mauris sit amet dolor blandit rutrum. Nunc in tempus turpis."}
                <br/>
                <small><a>{"Like"}</a>{" · "}<a>{"Reply"}</a>{" · "}{"3 hrs"}</small>
            </p>
            </div>

            <article class="media">
            <figure class="media-left">
                <p class="image is-48x48">
                <img src="https://bulma.io/images/placeholders/96x96.png"/>
                </p>
            </figure>
            <div class="media-content">
                <div class="content">
                <p>
                    <strong>{"Sean Brown"}</strong>
                    <br/>
                    {"Donec sollicitudin urna eget eros malesuada sagittis. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Aliquam blandit nisl a nulla sagittis, a lobortis leo feugiat."}
                    <br/>
                    <small><a>{"Like"}</a>{" · "}<a>{"Reply"}</a>{" · "}{"2 hrs"}</small>
                </p>
                </div>

                <article class="media">
                {"Vivamus quis semper metus, non tincidunt dolor. Vivamus in mi eu lorem cursus ullamcorper sit amet nec massa."}
                </article>

                <article class="media">
                {"Morbi vitae diam et purus tincidunt porttitor vel vitae augue. Praesent malesuada metus sed pharetra euismod. Cras tellus odio, tincidunt iaculis diam non, porta aliquet tortor."}
                </article>
            </div>
            </article>

            <article class="media">
            <figure class="media-left">
                <p class="image is-48x48">
                <img src="https://bulma.io/images/placeholders/96x96.png"/>
                </p>
            </figure>
            <div class="media-content">
                <div class="content">
                <p>
                    <strong>{"Kayli Eunice "}</strong>
                    <br/>
                    {"Sed convallis scelerisque mauris, non pulvinar nunc mattis vel. Maecenas varius felis sit amet magna vestibulum euismod malesuada cursus libero. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Phasellus lacinia non nisl id feugiat."}
                    <br/>
                    <small><a>{"Like"}</a>{" · "}<a>{"Reply"}</a>{" · "}{"2 hrs"}</small>
                </p>
                </div>
            </div>
            </article>
        </div>
        </article>

        <article class="media">
        <figure class="media-left">
            <p class="image is-64x64">
            <img src="https://bulma.io/images/placeholders/128x128.png"/>
            </p>
        </figure>
        <div class="media-content">
            <div class="field">
            <p class="control">
                <textarea class="textarea" placeholder="Add a comment..."></textarea>
            </p>
            </div>
            <div class="field">
            <p class="control">
                <button class="button">{"Post comment"}</button>
            </p>
            </div>
        </div>
        </article>
        </>
    }
}
