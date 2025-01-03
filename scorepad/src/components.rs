use leptos::{ev::KeyboardEvent, html, prelude::*};

#[component]
pub fn IntegerInput(setter: WriteSignal<i64>, class: &'static str) -> impl IntoView {
    let number_value = RwSignal::new(0);

    let input_element: NodeRef<html::Input> = NodeRef::new();
    let update_value = move |_| {
        // Get the input element
        let input = input_element.get().unwrap();
        // Get the value from the input element
        let binding = input.value();
        let value = binding.trim();

        // Set to 0 if empty or just a minus sign
        if value.is_empty() || value == "-" {
            number_value.set(0);
            setter.set(0);
            return;
        }

        // Only update if the value contains at least one numeric character
        if value.chars().any(|c| c.is_numeric()) {
            if let Ok(parsed) = value.parse::<i64>() {
                number_value.set(parsed);
                setter.set(parsed);
            }
        }
    };

    let prevent_non_numeric = move |ev: KeyboardEvent| {
        let key = ev.key();
        // Allow navigation keys, backspace, delete, etc.
        if key.len() == 1 {
            let c = key.chars().next().unwrap();
            let input = input_element.get().unwrap();
            let current_value = input.value();
            let selection_start = input.selection_start().unwrap().unwrap_or(0) as usize;

            // Prevent non-numeric characters except minus
            if !c.is_numeric() {
                // Only allow minus if:
                // 1. The character is minus
                // 2. The cursor is at the start
                // 3. There isn't already a minus sign
                if !(c == '-' && selection_start == 0 && !current_value.starts_with('-')) {
                    ev.prevent_default();
                }
            }
        }
    };

    let clear_zero_on_focus = move |_| {
        let input = input_element.get().unwrap();
        if input.value() == "0" {
            input.set_value("");
            number_value.set(0);
        }
    };

    let handle_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            input_element.get().unwrap().blur().unwrap();
        }
    };

    view! {
        <input
            type="text"
            placeholder="0"
            inputmode="numeric"
            autocomplete="off"
            on:input:target=update_value
            on:keypress=prevent_non_numeric
            on:focus=clear_zero_on_focus
            on:keydown=handle_keydown
            value=number_value
            node_ref=input_element
            class=class
        />
    }
}

#[component]
pub fn PlayerNameInput(setter: WriteSignal<String>, class: &'static str) -> impl IntoView {
    let name_value = RwSignal::new(String::from("Player"));
    let input_element: NodeRef<html::Input> = NodeRef::new();

    let update_value = move |_| {
        let input = input_element.get().unwrap();
        let value = input.value();

        // Don't allow empty names, reset to "Player" if empty
        if value.trim().is_empty() {
            name_value.set(String::from("Player"));
            setter.set(String::from("Player"));
            return;
        }

        name_value.set(value.clone());
        setter.set(value);
    };

    let clear_default_on_focus = move |_| {
        let input = input_element.get().unwrap();
        if input.value().starts_with("Player") {
            input.set_value("");
            name_value.set(String::new());
        }
    };

    let handle_keydown = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            input_element.get().unwrap().blur().unwrap();
        }
    };

    view! {
        <input
            type="text"
            placeholder="Player"
            autocomplete="off"
            on:input:target=update_value
            on:focus=clear_default_on_focus
            on:keydown=handle_keydown
            value=name_value
            node_ref=input_element
            class=class
        />
    }
}
