use super::*;
use web_sys::Element;

#[derive(Properties, PartialEq)]
pub struct PopModalProps {
    #[prop_or_default]
    pub children: Html,

    pub show: bool,
}

pub struct PopModal {
    node_ref: NodeRef,
    modal_host: Option<Element>,
}

impl Component for PopModal {
    type Message = ();
    type Properties = PopModalProps;

    fn create(_: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            modal_host: None,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let el = self.node_ref.cast::<Element>().unwrap();
            self.modal_host = Some(el);
            ctx.link().send_message(());
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let contents = self.modal_host.clone()
            .map_or(
                html! {<></>},
                |n| create_portal(ctx.props().children.clone(), n)
            );

        html! {
            <div ref={self.node_ref.clone()} class="pop" >
                if ctx.props().show {
                    {contents}
                }
            </div>
        }
    }
}
