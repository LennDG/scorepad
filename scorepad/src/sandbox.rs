use crate::components::IntegerInput;
use leptos::prelude::*;

#[component]
pub fn Sandbox() -> impl IntoView {
    let (value1, set_value1) = signal(0);
    let (value2, set_value2) = signal(0);

    view! {
        <div class="min-h-screen flex flex-col items-center justify-center p-4">
            <h1 class="text-4xl font-bold text-white mb-8">"Sandbox"</h1>
            <IntegerInput
                setter=set_value1
                class="w-full max-w-lg px-4 py-2 bg-gray-700 text-white border border-gray-600 rounded-lg focus:outline-none focus:border-blue-500 transition-colors"
            />
            <p class="mt-4 text-lg text-gray-300">{value1}</p>
            <IntegerInput
                setter=set_value2
                class="w-full max-w-lg px-4 py-2 bg-gray-700 text-white border border-gray-600 rounded-lg focus:outline-none focus:border-blue-500 transition-colors"
            />
            <p class="mt-4 text-lg text-gray-300">{value2}</p>
        </div>
    }
}
