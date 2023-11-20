use leptos::*;

#[component]
pub(crate) fn Icon<T: AsRef<str> + Default>(
    icon: T, #[prop(optional)] classes: T
) -> impl IntoView {
    let classes = format!("material-symbols-rounded {}", classes.as_ref());
    let icon = String::from(icon.as_ref());
    view! {
        <i class=classes>
            {icon}
        </i>
    }
}