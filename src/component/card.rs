use leptos::*;
use leptos_meta::*;
use leptos_router::*;
#[component]
pub fn FoodCard() -> impl IntoView {
    view! {
        <main class="p-4">
            <div class="max-w-sm mx-auto bg-white rounded-lg shadow-md overflow-hidden">
                <div class="md:flex">
                    <div class="md:flex-shrink-0">
                        <img
                            class="h-32 w-full object-cover md:w-32"
                            src="https://via.placeholder.com/150"
                            alt="Food Image"
                        />
                    </div>
                    <div class="p-4">
                        <div class="uppercase tracking-wide text-sm text-indigo-500 font-semibold">
                            Food Name
                        </div>
                        <p class="block mt-1 text-lg leading-tight font-medium text-black">
                            Grilled Chicken Salad
                        </p>
                        <p class="mt-2 text-gray-500">
                            A healthy grilled chicken salad with mixed greens, tomatoes, cucumbers, and a light vinaigrette dressing.
                        </p>
                        <div class="mt-4">
                            <span class="text-gray-600">Portion Size:</span>
                            <span class="text-gray-800 font-semibold">1 Serving</span>
                        </div>
                        <div class="mt-2">
                            <span class="text-gray-600">Calories:</span>
                            <span class="text-gray-800 font-semibold">350 kcal</span>
                        </div>
                        <div class="mt-4 flex justify-between">
                            <button class="text-indigo-600 hover:text-indigo-900 font-semibold">
                                Edit
                            </button>
                            <button class="text-red-600 hover:text-red-900 font-semibold">
                                Delete
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </main>
    }
}
