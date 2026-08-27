use patternfly_yew::prelude::*;
use popper_rs_sys::ModifierArguments;
use popper_rs::prelude::{State as PopperState, *};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use yew_hooks::prelude::*;
use yew::{html::ChildrenRenderer, prelude::*};

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownProperties {
    #[prop_or_default]
    pub children: ChildrenRenderer<MenuChildVariant>,
}

/// Dropdown menu component
///
/// ## Properties
///
/// Define by [`DropdownProperties`].
///
/// ## Contexts
///
/// Provides the following contexts to its children:
///
/// * [`CloseMenuContext`]
#[component]
pub fn Dropdown(props: &DropdownProperties) -> Html {
    let expanded = use_state_eq(|| false);
    let ontoggle = use_callback(expanded.clone(), move |_, expanded| {
        expanded.set(!**expanded)
    });

    // this defines what is "inside"
    let inside_ref = use_node_ref();
    let target_ref = use_node_ref();
    let menu_ref = use_node_ref();

    {
        // click away unless it was on the inside, which covers the toggle as well as
        // the menu content. As long as we use inline/absolute popover modes and not use
        // a portal.
        let expanded = expanded.clone();
        let menu_ref = menu_ref.clone();
        use_click_away(inside_ref.clone(), move |event: Event| {
            if let Some(menu) = menu_ref.cast::<web_sys::HtmlElement>()
                && !menu.contains(event.target().unwrap().dyn_ref::<web_sys::Node>())
            {
                expanded.set(false)
            }
        });
    }

    let state = use_state_eq(PopperState::default);
    let onstatechange = use_callback(state.clone(), |new_state, state| state.set(new_state));

    let onclose = use_callback(expanded.clone(), |(), expanded| expanded.set(false));
    let context = CloseMenuContext::new(onclose);

    let style = use_state_eq(|| state.styles.popper.clone());
    
    let width_mods = use_memo((), |()| {
        let style = style.clone();
        let inside_ref = inside_ref.clone();
        let state = state.clone();        

        Closure::wrap(
            Box::new(
                move |args: ModifierArguments| {
                    let s = args.instance().state();
                    if let Some(_elem) = inside_ref.cast::<web_sys::HtmlElement>() {
                        let new_style = state
                            .styles
                            .popper
                            .extend_with("z-index", "9999")
                            .extend_with("opacity", "1")
                            .extend_with("transition", "opacity cubic-bezier(0.54, 1.5, 0.38, 1.11)");

                        style.set(new_style)
                    }
                    state.set(s.into());
                }
            ) as Box<dyn Fn(ModifierArguments)>
        )
    });

    let modifiers= {
        Vec::from([Modifier::Custom { 
            name: "widthMods".into(), 
            phase: Some("beforeWrite".into()), 
            enabled: Some(true), 
            r#fn: Some(ModifierFn(width_mods)), 
        }])
    };    

    html!(
        <>
            <div style="display: inline" ref={inside_ref.clone()}>
                <MenuToggle
                    r#ref={target_ref.clone()}
                    expanded={*expanded}
                    {ontoggle}
                />
                <PortalPopper
                    visible={*expanded}
                    target={target_ref.clone()}
                    content={menu_ref.clone()}
                    modifiers={modifiers}
                    {onstatechange}
                >
                    <ContextProvider<CloseMenuContext> {context}>
                        <Menu r#ref={menu_ref} style={&(*style)}>{ props.children.clone() }</Menu>
                    </ContextProvider<CloseMenuContext>>
                </PortalPopper>
            </div>
        </>
    )
}
