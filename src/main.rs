#![recursion_limit = "1024"]

use browser_panic_hook::{CustomBody, IntoPanicHook};
use wasm_bindgen::prelude::*;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[hook]
pub fn use_open<IN>(url: impl Into<String>, target: impl Into<String>) -> Callback<IN, ()>
where
    IN: 'static,
{
    use_callback((url.into(), target.into()), |_, (url, target)| {
        let _ = gloo_utils::window().open_with_url_and_target(url, target);
    })
}


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PageProps {
    pub children: Children,
}

#[function_component(AppPage)]
fn page(props: &PageProps) -> Html {
    let callback_github = use_open(
        "https://github.com/patternfly-yew/patternfly-yew-quickstart",
        "_blank",
    );

    let backdropper = use_backdrop();

    let onabout = use_callback((), move |_, ()| {
        if let Some(backdropper) = &backdropper {
            ()
        }
    });

    // // track dark mode state
    // let darkmode = use_state_eq(|| {
    //     gloo_utils::window()
    //         .match_media("(prefers-color-scheme: dark)")
    //         .ok()
    //         .flatten()
    //         .map(|m| m.matches())
    //         .unwrap_or_default()
    // });
    let darkmode = use_state(|| true);

    // apply dark mode
    // use_effect_with(*darkmode, |state| match state {
    //     true => gloo_utils::document_element().set_class_name("pf-v6-theme-dark"),
    //     false => gloo_utils::document_element().set_class_name(""),
    // });

    // toggle dark mode
    let onthemeswitch = use_callback(darkmode.setter(), |state, setter| setter.set(state));

    let tools = html!(
        <Toolbar full_height=true>
            <ToolbarContent>
                <ToolbarGroup
                    modifiers={ToolbarElementModifier::End.all()}
                    variant={GroupVariant::IconButton}
                >
                    <ToolbarItem>
                        <patternfly_yew::prelude::Switch
                            checked={*darkmode}
                            onchange={onthemeswitch}
                            label="Dark Theme"
                        />
                    </ToolbarItem>
                    <ToolbarItem>
                        <Button
                            variant={ButtonVariant::Plain}
                            onclick={callback_github}
                        />
                    </ToolbarItem>
                    <ToolbarItem>
                        <Dropdown
                            position={Position::Right}
                            icon={Icon::QuestionCircle}
                            variant={MenuToggleVariant::Plain}
                        >
                            <MenuAction onclick={onabout}>{ "About" }</MenuAction>
                        </Dropdown>
                    </ToolbarItem>
                </ToolbarGroup>
            </ToolbarContent>
        </Toolbar>
    );

    html! (<Page {tools} full_height=true>{ for props.children.iter() }</Page>)
}

// this does not reproduce the problem :/
#[function_component(Reproduction)]
fn dropdown() -> Html {
    let toaster = use_toaster().unwrap();
    let show_toast = use_callback(toaster, |text, toaster| toaster.toast(text));

    html!{
        <Dropdown text={html!{"Foo"}}>
            <MenuAction onclick={show_toast.reform(|_|"Clicked Foo")}>{"Foo"}</MenuAction>
            <ListDivider/>
            <MenuAction icon={Icon::Cubes} onclick={show_toast.reform(|_|"Clicked Bar")}>{"Bar"}</MenuAction>
            <ListDivider/>
            <MenuLink href="https://patternfly.org" target="_blank">{"PatternFly "} {Icon::ExternalLinkAlt}</MenuLink>
        </Dropdown>        
    }
}

#[function_component(Application)]
fn app() -> Html {
    html! {
        <BackdropViewer>
            <ToastViewer>
                <Reproduction />
            </ToastViewer>
        </BackdropViewer>
    }
}

fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::set_custom_panic_hook(
        CustomBody(Box::new(|details| {
            format!(
                r#"
<div class="pf-v6-l-bullseye">
  <div class="pf-v6-l-bullseye__item">
    <div class="pf-v6-c-alert pf-m-danger" aria-label="Application panicked">
      <div class="pf-v6-c-alert__icon">
        <i class="fas fa-fw fa-exclamation-circle" aria-hidden="true"></i>
      </div>
      <p class="pf-v6-c-alert__title">
        <span class="pf-v6-screen-reader">Panick alert:</span>
        Application panicked
      </p>
      <div class="pf-v6-c-alert__description">
        <p>The application failed critically and cannot recover.</p>
        <p>Reason: <pre>{message}</pre></p>
      </div>
    </div>
  </div>
</div>
"#,
                message = details.message()
            )
        }))
        .into_panic_hook(),
    );    
    yew::Renderer::<Application>::new().render();
    Ok(())
}
