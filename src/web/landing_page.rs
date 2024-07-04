use leptos::*;
use leptos_meta::*;

#[component]
pub fn LandingPageFull() -> impl IntoView {
    view! {
        <Title text="Welcome"/>
        <main
            class="relative h-screen bg-center bg-cover"
            style="background-image: url('images/hero4.webp'); z-index: 1;"
        >
            <div class="absolute inset-0 bg-black opacity-50"></div>
            <div class="flex relative z-10 justify-center items-center h-full text-center">
                <div class="px-4 text-white md:px-10">
                    <h1 class="mb-4 text-4xl font-bold md:text-6xl">
                        "Achieve Your Fitness Goals with Balance and Ease"
                    </h1>

                    <p class="mb-8 text-lg md:text-2xl">
                        "Find your rhythm, track your journey, stay inspired, and reach new heights."
                    </p>

                    <div class="flex justify-center space-x-4">
                        <a
                            href="/signup"
                            class="inline-block py-4 px-8 text-xl font-bold text-white bg-gradient-to-r from-orange-500 to-orange-700 rounded hover:from-orange-600 hover:to-orange-800"
                        >
                            "Get Started"
                        </a>
                    </div>
                </div>
            </div>
        </main>
    }
}

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <Title text="Welcome"/>
        <section
            class="relative bg-center bg-cover h-screen md:h-auto"
            style="background-image: url('images/site/hero4.webp'); z-index: 1;"
        >
            <div class="absolute inset-0 bg-black opacity-50"></div>
            <div class="flex relative z-10 justify-center items-center h-full md:h-96 text-center">
                <div class="px-4 text-white md:px-10">
                    <h1 class="mb-4 text-4xl font-bold md:text-6xl">
                        "Achieve Your Fitness Goals with Balance and Ease"
                    </h1>
                    <p class="mb-8 text-lg md:text-2xl">
                        "Find your rhythm, track your journey, stay inspired, and reach new heights."
                    </p>
                    <div class="flex justify-center space-x-4">
                        <a
                            href="/login"
                            class="inline-block py-4 px-8 text-xl font-bold text-white border border-white rounded hover:bg-white hover:text-black transition-colors"
                        >
                            "Log in"
                        </a>
                        <a
                            href="/signup"
                            class="inline-block py-4 px-8 text-xl font-bold text-white bg-gradient-to-r from-orange-500 to-orange-700 rounded hover:from-orange-600 hover:to-orange-800"
                        >
                            "Get Started"
                        </a>
                    </div>
                </div>
            </div>
        </section>

        // Features Section
        <section class="py-16 bg-gray-100">
            <div class="container mx-auto px-6 md:px-12">
                <div class="mb-12 text-center">
                    <h2 class="text-3xl font-bold md:text-4xl">"Key Features"</h2>
                    <p class="mt-4 text-lg md:text-xl">"Discover the benefits of Trackedfitness"</p>
                </div>
                <div class="flex flex-wrap justify-center">
                    <div class="w-full md:w-1/3 px-4 mb-8">
                        <div class="p-6 bg-white rounded-lg shadow-md text-center">
                            <i class="mb-4 text-4xl text-orange-500 fas fa-utensils"></i>
                            <h3 class="mb-2 text-xl font-semibold">"Meal Planning"</h3>
                            <p class="text-gray-600">"Easily plan and track your meals."</p>
                        </div>
                    </div>
                    <div class="w-full md:w-1/3 px-4 mb-8">
                        <div class="p-6 bg-white rounded-lg shadow-md text-center">
                            <i class="mb-4 text-4xl text-orange-500 fas fa-dumbbell"></i>
                            <h3 class="mb-2 text-xl font-semibold">"Workout Tracking"</h3>
                            <p class="text-gray-600">"Log your workouts and monitor progress."</p>
                        </div>
                    </div>
                    <div class="w-full md:w-1/3 px-4 mb-8">
                        <div class="p-6 bg-white rounded-lg shadow-md text-center">
                            <i class="mb-4 text-4xl text-orange-500 fas fa-chart-line"></i>
                            <h3 class="mb-2 text-xl font-semibold">"Progress Reports"</h3>
                            <p class="text-gray-600">"See detailed reports on your progress."</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        // Testimonials Section
        <section class="py-16 bg-white">
            <div class="container mx-auto px-6 md:px-12">
                <div class="mb-12 text-center">
                    <h2 class="text-3xl font-bold md:text-4xl">"User Testimonials"</h2>
                    <p class="mt-4 text-lg md:text-xl">"Hear what our users have to say"</p>
                </div>
                <div class="flex flex-wrap justify-center">
                    <div class="w-full md:w-1/3 px-4 mb-8">
                        <div class="p-6 bg-gray-100 rounded-lg shadow-md text-center">
                            <p class="text-gray-700">"Trackedfitness changed my life!"</p>
                            <h3 class="mt-4 text-lg font-semibold">"Sarah J."</h3>
                        </div>
                    </div>
                    <div class="w-full md:w-1/3 px-4 mb-8">
                        <div class="p-6 bg-gray-100 rounded-lg shadow-md text-center">
                            <p class="text-gray-700">"Easy to use and very effective."</p>
                            <h3 class="mt-4 text-lg font-semibold">"John D."</h3>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <section class="py-16 bg-gray-100">
            <div class="container mx-auto px-6 md:px-12">
                <div class="mb-12 text-center">
                    <h2 class="text-3xl font-bold md:text-4xl">"How It Works"</h2>
                    <p class="mt-4 text-lg md:text-xl">"Get started in a few simple steps"</p>
                </div>
                <div class="flex flex-wrap justify-center">
                    <div class="w-full md:w-1/4 px-4 mb-8">
                        <div class="p-6 bg-white rounded-lg shadow-md text-center">
                            <i class="mb-4 text-4xl text-orange-500 fas fa-user-plus"></i>
                            <h3 class="mb-2 text-xl font-semibold">"Sign Up"</h3>
                            <p class="text-gray-600">"Create your free account."</p>
                        </div>
                    </div>
                    <div class="w-full md:w-1/4 px-4 mb-8">
                        <div class="p-6 bg-white rounded-lg shadow-md text-center">
                            <i class="mb-4 text-4xl text-orange-500 fas fa-cogs"></i>
                            <h3 class="mb-2 text-xl font-semibold">"Customize"</h3>
                            <p class="text-gray-600">"Set your diet and workout goals."</p>
                        </div>
                    </div>
                    <div class="w-full md:w-1/4 px-4 mb-8">
                        <div class="p-6 bg-white rounded-lg shadow-md text-center">
                            <i class="mb-4 text-4xl text-orange-500 fas fa-clipboard-check"></i>
                            <h3 class="mb-2 text-xl font-semibold">"Track"</h3>
                            <p class="text-gray-600">"Log your meals and workouts daily."</p>
                        </div>
                    </div>
                    <div class="w-full md:w-1/4 px-4 mb-8">
                        <div class="p-6 bg-white rounded-lg shadow-md text-center">
                            <i class="mb-4 text-4xl text-orange-500 fas fa-trophy"></i>
                            <h3 class="mb-2 text-xl font-semibold">"Achieve"</h3>
                            <p class="text-gray-600">
                                "Monitor your progress and reach your goals."
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        // Footer Section
        <footer class="py-8 bg-zinc-800 text-zinc-100">
            <div class="container mx-auto px-6 md:px-12 text-center">
                <p class="text-lg">"© 2024 Trackedfitness. All rights reserved."</p>
                <div class="mt-4">
                    <a href="/privacy-policy" class="text-orange-500 hover:text-orange-700">
                        "Privacy Policy"
                    </a>
                    " | "
                    <a href="/terms-of-service" class="text-orange-500 hover:text-orange-700">
                        "Terms of Service"
                    </a>
                </div>
            </div>
        </footer>
    }
}
