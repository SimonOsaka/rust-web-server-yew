use yew::{function_component, html, Classes, Properties};

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct ImageProps {
    #[prop_or("".to_string())]
    pub src: String,
    #[prop_or(false)]
    pub round: bool,
    #[prop_or(ImageDimension::Default)]
    pub dimension: ImageDimension,
    #[prop_or(ImageRatio::Default)]
    pub ratio: ImageRatio,
    #[prop_or("".to_string())]
    pub extra_figure_class: String,
    #[prop_or("".to_string())]
    pub extra_img_class: String,
}
#[function_component(Image)]
pub fn image(props: &ImageProps) -> Html {
    let ImageProps {
        dimension,
        src,
        round,
        ratio,
        extra_figure_class,
        extra_img_class,
    } = props.clone();

    let mut figure_cls = Classes::new();
    figure_cls.push(extra_figure_class);
    figure_cls.push("image");

    let dimension = match dimension {
        ImageDimension::Image128 => "is-128x128",
        ImageDimension::Image96 => "is-96x96",
        ImageDimension::Image64 => "is-64x64",
        ImageDimension::Image48 => "is-48x48",
        ImageDimension::Image32 => "is-32x32",
        ImageDimension::Image24 => "is-24x24",
        ImageDimension::Image16 => "is-16x16",
        ImageDimension::Default => "",
    };
    figure_cls.push(dimension);

    let ratio = match ratio {
        ImageRatio::ImageSquare => "is-square",
        ImageRatio::Image1by1 => "is-1by1",
        ImageRatio::Image5by4 => "is-5by4",
        ImageRatio::Image4by3 => "is-4by3",
        ImageRatio::Image3by2 => "is-3by2",
        ImageRatio::Image5by3 => "is-5by3",
        ImageRatio::Image16by9 => "is-16by9",
        ImageRatio::Image2by1 => "is-2by1",
        ImageRatio::Image3by1 => "is-3by1",
        ImageRatio::Image4by5 => "is-4by5",
        ImageRatio::Image3by4 => "is-3by4",
        ImageRatio::Image2by3 => "is-2by3",
        ImageRatio::Image3by5 => "is-3by5",
        ImageRatio::Image9by16 => "is-9by16",
        ImageRatio::Image1by2 => "is-1by2",
        ImageRatio::Image1by3 => "is-1by3",
        ImageRatio::Default => "",
    };
    figure_cls.push(ratio);

    let mut img_cls = Classes::new();
    img_cls.push(extra_img_class);

    if round {
        img_cls.push("is-rounded");
    }

    html! {
        <figure class={figure_cls}>
            <img class={img_cls} {src}/>
        </figure>
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[allow(dead_code)]
pub enum ImageDimension {
    Image128,
    Image96,
    Image64,
    Image48,
    Image32,
    Image24,
    Image16,
    Default,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[allow(dead_code)]
pub enum ImageRatio {
    ImageSquare,
    Image1by1,
    Image5by4,
    Image4by3,
    Image3by2,
    Image5by3,
    Image16by9,
    Image2by1,
    Image3by1,
    Image4by5,
    Image3by4,
    Image2by3,
    Image3by5,
    Image9by16,
    Image1by2,
    Image1by3,
    Default,
}
