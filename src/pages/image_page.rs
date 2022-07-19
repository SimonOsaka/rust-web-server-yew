use crate::components::image::{Image, ImageDimension, ImageRatio};
use yew::prelude::*;

#[function_component(ImagePage)]
pub fn image_page() -> Html {
    html! {
        <>
           <div class="block">
               <Image src={"https://bulma.io/images/placeholders/128x128.png".to_string()}
                   dimension={ImageDimension::Image128} />
           </div>

           <div class="block">
               <Image src={"https://bulma.io/images/placeholders/128x128.png".to_string()}
                   dimension={ImageDimension::Image128} round={true} />
           </div>

           <div class="block">
               <Image src={"https://bulma.io/images/placeholders/24x24.png".to_string()}
                   dimension={ImageDimension::Image24} />
           </div>

           <div class="block">
               <Image src={"https://bulma.io/images/placeholders/640x320.png".to_string()}
                   ratio={ImageRatio::Image2by1} />
           </div>

           <div class="block">
               <Image src={"https://bulma.io/images/placeholders/128x128.png".to_string()}
                   dimension={ImageDimension::Image128} extra_img_class={"image_example".to_string()}
                   extra_figure_class={"image_figure_example".to_string()} />
           </div>
       </>
    }
}
